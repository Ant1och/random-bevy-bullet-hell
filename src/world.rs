use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config::WORLD_GRAVITY;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    commands.spawn(Camera2d);

    rapier_config.single_mut().gravity = Vec2::new(0., WORLD_GRAVITY);

    let ldtk_handle = asset_server.load("level.ldtk").into();
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
