use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    bullet_pattern::{construction::ConstructionType, PatternParams},
    enemy::spawner::turret::{TurretAmmo, TurretAmmoList, TurretBundle},
    physics::movement::MovementType,
};

use super::SpellCardTimer;

#[derive(Component, Default)]
pub struct CirclesOfFifth;

#[derive(Component, Default)]
struct Params {
    frequency: Duration,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct CirclesOfFifthBundle {
    pub name: Name,
    pub entity: CirclesOfFifth,
    pub timer: SpellCardTimer,
    params: Params,
    pub transform: Transform,
    // #[worldly]
    // pub worldy: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

impl CirclesOfFifthBundle {
    pub fn new(frequency: Duration, length: Duration) -> Self {
        Self {
            name: Name::new("Circles Of Fifth"),
            timer: SpellCardTimer::new(length),
            params: Params { frequency },
            ..default()
        }
    }
}

fn circle_of_fifth_setup(
    spell_cards: Query<(Entity, &Params), Added<CirclesOfFifth>>,
    mut cmd: Commands,
) {
    let ammo_list = TurretAmmoList::new(
        [
            vec![
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 24,
                        bullet_movement: MovementType::Circle {
                            speed: 40.,
                            accel: 40.,
                        },
                        construction_frequency: Duration::from_secs_f64(0.01),
                    },
                    construction: ConstructionType::Circle,
                    speed: 200.,
                    accel: 0.01,
                };
                3
            ],
            vec![
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 48,
                        bullet_movement: MovementType::Circle {
                            speed: 60.,
                            accel: 300.,
                        },
                        construction_frequency: Duration::from_secs_f64(0.008),
                    },
                    construction: ConstructionType::RegularPolygon(6),
                    speed: 200.,
                    accel: 0.01,
                },
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 96,
                        bullet_movement: MovementType::Circle {
                            speed: 30.,
                            accel: 300.,
                        },
                        construction_frequency: Duration::from_secs_f64(0.008),
                    },
                    construction: ConstructionType::RegularPolygon(4),
                    speed: 160.,
                    accel: 0.01,
                },
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 128,
                        bullet_movement: MovementType::Circle {
                            speed: 30.,
                            accel: 300.,
                        },
                        construction_frequency: Duration::from_secs_f64(0.007),
                    },
                    construction: ConstructionType::Shuriken(3),
                    speed: 160.,
                    accel: 0.01,
                },
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 128,
                        bullet_movement: MovementType::Circle {
                            speed: 60.,
                            accel: 300.,
                        },
                        construction_frequency: Duration::from_secs_f64(0.007),
                    },
                    construction: ConstructionType::Shuriken(4),
                    speed: 160.,
                    accel: 0.01,
                },
            ],
            vec![
                TurretAmmo {
                    params: PatternParams {
                        scale: 40.,
                        bullet_amount: 24,
                        bullet_movement: MovementType::Circle {
                            speed: 25.,
                            accel: 0.01,
                        },
                        construction_frequency: Duration::from_secs_f64(0.01),
                    },
                    construction: ConstructionType::Circle,
                    speed: 0.,
                    accel: 0.,
                };
                1
            ],
        ]
        .concat(),
    );

    for (card, params) in &spell_cards {
        cmd.spawn(TurretBundle::from_params(
            ammo_list.clone(),
            params.frequency,
            Duration::ZERO,
        ))
        .insert(ChildOf(card));
    }
}

pub struct CirclesOfFifthPlugin;

impl Plugin for CirclesOfFifthPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CirclesOfFifthBundle>("Circles Of Fifth")
            .add_systems(Update, circle_of_fifth_setup);
    }
}
