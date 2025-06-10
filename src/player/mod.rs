use crate::physics::looking_direction::LookDir;
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseAnimation;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub mod config;
use config::PLAYER_DEFAULT_LOOKING_DIRECTION;

mod animation;
use animation::PlayerAnimationPlugin;

mod physics;
use physics::PlayerPhysicsPlugin;

pub mod stats;
use stats::{PlayerStats, PlayerStatsPlugin};

#[derive(Debug, Default, Component)]
pub struct Player;

#[derive(Resource)]
pub struct DashTimer(Timer);

#[derive(Component)]
pub struct LookingDirection(LookDir);

impl Default for LookingDirection {
    fn default() -> Self {
        LookingDirection(PLAYER_DEFAULT_LOOKING_DIRECTION)
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet(no_grid)]
    pub sprite: Sprite,
    pub animation: AseAnimation,
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins(PlayerPhysicsPlugin)
            .add_plugins(PlayerAnimationPlugin)
            .add_plugins(PlayerStatsPlugin);
    }
}
