use crate::physics::movement::MovementType;
use crate::physics::Acceleration;
use crate::player::stats::player_damage;
use crate::{colliders::SensorBundle, player::Player};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod config;

mod animation;
use animation::BulletAnimationPlugin;
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};

// mod physics;
// use physics::BulletPhysicsPlugin;

#[derive(PartialEq, Debug, Default, Component)]
pub struct Bullet;

#[derive(PartialEq, Debug, Default, Component)]
pub struct BulletParams {
    acceleration_scale: f64,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct BulletBundle {
    pub entity: Bullet,
    pub name: Name,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub params: BulletParams,
    pub movement: MovementType,
    // #[worldly]
    // pub worldly: Worldly,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

fn bullet_player_collision(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Has<Player>>,
    bullet_query: Query<Has<Bullet>>,
) -> bool {
    collision_events.read().any(|event| match event {
        CollisionEvent::Started(e1, e2, _) => {
            (player_query.contains(*e1) || player_query.contains(*e2))
                && (bullet_query.contains(*e1) || bullet_query.contains(*e2))
        }
        _ => false,
    })
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BulletBundle>("Bullet")
            // app
            // .add_plugins(BulletPhysicsPlugin)
            // .add_systems(Update, bullet_player_collision)
            .add_systems(Update, player_damage.run_if(bullet_player_collision))
            .add_plugins(BulletAnimationPlugin);
    }
}
