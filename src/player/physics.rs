use crate::{
    ground_detection::GroundDetection,
    player::{direction, DashState, DashTimer, LookingDirection, Player},
    shared::move_toward_f32,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::player::config::physics::*;

fn dash(velocity: &mut Mut<'_, Velocity>, looking_direction: f32, direction: Vec2) {
    let looking_direction = match looking_direction {
        1. => 1.,
        -1. => -1.,
        _ => {
            error!("Invalid looking direction!");
            return;
        }
    };

    // velocity.linvel.x = 1000. * looking_direction;
    let dash_direction_x = match direction.y {
        0. => looking_direction,
        _ => direction.x,
    };
    let dash_direction_y = direction.y;

    let dash_vel = Vec2 {
        x: dash_direction_x / dash_direction_x.hypot(dash_direction_y),
        y: dash_direction_y / dash_direction_y.hypot(dash_direction_x),
    };

    velocity.linvel = dash_vel * PLAYER_DASH_STRENGTH;
}

fn player_dash(
    mut query: Query<
        (
            &LookingDirection,
            &mut Velocity,
            &mut DashState,
            &GroundDetection,
        ),
        With<Player>,
    >,
    input: Res<ButtonInput<KeyCode>>,
    mut dash_timer: ResMut<DashTimer>,
    time: Res<Time>,
) {
    let delta = time.delta_secs_f64();

    for (looking_direction, mut velocity, mut dash_state, ground_detection) in &mut query {
        if input.just_pressed(KeyCode::ShiftLeft) {
            dash(&mut velocity, looking_direction.0, direction(&input));

            dash_timer.0.reset();
            dash_state.is_dashing = true;
        }

        if ground_detection.on_ground && !input.pressed(KeyCode::ShiftLeft) {
            dash_state.is_dashing = false;
        }

        if dash_state.is_dashing && velocity.linvel.y > 0. {
            velocity.linvel.y = move_toward_f32(velocity.linvel.y, 0., PLAYER_DECELLERATION * delta)
        }
    }
}

fn player_gravity(
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    mut dash_timer: ResMut<DashTimer>,
) {
    dash_timer.0.tick(time.delta());

    for mut velocity in &mut query {
        let delta = time.delta().as_secs_f64();

        if dash_timer.0.finished() {
            let gravity = (PLAYER_GRAVITY * delta) as f32;
            velocity.linvel.y -= gravity;
        }
    }
}

fn player_jump(
    mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, ground_detection) in &mut query {
        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = PLAYER_JUMP_STRENGTH;
        }
        // println!("{}", jump_state.is_jumping);
    }
}

fn player_horizontal_movement(
    mut query: Query<(&mut Velocity, &mut LookingDirection), With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let delta = time.delta().as_secs_f64();

    let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

    let new_vel_x = (right - left) * PLAYER_SPEED;

    for (mut velocity, mut looking_direction) in &mut query {
        velocity.linvel.x = match direction(&input).x {
            0. => move_toward_f32(velocity.linvel.x, new_vel_x, PLAYER_DECELLERATION * delta),
            _ => move_toward_f32(velocity.linvel.x, new_vel_x, PLAYER_ACELLERATION * delta),
        };

        if right - left != 0. {
            looking_direction.0 = right - left;
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DashTimer(Timer::from_seconds(
            PLAYER_DASH_BUFFER,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (
                player_jump,
                player_gravity,
                player_dash,
                player_horizontal_movement,
            ),
        );
    }
}
