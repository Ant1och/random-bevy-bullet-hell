use crate::input::Action;
use crate::physics::looking_direction::LookDir;
use crate::player::state::{PlayerState, StatePlugin};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub mod config;
use config::PLAYER_DEFAULT_LOOKING_DIRECTION;
pub mod animation;
use animation::PlayerAnimationPlugin;
mod physics;
use leafwing_input_manager::prelude::InputMap;
use physics::PlayerPhysicsPlugin;
pub mod stats;
use stats::{PlayerStats, PlayerStatsPlugin};
pub mod attack;
use attack::AttackPlugin;
pub mod state;

#[derive(Debug, Default, Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct ControlsEnabled(pub bool);

#[derive(Resource, Default)]
pub struct PhysicsEnabled(pub bool);

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
    // #[sprite_sheet(no_grid)]
    // pub sprite: Sprite,
    // pub animation: Aseprite,
    pub stats: PlayerStats,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub looking_direction: LookingDirection,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    pub character_controller: KinematicCharacterController,
    pub input_map: InputMap<Action>,
    pub state: PlayerState,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn controls_enabled(
    controls_enabled: Res<ControlsEnabled>,
    physics_enabled: Res<PhysicsEnabled>,
) -> bool {
    controls_enabled.0 && physics_enabled.0
}

pub fn physics_enabled(enabled: Res<PhysicsEnabled>) -> bool {
    enabled.0
}

pub fn physics_disabled(enabled: Res<PhysicsEnabled>) -> bool {
    !enabled.0
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .insert_resource(ControlsEnabled(false))
            .insert_resource(PhysicsEnabled(false))
            .add_plugins(PlayerPhysicsPlugin)
            .add_plugins(PlayerAnimationPlugin)
            .add_plugins(PlayerStatsPlugin)
            .add_plugins(AttackPlugin)
            .add_plugins(StatePlugin);
    }
}
