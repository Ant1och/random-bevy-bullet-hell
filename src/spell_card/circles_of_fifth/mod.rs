use crate::{
    bullet_pattern::circle::{CirclePatternBundle, CirclePatternParams},
    player::Player,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

#[derive(Component, Default)]
pub struct CirclesOfFifth;

#[derive(Clone, Default)]
pub struct Circle {
    pub params: CirclePatternParams,
    pub speed: f32,
    pub accel: f32,
}

#[derive(Component, Default)]
pub struct Circles {
    pub list: Vec<Circle>,
    i: usize,
}

impl Circles {
    pub fn new(list: Vec<Circle>) -> Self {
        Circles { list, i: 0 }
    }
}

#[derive(Component, Default)]
pub struct ShootTimer(Timer);

impl ShootTimer {
    fn from_duration(duration: Duration) -> Self {
        ShootTimer(Timer::new(duration, TimerMode::Repeating))
    }
}

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
    pub circles: Circles,
    pub shoot_timer: ShootTimer,
}

impl CirclesOfFifthBundle {
    pub fn from_params(circles: Circles, frequency: Duration) -> Self {
        CirclesOfFifthBundle {
            name: Name::from("Circles Of Fifth"),
            circles,
            shoot_timer: ShootTimer::from_duration(frequency),
            ..default()
        }
    }
}

// pub fn circle_of_fifth_setup(
//     mut spell_card: Query<(&mut ShootTimer, &CirclesOfFifthParams), Added<CirclesOfFifth>>,
// ) {
//     for (mut shoot_timer, params) in &mut spell_card {
//         shoot_timer.0.set_mode(TimerMode::Repeating);
//         shoot_timer.0.set_duration(params.frequency);
//     }
// }

pub fn circles_of_fifth(
    player: Query<&Transform, With<Player>>,
    mut spell_card: Query<
        (&mut ShootTimer, &GlobalTransform, &mut Circles, Entity),
        With<CirclesOfFifth>,
    >,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let Ok(player_position) = player.get_single() else {
        return;
    };
    let player_position = player_position.translation.truncate();

    for (mut shoot_timer, transform, mut circles, spell_card) in &mut spell_card {
        if shoot_timer.0.tick(time.delta()).just_finished() {
            circles.i += 1;

            if circles.i >= circles.list.len() {
                circles.i = 0;
            }

            let params = circles.list[circles.i].params.clone();

            let mut dir_to_player = player_position - transform.translation().truncate();
            dir_to_player = dir_to_player / dir_to_player.length();

            let circle = cmd
                .spawn(CirclePatternBundle::from_params(
                    params,
                    dir_to_player * circles.list[circles.i].speed,
                    dir_to_player * circles.list[circles.i].accel,
                ))
                .id();

            cmd.entity(spell_card).add_child(circle);
        }
    }
}

// pub fn cof_setup_circles_after_construction(
//     player: Query<&Transform, With<Player>>,
//     children: Query<&Children>,
//     mut spell_cards: Query<(&GlobalTransform, Entity), With<CirclesOfFifth>>,
//     mut pattern_query: Query<
//         (&CirclePatternConstruction, &mut Velocity, &mut Acceleration),
//         With<CirclePattern>,
//     >,
// ) {
//     let Ok(player_position) = player.get_single() else {
//         return;
//     };
//     let player_position = player_position.translation.truncate();

//     for (transform, spell_card) in &mut spell_cards {
//         for circle in children.children(spell_card) {
//             let Ok((construction, mut velocity, mut accel)) = pattern_query.get_mut(*circle) else {
//                 return;
//             };
//             let spell_position = transform.translation().truncate();

//             if !construction.finished {
//                 velocity.linvel = player_position - spell_position;
//                 accel.0 = (player_position - spell_position) / 1000.;
//             }
//         }
//     }
// }
