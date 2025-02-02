use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Bullet;
use crate::physics::shared::Acceleration;

// fn bullet_physics_enabled(
//     mut removals: RemovedComponents<RigidBodyDisabled>,
//     bullets_query: Query<Entity, With<Bullet>>,
// ) -> bool {
//     for entity in removals.read() {
//         return match bullets_query.get(entity) {
//             Ok(_) => true,
//             Err(_) => false,
//         };
//     }
//     false
// }

fn bullet_acceleration(mut bullets: Query<(&mut Velocity, &Acceleration), With<Bullet>>) {
    for (mut velocity, acceleration) in &mut bullets {
        velocity.linvel += acceleration.0;
    }
}

pub struct BulletPhysicsPlugin;

impl Plugin for BulletPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_acceleration);
    }
}
