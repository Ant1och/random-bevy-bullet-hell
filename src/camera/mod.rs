use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;
use crate::shared::move_toward_exp_vec2;

mod config;
use crate::camera::config::*;

#[derive(Default, Component)]
pub struct Target(Vec2);

#[derive(Default, Bundle)]
pub struct CameraBundle {
    camera: Camera2d,
    target: Target,
    msaa: Msaa,
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(CameraBundle {
        // Texture bleeding fix
        msaa: Msaa::Off,
        ..default()
    });
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Target,
        ),
        (With<Camera2d>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut target) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            if level_selection.is_match(&LevelIndices::default(), level) {
                orthographic_projection.viewport_origin = Vec2::ZERO;
                let height = (level.px_hei as f32 / 9.).round() * 9.;
                let width = height * ASPECT_RATIO;
                orthographic_projection.scaling_mode =
                    bevy::render::camera::ScalingMode::Fixed { width, height };

                target.0 = Vec2::new(
                    (player_translation.x - level_transform.translation.x - width / 2.)
                        .clamp(0., (level.px_wid as f32 - width).max(0.)),
                    (player_translation.y - level_transform.translation.y - height / 2.)
                        .clamp(0., (level.px_hei as f32 - height).max(0.)),
                );

                target.0.x += level_transform.translation.x;
                target.0.y += level_transform.translation.y;
            }
        }
    }
}

fn camera_move_to_target(
    mut camera: Query<(&mut Transform, &Target), With<Camera2d>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs_f64();
    let Ok((mut transform, target)) = camera.get_single_mut() else {
        return;
    };

    transform.translation = move_toward_exp_vec2(
        transform.translation.truncate(),
        target.0,
        CAMERA_PANNING,
        CAMERA_MIN_SPEED,
        delta * CAMERA_BASE_SPEED,
    )
    .extend(0.);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (camera_fit_inside_current_level, camera_move_to_target),
        );
    }
}
