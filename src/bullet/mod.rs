use crate::physics::movement::MovementType;
use crate::physics::{Acceleration, DespawnIfOutOfBounds};
use crate::player::stats::ChangeHealth;
use crate::{colliders::SensorBundle, player::Player};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod config;

mod animation;
use animation::BulletAnimationPlugin;
use bevy_rapier2d::plugin::ReadRapierContext;
use bevy_rapier2d::prelude::Velocity;

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
    pub animation: AseAnimation,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub params: BulletParams,
    pub movement: MovementType,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub auto_despawn: DespawnIfOutOfBounds,
}

fn bullet_player_collision(
    rapier_context: ReadRapierContext,
    player: Single<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut health_event: EventWriter<ChangeHealth>,
) -> Result {
    let context = rapier_context.single()?;

    if bullet_query
        .iter()
        .any(|bullet| context.intersection_pair(bullet, player.entity()) == Some(true))
    {
        health_event.write(ChangeHealth(-1));
    }
    Ok(())
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BulletBundle>("Bullet")
            .add_systems(Update, bullet_player_collision)
            .add_plugins(BulletAnimationPlugin);
    }
}
