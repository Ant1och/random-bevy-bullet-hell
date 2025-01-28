use crate::{
    ground_detection::GroundDetection,
    player::{direction, DashState, DashTimer, LookingDirection, Player},
    shared::move_toward_f32,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

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
    mut player: Query<
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
    let Ok((looking_direction, mut velocity, mut dash_state, ground_detection)) =
        player.get_single_mut()
    else {
        return;
    };

    if input.just_pressed(KeyCode::ShiftLeft) {
        dash(&mut velocity, looking_direction.0, direction(&input));

        dash_timer.0.reset();
        dash_state.is_dashing = true;
    }

    if ground_detection.just_grounded {
        //&& !input.just_pressed(KeyCode::ShiftLeft) {
        dash_state.is_dashing = false;
    }

    if dash_state.is_dashing && velocity.linvel.y > 0. {
        velocity.linvel.y = move_toward_f32(velocity.linvel.y, 0., PLAYER_DECELLERATION * delta)
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

fn player_jump(
    mut player: Query<(&mut Velocity, &GroundDetection), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, ground_detection)) = player.get_single_mut() else {
        return;
    };

    if input.pressed(KeyCode::Space) && ground_detection.grounded {
        velocity.linvel.y = PLAYER_JUMP_STRENGTH;
    }
}

fn player_horizontal_movement(
    mut player: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let delta = time.delta().as_secs_f64();
    let direction = direction(&input);
    let new_vel_x = direction.x * PLAYER_SPEED;
    let Ok(mut velocity) = player.get_single_mut() else {
        return;
    };

    velocity.linvel.x = match direction.x {
        0. => move_toward_f32(velocity.linvel.x, new_vel_x, PLAYER_DECELLERATION * delta),
        _ => move_toward_f32(velocity.linvel.x, new_vel_x, PLAYER_ACELLERATION * delta),
    };
}

fn player_looking_direction(
    mut player: Query<&mut LookingDirection, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let direction = direction(&input);

    let Ok(mut looking_direction) = player.get_single_mut() else {
        return;
    };

    if direction.x != 0. {
        looking_direction.0 = direction.x;
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
            ),
        );
    }
}
