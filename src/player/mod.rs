use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub mod config;

mod animation;
use animation::PlayerAnimationPlugin;

mod physics;
use physics::PlayerPhysicsPlugin;

pub mod stats;
use stats::{PlayerStats, PlayerStatsPlugin};

#[derive(PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct LookingDirection(f32);

#[derive(Resource)]
pub struct DashTimer(Timer);

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet(no_grid)]
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    pub stats: PlayerStats,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub looking_direction: LookingDirection,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    pub character_controller: KinematicCharacterController,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn position(player: Query<&Transform, With<Player>>) -> Option<Vec2> {
    let Ok(transform) = player.get_single() else {
        return None;
    };

    Some(transform.translation.truncate())
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins(PlayerPhysicsPlugin)
            .add_plugins(PlayerAnimationPlugin)
            .add_plugins(PlayerStatsPlugin);
    }
}
