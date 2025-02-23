use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use super::Acceleration;

#[derive(Component, Default, PartialEq, Debug, Clone)]
pub enum MovementType {
    #[default]
    Still,
    Linear {
        velocity: Vec2,
        accel: f32,
    },
    Circle {
        speed: f32,
        accel: f32,
    },
}

impl MovementType {
    pub fn start_velocity(&self, position: Vec2) -> Vec2 {
        use MovementType::*;
        match self {
            Still => Vec2::ZERO,
            Linear { velocity, accel: _ } => *velocity,
            Circle { speed, accel: _ } => {
                let velocity = Vec2::from_angle(PI / 2.).rotate(position);
                velocity / velocity.length() * speed
            }
        }
    }
    pub fn acceleration(&self, position: Vec2) -> Vec2 {
        use MovementType::*;
        match self {
            Still => Vec2::ZERO,
            Linear { velocity, accel } => velocity.normalize_or_zero() * accel,
            Circle { speed: _, accel } => -position.normalize_or_zero() * accel,
        }
    }
}

fn movement_start_velocity(
    mut entities: Query<(&MovementType, &Transform, &mut Velocity), With<RigidBodyDisabled>>,
) {
    for (movement, transform, mut velocity) in &mut entities {
        let relative_position = transform.translation.truncate();
        velocity.linvel = movement.start_velocity(relative_position);
    }
}

fn movement_acceleration(
    mut entities: Query<(&MovementType, &Transform, &mut Acceleration), Without<RigidBodyDisabled>>,
) {
    for (movement, transform, mut accel) in &mut entities {
        let relative_position = transform.translation.truncate();
        accel.0 = movement.acceleration(relative_position);
    }
}

pub struct MovementTypePlugin;

impl Plugin for MovementTypePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_start_velocity)
            .add_systems(Update, movement_acceleration);
    }
}
