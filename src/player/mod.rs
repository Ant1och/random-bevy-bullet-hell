use crate::{
    colliders::ColliderBundle, ground_detection::GroundDetection, shared::move_toward_f32,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;

mod config;
use config::*;

const ANIMATION_SPEED: f32 = 1.15;

// use sprite::
// mod sprite;

pub struct Animations;

impl Animations {
    const WALK_R: &str = "walk_right";
    // There is no left yet...
    const WALK_L: &str = "walk_right";
}

#[derive(PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct JumpState {
    is_jumping: bool,
}

#[derive(Component, Default)]
pub struct LookingDirection(f32);

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    // #[sprite("reimu_r.png")]
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    pub jump_state: JumpState,
    pub looking_direction: LookingDirection,
    #[worldly]
    pub worldly: Worldly,
    pub ground_detection: GroundDetection,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Resource)]
struct DashTimer(Timer);

fn set_player_sprite(
    mut player: Query<&mut AseSpriteAnimation, Added<Player>>,
    server: Res<AssetServer>,
) {
    let Ok(mut animation) = player.get_single_mut() else {
        return;
    };
    animation.aseprite = server.load("reimu.aseprite");
    animation.animation = Animation::default().with_tag("walk_right")
}

fn player_animation(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut AseSpriteAnimation, With<Player>>,
) {
    let Ok(mut asesprite) = player.get_single_mut() else {
        // error!("Cannot find Player in App!");
        return;
    };

    let direction = direction(&input);

    let new_animation = match direction.x {
        1. => Animations::WALK_R,
        -1. => Animations::WALK_L,
        _ => {
            return;
        }
    };

    asesprite.animation = Animation::tag(new_animation).with_speed(ANIMATION_SPEED);
}

fn direction(input: &Res<ButtonInput<KeyCode>>) -> Vec2 {
    let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
    let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };
    let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
    let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

    Vec2::new(right - left, up - down)
}

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
    mut query: Query<(&LookingDirection, &mut Velocity, &mut JumpState), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut dash_timer: ResMut<DashTimer>,
) {
    for (looking_direction, mut velocity, mut jump_state) in &mut query {
        if input.just_pressed(KeyCode::ShiftLeft) {
            let direction = direction(&input);
            dash(&mut velocity, looking_direction.0, direction);
            dash_timer.0.reset();

            jump_state.is_jumping = false;
        }
    }
}

fn player_gravity(
    mut query: Query<(&mut Velocity, &JumpState), With<Player>>,
    time: Res<Time>,
    mut dash_timer: ResMut<DashTimer>,
) {
    dash_timer.0.tick(time.delta());

    for (mut velocity, jump_state) in &mut query {
        let delta = time.delta().as_secs_f64();

        if dash_timer.0.finished() {
            let gravity = (PLAYER_GRAVITY * delta) as f32;
            velocity.linvel.y -= gravity;
        }

        if !jump_state.is_jumping && velocity.linvel.y > 0. {
            velocity.linvel.y = move_toward_f32(velocity.linvel.y, 0., PLAYER_DECELLERATION * delta)
        }
    }
}

fn player_jump(
    mut query: Query<(&mut Velocity, &GroundDetection, &mut JumpState), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, ground_detection, mut jump_state) in &mut query {
        if ground_detection.on_ground {
            jump_state.is_jumping = false;
        }
        if input.pressed(KeyCode::Space) {
            jump_state.is_jumping = true;
        }
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .insert_resource(DashTimer(Timer::from_seconds(
                PLAYER_DASH_BUFFER,
                TimerMode::Once,
            )))
            .add_systems(Update, set_player_sprite)
            .add_systems(
                Update,
                (
                    player_jump,
                    player_gravity,
                    player_dash,
                    player_horizontal_movement,
                    player_animation,
                ),
            );
    }
}
