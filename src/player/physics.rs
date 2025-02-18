use crate::{
    ground_detection::GroundDetection,
    input::prelude::*,
    physics::looking_direction::LookDir,
    player::{DashTimer, LookingDirection, Player},
    shared::move_toward_f32,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::player::config::physics::*;

fn dash(velocity: &mut Mut<'_, Velocity>, looking_direction: &LookDir, direction: Vec2) {
    let dash_direction_x = match direction.y {
        0. => looking_direction.into(),
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
    mut player: Query<(&LookingDirection, &mut Velocity), With<Player>>,
    input: Query<(&Direction, &KeysPressed), With<CustomInput>>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let Ok((looking_direction, mut velocity)) = player.get_single_mut() else {
        return;
    };

    let Ok((direction, keys)) = input.get_single() else {
        return;
    };

    if keys.just_pressed(KeyType::Dash) {
        dash(&mut velocity, &looking_direction.0, direction.0);

        dash_timer.0.reset();
    }
}

fn player_gravity(
    mut player: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let delta = time.delta_secs_f64();
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };

    if dash_timer.0.tick(time.delta()).finished() {
        velocity.linvel.y -= (PLAYER_GRAVITY * delta) as f32;
    }
}

fn player_decelleration(mut player: Query<&mut Velocity, With<Player>>, time: Res<Time>) {
    let delta = time.delta().as_secs_f64();
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };
    let decelleration_x =
        PLAYER_DECELLERATION * (velocity.linvel.x.abs().powf(0.5) + 1.) as f64 * delta;
    let decelleration_y =
        PLAYER_DECELLERATION * (velocity.linvel.y.abs().powf(0.5) + 1.) as f64 * delta;
    let decelleration_y_down = PLAYER_DECELLERATION / PLAYER_GRAVITY * delta;

    velocity.linvel.y = match velocity.linvel.y > 0. {
        true => move_toward_f32(velocity.linvel.y, 0., decelleration_y),
        false => move_toward_f32(velocity.linvel.y, 0., decelleration_y_down),
    };

    velocity.linvel.x = move_toward_f32(velocity.linvel.x, 0., decelleration_x);
}

fn player_jump(
    mut player: Query<(&mut Velocity, &GroundDetection), With<Player>>,
    input: Query<&KeysPressed, With<CustomInput>>,
) {
    let Ok((mut velocity, ground_detection)) = player.get_single_mut() else {
        return;
    };
    let Ok(keys) = input.get_single() else {
        return;
    };

    if keys.pressed(KeyType::Jump) && ground_detection.grounded {
        velocity.linvel.y = PLAYER_JUMP_STRENGTH;
    }
}

fn player_horizontal_movement(
    mut player: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    input: Query<&Direction, With<CustomInput>>,
) {
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };
    let Ok(Direction(direction)) = input.get_single() else {
        return;
    };

    let delta = time.delta().as_secs_f64();
    let new_vel_x = direction.x * PLAYER_SPEED;
    let acceleration = PLAYER_ACELLERATION * delta;

    if direction.x != 0. && new_vel_x.abs() > velocity.linvel.x.abs() {
        velocity.linvel.x = move_toward_f32(velocity.linvel.x, new_vel_x, acceleration);
    }
}

fn player_looking_direction(
    mut player: Query<&mut LookingDirection, With<Player>>,
    input: Query<&Direction, With<CustomInput>>,
) {
    let Ok(Direction(direction)) = input.get_single() else {
        return;
    };

    let Ok(mut looking_direction) = player.get_single_mut() else {
        return;
    };

    if direction.x != 0. {
        looking_direction.0 = LookDir::from(direction.x);
    }
}

fn player_autostep(
    ground_detection: Query<&GroundDetection, With<Player>>,
    mut controller: Query<&mut KinematicCharacterController>,
) {
    let Ok(mut controller) = controller.get_single_mut() else {
        return;
    };
    let Ok(ground_detection) = ground_detection.get_single() else {
        return;
    };

    if ground_detection.grounded {
        controller.translation = Some(Vec2::new(0., PLAYER_AUTOSTEP_AMOUNT));
    }
}

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DashTimer(Timer::from_seconds(
            PLAYER_DASH_BUFFER,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (
                player_jump,
                (player_dash, player_gravity).chain(),
                player_horizontal_movement,
                player_looking_direction,
                player_autostep,
                player_decelleration,
            ),
        );
    }
}
