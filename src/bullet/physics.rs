use std::any::Any;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Bullet;
use super::BulletPivot;
use super::PhysicsEnabled;
use crate::physics::shared::Acceleration;
use crate::physics::shared::AccelerationScale;

fn bullet_setup(
    mut bullets: Query<(&mut Velocity, &mut Acceleration, &BulletPivot, &Transform), With<Bullet>>,
) {
    for (
        mut velocity,
        mut acceleration,
        BulletPivot(Transform {
            translation: pivot,
            rotation: _,
            scale: _,
        }),
        Transform {
            translation,
            rotation: _,
            scale: _,
        },
    ) in &mut bullets
    {
        velocity.linvel = Vec2::from_angle(PI / 2.).rotate((pivot - translation).truncate());
        velocity.linvel = velocity.linvel / velocity.linvel.length() * 1.;
        acceleration.0 = Vec2::ZERO;
    }
}

fn bullet_physics_enabled(
    mut removals: RemovedComponents<RigidBodyDisabled>,
    bullets_query: Query<Entity, With<Bullet>>,
) -> bool {
    for entity in removals.read() {
        return match bullets_query.get(entity) {
            Ok(_) => true,
            Err(_) => false,
        };
    }
    false
}

fn bullet_acceleration(
    mut bullets: Query<(&mut Velocity, &Acceleration, &PhysicsEnabled), With<Bullet>>,
) {
    for (mut velocity, acceleration, physics_enabled) in &mut bullets {
        if physics_enabled.0 {
            velocity.linvel += acceleration.0;
        } else {
            velocity.linvel = Vec2::ZERO;
        }
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

        acceleration.0 = (destination - position) * (0.5 * delta) as f32;
        println!("{:?}", acceleration.0);
    }
}

pub struct BulletPhysicsPlugin;

impl Plugin for BulletPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_setup.run_if(bullet_physics_enabled));
        app.add_systems(Update, bullet_acceleration.run_if());
        app.add_systems(Update, accelerate_towards_pivot);
    }
}
