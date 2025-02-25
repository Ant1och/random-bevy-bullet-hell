use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::time::Duration;

use super::super::ShootTimer;
use crate::{
    bullet_pattern::{construction::ConstructionType, PatternBundle, PatternParams},
    physics::movement::MovementType,
    player::Player,
};

#[derive(Component, Default)]
pub struct Turret;

#[derive(Component, Default, Clone)]
pub struct TurretAmmo {
    pub params: PatternParams,
    pub construction: ConstructionType,
    pub speed: f32,
    pub accel: f32,
}

#[derive(Component, Default, Clone)]
pub struct TurretAmmoList {
    pub list: Vec<TurretAmmo>,
    i: usize,
}

impl TurretAmmoList {
    pub fn new(list: Vec<TurretAmmo>) -> Self {
        TurretAmmoList { list, i: 0 }
    }
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct TurretBundle {
    pub name: Name,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    pub entity: Turret,
    // #[worldly]
    // pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub ammo: TurretAmmoList,
    pub shoot_timer: ShootTimer,
}

impl TurretBundle {
    pub fn from_params(ammo: TurretAmmoList, frequency: Duration) -> Self {
        TurretBundle {
            name: Name::from("Turret"),
            ammo,
            shoot_timer: ShootTimer::from_duration(frequency),
            ..default()
        }
    }
}

// pub fn circle_of_fifth_setup(
//     mut spell_card: Query<(&mut ShootTimer, &TurretParams), Added<Turret>>,
// ) {
//     for (mut shoot_timer, params) in &mut spell_card {
//         shoot_timer.0.set_mode(TimerMode::Repeating);
//         shoot_timer.0.set_duration(params.frequency);
//     }
// }

fn turret_shoot(
    player: Query<&Transform, With<Player>>,
    mut turrets: Query<
        (
            &mut ShootTimer,
            &GlobalTransform,
            &mut TurretAmmoList,
            Entity,
        ),
        With<Turret>,
    >,
    mut cmd: Commands,
    time: Res<Time>,
) {
    let Ok(player_position) = player.get_single() else {
        return;
    };
    let player_position = player_position.translation.truncate();

    for (mut shoot_timer, transform, mut ammo, turret) in &mut turrets {
        if shoot_timer.0.tick(time.delta()).just_finished() {
            ammo.i += 1;

            if ammo.i >= ammo.list.len() {
                ammo.i = 0;
            }

            let params = ammo.list[ammo.i].params.clone();
            let construction = ammo.list[ammo.i].construction.clone();
            let speed = ammo.list[ammo.i].speed;
            let accel = ammo.list[ammo.i].accel;

            let mut dir_to_player = player_position - transform.translation().truncate();
            dir_to_player = dir_to_player / dir_to_player.length();

            let pattern = cmd
                .spawn(PatternBundle::new(
                    params,
                    construction,
                    MovementType::Linear {
                        velocity: dir_to_player * speed,
                        accel,
                    },
                ))
                .id();

            cmd.entity(turret).add_child(pattern);
        }
    }
}

// pub fn cof_setup_TurretAmmoList_after_construction(
//     player: Query<&Transform, With<Player>>,
//     children: Query<&Children>,
//     mut spell_cards: Query<(&GlobalTransform, Entity), With<Turret>>,
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

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TurretBundle>("Turret")
            .add_systems(Update, turret_shoot);
    }
}
