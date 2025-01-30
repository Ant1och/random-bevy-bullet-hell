use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Bullet;
use super::BulletPivot;
use crate::physics::shared::Acceleration;
use crate::physics::shared::AccelerationScale;

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

fn accelerate_towards_pivot(
    mut bullets: Query<
        (
            &mut Acceleration,
            &AccelerationScale,
            &BulletPivot,
            &Transform,
        ),
        With<Bullet>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta_secs_f64();

    for (mut acceleration, acceleration_scale, pivot, transform) in &mut bullets {
        let position = transform.translation.truncate();
        let destination = pivot.0.translation.truncate();

        // println!("{:?}", acceleration.0);
        acceleration.0 = -position * (2.5 * delta) as f32;
        // println!("{:?}", acceleration.0);
    }
}

pub struct BulletPhysicsPlugin;

impl Plugin for BulletPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_acceleration);
        app.add_systems(Update, accelerate_towards_pivot);
    }
}
