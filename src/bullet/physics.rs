use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Bullet;
use crate::physics::Acceleration;

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

pub struct BulletPhysicsPlugin;

impl Plugin for BulletPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_acceleration);
    }
}
