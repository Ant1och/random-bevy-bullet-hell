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

#[derive(PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct LookingDirection(f32);

#[derive(Resource)]
pub struct DashTimer(Timer);

#[derive(Component, Default)]
pub struct PlayerStats {
    health: i64,
}

#[derive(Resource)]
pub struct InvincibilityTimer(Timer);

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
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

pub fn direction(input: &Res<ButtonInput<KeyCode>>) -> Vec2 {
    let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
    let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };
    let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

    Vec2::new(right - left, up - down)
}

fn set_player_default_stats(mut player: Query<&mut PlayerStats, Added<Player>>) {
    let Ok(mut stats) = player.get_single_mut() else {
        return;
    };

    stats.health = 10;
}

pub fn player_invincibility_timer(
    mut invicibility_timer: ResMut<InvincibilityTimer>,
    time: Res<Time>,
) {
    invicibility_timer.0.tick(time.delta());
}

pub fn player_damage(
    mut player: Query<&mut PlayerStats, With<Player>>,
    mut invicibility_timer: ResMut<InvincibilityTimer>,
) {
    let Ok(mut stats) = player.get_single_mut() else {
        return;
    };

    if invicibility_timer.0.finished() {
        stats.health -= 1;
        invicibility_timer.0.reset();
    }

    println!("{}", stats.health);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .insert_resource(InvincibilityTimer(Timer::from_seconds(
                0.45,
                TimerMode::Once,
            )))
            .add_systems(Update, set_player_default_stats)
            .add_systems(Update, player_invincibility_timer)
            .add_plugins(PlayerPhysicsPlugin)
            .add_plugins(PlayerAnimationPlugin);
    }
}
