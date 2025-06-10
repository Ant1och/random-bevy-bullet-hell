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
    pub animation: AseAnimation,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

fn setup(plants: Query<(Entity, &EntityInstance), Added<DaoPlant>>, mut cmd: Commands) {
    for (plant, ldtk_entity) in &plants {
        let ammo = TurretAmmoList::new(vec![TurretAmmo {
            params: PatternParams {
                scale: 0.,
                bullet_amount: 1,
                bullet_movement: MovementType::Still,
                construction_frequency: Duration::from_secs_f64(0.),
            },
            construction: ConstructionType::Circle,
            speed: *ldtk_entity
                .get_float_field("bullet_speed")
                .expect("DaoPlant should have bullet_speed field"),
            accel: *ldtk_entity
                .get_float_field("bullet_accel")
                .expect("DaoPlant should have bullet_accel field"),
        }]);

        let shoot_delay = *ldtk_entity
            .get_float_field("shoot_delay")
            .expect("DaoPlant should have shoot_delay field");
        let shoot_phase = *ldtk_entity
            .get_float_field("shoot_phase")
            .expect("DaoPlant should have shoot_offset field");
        cmd.spawn(TurretBundle::from_params(
            ammo,
            Duration::from_secs_f32(shoot_delay),
            Duration::from_secs_f32(shoot_phase),
        ))
        .insert(ChildOf(plant));
    }
}

fn animation(mut plants: Query<&mut AseAnimation, Added<DaoPlant>>, server: Res<AssetServer>) {
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
