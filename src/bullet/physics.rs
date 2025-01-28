use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::BulletPivot;
// use crate::bullet::config::physics::*;
use super::Bullet;
use crate::physics::shared::Acceleration;
use crate::physics::shared::AccelerationScale;

fn bullet_setup(
    mut bullets: Query<(&mut Velocity, &mut Acceleration, &BulletPivot, &Transform), Added<Bullet>>,
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
        acceleration.0 = Vec2::ZERO;
    }
}

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

        acceleration.0 = (destination - position) * (0.4 * delta) as f32;
    }
}

pub struct BulletPhysicsPlugin;

impl Plugin for BulletPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_setup);
        app.add_systems(Update, bullet_acceleration);
        app.add_systems(Update, accelerate_towards_pivot);
    }
}
