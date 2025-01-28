use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default)]
pub struct AccelerationScale(pub f64);
