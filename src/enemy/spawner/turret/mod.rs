use bevy::prelude::*;
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
    pub entity: Turret,
    // #[worldly]
    // pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub ammo: TurretAmmoList,
    pub transform: Transform,
    pub shoot_timer: ShootTimer,
}

impl TurretBundle {
    pub fn from_params(ammo: TurretAmmoList, delay: Duration, phase: Duration) -> Self {
        TurretBundle {
            name: Name::from("Turret"),
            ammo,
            shoot_timer: ShootTimer::from_duration(delay, phase),
            ..default()
        }
    }
}

fn turret_shoot(
    player: Single<&Transform, With<Player>>,
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
    let player_position = player.translation.truncate();

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

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<TurretBundle>("Turret")
            .add_systems(Update, turret_shoot);
    }
}
