use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Default, PartialEq, Debug, Clone)]
pub enum MovementType {
    #[default]
    Still,
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
            Circle { speed: _, accel } => -position / position.length() * accel,
        }
    }
}
