use super::controls_enabled;
use crate::{
    ground_detection::GroundDetection,
    input::Action,
    physics::looking_direction::LookDir,
    player::{physics_disabled, physics_enabled, DashTimer, LookingDirection, Player},
    shared::{move_toward_f32, move_toward_vec2},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::player::config::physics::*;

use super::stats::PlayerStats;

#[derive(Event, Default)]
pub struct Dash;

fn dash(looking_direction: &LookDir, direction: &Vec2) -> Vec2 {
    let dash_direction = match direction {
        &Vec2::ZERO => looking_direction.into(),
        _ => direction.normalize_or_zero(),
    };
    dash_direction * PLAYER_DASH_STRENGTH
}

fn player_dash(
    player: Single<
        (
            &ActionState<Action>,
            &LookingDirection,
            &PlayerStats,
            &mut Velocity,
        ),
        With<Player>,
    >,
    mut event_writer: EventWriter<Dash>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let (input, looking_direction, PlayerStats { stamina, .. }, mut velocity) = player.into_inner();

    let direction = input.axis_pair(&Action::Direction);

    if *stamina > 0 && input.just_pressed(&Action::Dash) {
        velocity.linvel = dash(&looking_direction.0, &direction);
        dash_timer.0.reset();
        event_writer.write(Dash);
    }
}

fn player_gravity(
    mut player: Single<&mut Velocity, With<Player>>,
    time: Res<Time>,
    mut dash_timer: ResMut<DashTimer>,
) {
    let delta = time.delta_secs_f64();
    let velocity = &mut player.linvel;

    if dash_timer.0.tick(time.delta()).finished() {
        velocity.y -= (PLAYER_GRAVITY * delta) as f32;
    }
}

fn player_decelleration(mut player: Single<&mut Velocity, With<Player>>, time: Res<Time>) {
    let delta = time.delta().as_secs_f64();
    let velocity = &mut player.linvel;
    let decelleration_x = PLAYER_DECELLERATION * (velocity.x.abs().sqrt() + 1.) as f64 * delta;
    let decelleration_y = PLAYER_DECELLERATION * (velocity.y.abs().sqrt() + 1.) as f64 * delta;
    let decelleration_y_down = PLAYER_DECELLERATION / PLAYER_GRAVITY * delta;

    velocity.y = match velocity.y > 0. {
        true => move_toward_f32(velocity.y, 0., decelleration_y),
        false => move_toward_f32(velocity.y, 0., decelleration_y_down),
    };

    velocity.x = move_toward_f32(velocity.x, 0., decelleration_x);
}

fn player_jump(
    player: Single<(&ActionState<Action>, &mut Velocity, &GroundDetection), With<Player>>,
) {
    let (input, mut velocity, GroundDetection { grounded, .. }) = player.into_inner();

    if input.pressed(&Action::Jump) && *grounded {
        velocity.linvel.y = PLAYER_JUMP_STRENGTH;
    }
}

fn player_horizontal_movement(
    player: Single<(&ActionState<Action>, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (input, mut velocity) = player.into_inner();
    let velocity = &mut velocity.linvel;
    let direction = input.axis_pair(&Action::Direction);

    let delta = time.delta().as_secs_f64();
    let new_vel_x = direction.x * PLAYER_SPEED;
    let acceleration = PLAYER_ACELLERATION * delta;

    if direction.x != 0. && new_vel_x.abs() > velocity.x.abs() {
        velocity.x = move_toward_f32(velocity.x, new_vel_x, acceleration);
    }
}

fn player_looking_direction(
    player: Single<(&ActionState<Action>, &mut LookingDirection), With<Player>>,
) {
    let (input, mut looking_direction) = player.into_inner();
    let direction = input.axis_pair(&Action::Direction);

    if direction.x != 0. {
        looking_direction.0 = LookDir::from(direction.x);
    }
}

fn player_autostep(
    ground_detection: Single<&GroundDetection, With<Player>>,
    controller: Single<&mut KinematicCharacterController>,
) {
    let mut controller = controller.into_inner();
    let grounded = ground_detection.grounded;
    if grounded {
        controller.translation = Some(Vec2::new(0., PLAYER_AUTOSTEP_AMOUNT));
    }
}

fn stop(player: Single<&mut Velocity, With<Player>>) {
    let mut velocity = player.into_inner();
    velocity.linvel = move_toward_vec2(velocity.linvel, Vec2::ZERO, PLAYER_DECELLERATION / 5.);
}

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Dash>()
            .insert_resource(DashTimer(Timer::from_seconds(
                PLAYER_DASH_BUFFER,
                TimerMode::Once,
            )))
            .add_systems(
                Update,
                (
                    player_jump.run_if(controls_enabled),
                    (
                        player_dash.run_if(controls_enabled),
                        player_gravity.run_if(physics_enabled),
                    )
                        .chain(),
                    player_horizontal_movement.run_if(controls_enabled),
                    player_looking_direction.run_if(controls_enabled),
                    player_autostep.run_if(physics_enabled),
                    player_decelleration.run_if(physics_enabled),
                    stop.run_if(physics_disabled),
                ),
            );
    }
}

#[cfg(test)]
#[test]
fn dash_direction() {
    const PRECCISION: f32 = 1000000.;
    const DIRECTIONS: &[Vec2] = &[
        Vec2::new(1., 1.),
        Vec2::new(-1., -1.),
        Vec2::new(0., 1.),
        Vec2::new(0., -1.),
        Vec2::new(1., 0.),
        Vec2::new(-1., 0.),
        Vec2::new(-1., 1.),
        Vec2::new(1., -1.),
    ];

    let dir = Vec2::new(0., 0.);
    assert_eq!(dash(&LookDir::Left, &dir).normalize_or_zero(), Vec2::NEG_X);
    assert_eq!(dash(&LookDir::Right, &dir).normalize_or_zero(), Vec2::X);

    for dir in DIRECTIONS {
        assert_eq!(
            (dash(&LookDir::Right, dir).normalize_or_zero() * PRECCISION).round(),
            (dir.normalize_or_zero() * PRECCISION).round()
        );
        assert_eq!(
            (dash(&LookDir::Left, dir).normalize_or_zero() * PRECCISION).round(),
            (dir.normalize_or_zero() * PRECCISION).round()
        );
    }
}
