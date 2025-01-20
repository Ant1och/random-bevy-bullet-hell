use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;
use bevy_ecs_ldtk::prelude::*;

pub mod config;

mod animation;
use animation::AnimationPlugin;

mod physics;
use physics::PhysicsPlugin;

#[derive(PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct DashState {
    is_dashing: bool,
}

#[derive(Component, Default)]
pub struct LookingDirection(f32);

#[derive(Resource)]
pub struct DashTimer(Timer);

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub dash_state: DashState,
    pub looking_direction: LookingDirection,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn direction(input: &Res<ButtonInput<KeyCode>>) -> Vec2 {
    let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
    let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };
    let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

    Vec2::new(right - left, up - down)
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_plugins(PhysicsPlugin)
            .add_plugins(AnimationPlugin);
    }
}
