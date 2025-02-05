use crate::physics::shared::AccelerationScale;
use crate::{
    bullet_pattern::circle::{CirclePatternBundle, CirclePatternConstruction},
    player::Player,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

#[derive(Component, Default)]
pub struct CirclesOfFifth;

#[derive(Component, Default)]
pub struct CirclesOfFifthParams {
    pub frequency: Duration,
}

#[derive(Component, Default)]
pub struct ShootTimer(Timer);

// impl ShootTimer {
//     fn from_duration(duration: Duration) -> Self {
//         ShootTimer(Timer::new(duration, TimerMode::Repeating))
//     }
// }

#[derive(Bundle, Default, LdtkEntity)]
pub struct CirclesOfFifthBundle {
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    pub entity: CirclesOfFifth,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub parameters: CirclesOfFifthParams,
    pub shoot_timer: ShootTimer,
}

impl CirclesOfFifthBundle {
    pub fn from_params(params: CirclesOfFifthParams) -> Self {
        CirclesOfFifthBundle {
            parameters: params,
            ..default()
        }
    }
}

pub fn circle_of_fifth_setup(
    mut spell_card: Query<(&mut ShootTimer, &CirclesOfFifthParams), Added<CirclesOfFifth>>,
) {
    for (mut shoot_timer, params) in &mut spell_card {
        shoot_timer.0.set_mode(TimerMode::Repeating);
        shoot_timer.0.set_duration(params.frequency);
    }
}

pub fn circles_of_fifth(
    player: Query<&Transform, With<Player>>,
    mut spell_card: Query<(&mut ShootTimer, Entity), With<CirclesOfFifth>>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let Ok(player_position) = player.get_single() else {
        return;
    };
    let player_position = player_position.translation.truncate();

    for (mut shoot_timer, spell_card) in &mut spell_card {
        if shoot_timer.0.tick(time.delta()).just_finished() {
            let circle_bundle = CirclePatternBundle::from_params(CirclePatternConstruction {
                radius: 100.,
                bullets_max_amount: 12,
                speed: Duration::from_secs_f64(0.1),
                rotation_speed: 30.,
                bullets_acceleration_scale: AccelerationScale(10.),
                ..default()
            });
            let circle = cmd.spawn(circle_bundle).id();

            cmd.entity(spell_card).add_child(circle);
        }
    }
}
