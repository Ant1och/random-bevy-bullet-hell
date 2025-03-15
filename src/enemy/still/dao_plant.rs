use std::time::Duration;

use crate::{
    bullet_pattern::{construction::ConstructionType, PatternParams},
    colliders::SensorBundle,
    enemy::spawner::turret::{TurretAmmo, TurretAmmoList, TurretBundle},
    physics::movement::MovementType,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct DaoPlant;

#[derive(Bundle, Default, LdtkEntity)]
pub struct DaoPlantBundle {
    pub entity: DaoPlant,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

fn setup(plants: Query<Entity, Added<DaoPlant>>, mut cmd: Commands) {
    for plant in &plants {
        let ammo = TurretAmmoList::new(vec![TurretAmmo {
            params: PatternParams {
                scale: 0.,
                bullet_amount: 1,
                bullet_movement: MovementType::Still,
                construction_frequency: Duration::from_secs_f64(0.),
            },
            construction: ConstructionType::Circle,
            speed: 300.,
            accel: 0.01,
        }]);

        cmd.spawn(TurretBundle::from_params(ammo, Duration::from_secs_f64(2.)))
            .set_parent(plant);
    }
}

fn animation(
    mut plants: Query<&mut AseSpriteAnimation, Added<DaoPlant>>,
    server: Res<AssetServer>,
) {
    for mut animation in &mut plants {
        animation.aseprite = server.load("dao_plant.aseprite");
        animation.animation = Animation::tag("idle");
    }
}

pub struct DaoPlantPlugin;

impl Plugin for DaoPlantPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<DaoPlantBundle>("DaoPlant")
            .add_systems(Update, setup)
            .add_systems(Update, animation);
    }
}
