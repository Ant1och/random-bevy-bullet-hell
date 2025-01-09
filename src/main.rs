use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod colliders;
mod config;
mod ground_detection;
mod player;
mod shared;
mod walls;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0),
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(world::WorldPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Update, camera::camera_fit_inside_current_level)
        .run();
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2d);

//     commands.spawn(LdtkWorldBundle {
//         ldtk_handle: asset_server.load("level.ldtk").into(),
//         ..Default::default()
//     });
// }

// #[derive(Default, Component)]
// struct ComponentA;

// #[derive(Default, Component)]
// struct ComponentB;

// #[derive(Default, Bundle, LdtkEntity)]
// pub struct MyBundle {
//     a: ComponentA,
//     b: ComponentB,
//     #[sprite_sheet]
//     sprite_sheet: Sprite,
// }
