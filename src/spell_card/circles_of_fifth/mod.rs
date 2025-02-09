use crate::{
    bullet_pattern::circle::{
        CirclePattern, CirclePatternBundle, CirclePatternConstruction, CirclePatternParams,
    },
    physics::{shared::MovementType, Acceleration},
    player::Player,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;
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
    pub name: Name,
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
            name: Name::from("Circles Of Fifth"),
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
    mut spell_card: Query<(&mut ShootTimer, &GlobalTransform, Entity), With<CirclesOfFifth>>,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let Ok(player_position) = player.get_single() else {
        return;
    };
    let player_position = player_position.translation.truncate();

    for (mut shoot_timer, transform, spell_card) in &mut spell_card {
        if shoot_timer.0.tick(time.delta()).just_finished() {
            let spell_position = transform.translation().truncate();
            println!("{:?}", spell_position);
            let circle_bundle = CirclePatternBundle::from_params(
                CirclePatternParams {
                    movement_type: MovementType::Circle {
                        speed: 24.,
                        accel: 190.,
                    },
                    radius: 45.,
                    bullets_max_amount: 10,
                    construction_frequency: Duration::from_secs_f64(0.1),
                },
                Vec2::ZERO,
                player_position - spell_position,
            );
            let circle = cmd.spawn(circle_bundle).id();

            cmd.entity(spell_card).add_child(circle);
        }
    }
}

pub fn cof_setup_circles_after_construction(
    player: Query<&Transform, With<Player>>,
    children: Query<&Children>,
    mut spell_cards: Query<(&GlobalTransform, Entity), With<CirclesOfFifth>>,
    mut pattern_query: Query<
        (&CirclePatternConstruction, &mut Velocity, &mut Acceleration),
        With<CirclePattern>,
    >,
) {
    let Ok(player_position) = player.get_single() else {
        return;
    };
    let player_position = player_position.translation.truncate();

    for (transform, spell_card) in &mut spell_cards {
        for circle in children.children(spell_card) {
            let Ok((construction, mut velocity, mut accel)) = pattern_query.get_mut(*circle) else {
                return;
            };
            let spell_position = transform.translation().truncate();

            if !construction.finished {
                velocity.linvel = player_position - spell_position;
                accel.0 = (player_position - spell_position) / 1000.;
            }
        }
    }
}
