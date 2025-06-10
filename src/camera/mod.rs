use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::IsOutOfBounds;
use crate::player::Player;
use crate::shared::{move_toward_exp_f32, move_toward_quad_f32};

mod config;
use crate::camera::config::*;
use crate::world::update_level_selection;
const CAMERA_VERTICAL_STEP: f32 = 200.;

#[derive(Default, Component)]
pub struct Target(Vec2);

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera2d,
        Name::new("Camera"),
        // Texture bleeding fix
        Msaa::Off,
        Target(Vec2::ZERO),
        IsOutOfBounds::default(),
        Projection::Orthographic(OrthographicProjection {
            viewport_origin: Vec2::new(0.5, 0.5),
            near: 0.0,
            far: 1200.,
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0,
            area: Rect::EMPTY,
        }),
    ));
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut Projection, &mut Target), (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<Projection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.single()
    {
        let player_translation = *player_translation;

        let Ok((projection, mut target)) = camera_query.single_mut() else {
            return;
        };

        if let Projection::Orthographic(ref mut orthographic_projection) = *projection.into_inner()
        {
            for (level_transform, level_iid) in &level_query {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single().unwrap())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .get_raw_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                if level_selection.is_match(&LevelIndices::default(), level) {
                    orthographic_projection.viewport_origin = Vec2::ZERO;

                    let level_ratio = level.px_wid as f32 / level.px_hei as f32;

                    let mut height = (level.px_hei as f32 / 9.).round() * 9.;
                    let mut width = (level.px_wid as f32 / 16.).round() * 16.;

                    if level_ratio > ASPECT_RATIO {
                        width = height * ASPECT_RATIO;
                    } else {
                        height = width / ASPECT_RATIO;
                    }
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    orthographic_projection.viewport_origin = Vec2::new(0.5, 0.5);
                    let discrete_player_translation_y =
                        (player_translation.y / CAMERA_VERTICAL_STEP).round()
                            * CAMERA_VERTICAL_STEP;

                    target.0 = Vec2::new(
                        (player_translation.x - level_transform.translation.x).clamp(
                            width / 2.,
                            (level.px_wid as f32 - width / 2.).max(width / 2.),
                        ),
                        (discrete_player_translation_y - level_transform.translation.y).clamp(
                            height / 2.,
                            (level.px_hei as f32 - height / 2.).max(height / 2.),
                        ),
                    );

                    target.0.x += level_transform.translation.x;
                    target.0.y += level_transform.translation.y;
                }
            }
        }
    }
}

fn camera_move_to_target(
    mut camera: Query<(&mut Transform, &Target, &IsOutOfBounds), With<Camera2d>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs_f64();
    let Ok((mut transform, target, out_of_bounds)) = camera.single_mut() else {
        return;
    };

    if out_of_bounds.0 {
        transform.translation = target.0.extend(0.);
        return;
    }

    transform.translation.x = move_toward_exp_f32(
        transform.translation.x,
        target.0.x,
        CAMERA_PANNING,
        CAMERA_MIN_SPEED,
        delta * CAMERA_BASE_SPEED,
    );

    transform.translation.y = move_toward_quad_f32(
        transform.translation.y,
        target.0.y,
        0.011,
        2.25,
        delta * 80.5,
    );
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_camera).after(update_level_selection))
            .add_systems(
                Update,
                (camera_fit_inside_current_level, camera_move_to_target),
            );
    }
}

// #[test]
// fn camera_auto_scaling() {
//     let mut app = App::new();
//     app.add_plugins(DefaultPlugins).add_systems(Startup, setup);

//     let camera_id = app.world_mut().spawn(CameraBundle::default()).id();
//     app.add_systems(Update, camera_fit_inside_current_level);

//     let ldtk_project_handle = app
//         .world_mut()
//         .query::<&LdtkProjectHandle>()
//         .iter(app.world())
//         .next()
//         .unwrap();

//     let ldtk_project = app
//         .world()
//         .get_resource::<Assets<LdtkProject>>()
//         .unwrap()
//         .get(ldtk_project_handle)
//         .expect("Project should be loaded if level has spawned");

//     for level_iid in app.world().query::<&LevelIid>().iter(app.world()) {
//         let level = ldtk_project
//             .get_raw_level_by_iid(&level_iid.to_string())
//             .expect("Spawned level should exist in LDtk project");

//         assert!(
//             app.world()
//                 .get::<OrthographicProjection>(camera_id)
//                 .unwrap()
//                 .area
//                 .width() as i32
//                 <= level.px_wid,
//         );
//         assert!(
//             app.world()
//                 .get::<OrthographicProjection>(camera_id)
//                 .unwrap()
//                 .area
//                 .height() as i32
//                 <= level.px_hei,
//         );
//     }
// }
