use std::f64::consts::PI;

use bevy::{math::IVec2, prelude::Vec2};

pub const fn ldtk_to_bevy_vec2(from: IVec2) -> Vec2 {
    Vec2 {
        x: from.x as f32,
        y: -from.y as f32,
    }
}

pub fn move_toward_f32(from: f32, to: f32, delta: f64) -> f32 {
    let diff = to - from;

    match diff.abs() as f64 <= delta || diff.abs() <= 1e-8 {
        true => to,
        // Same as from + diff.signum() * delta as f32,
        false => diff.signum().mul_add(delta as f32, from),
    }
}

pub fn move_toward_vec2(from: Vec2, to: Vec2, delta: f64) -> Vec2 {
    let delta_vec = to - from;
    let length = delta_vec.length();

    match length as f64 <= delta || length <= 1e-4 {
        true => to,
        false => from + delta_vec / length * delta as f32,
    }
}

#[allow(dead_code)]
pub fn move_toward_exp_vec2(from: Vec2, to: Vec2, easing: f64, min_speed: f64, delta: f64) -> Vec2 {
    let distance = from.distance(to) as f64;
    move_toward_vec2(from, to, (easing * distance).exp().max(min_speed) * delta)
}

pub fn move_toward_exp_f32(from: f32, to: f32, easing: f64, min_speed: f64, delta: f64) -> f32 {
    let distance = (from - to).abs() as f64;
    move_toward_f32(from, to, (easing * distance).exp().max(min_speed) * delta)
}

#[allow(dead_code)]
pub fn move_toward_sigmoid_f32(from: f32, to: f32, easing: f64, min_speed: f64, delta: f64) -> f32 {
    let distance = (from - to).abs() as f64;
    move_toward_f32(
        from,
        to,
        (distance * easing / (1. + (-distance * easing).exp())).max(min_speed) * delta,
    )
}

#[allow(dead_code)]
pub fn move_toward_sin_in_f32(from: f32, to: f32, easing: f64, min_speed: f64, delta: f64) -> f32 {
    let distance = (from - to).abs() as f64;
    move_toward_f32(
        from,
        to,
        (1. - (distance * easing * PI / 2.).cos()).max(min_speed) * delta,
    )
}

pub fn move_toward_quad_f32(from: f32, to: f32, easing: f64, min_speed: f64, delta: f64) -> f32 {
    let distance = (from - to).abs() as f64;
    move_toward_f32(
        from,
        to,
        ((distance * easing).powi(2)).max(min_speed) * delta,
    )
}
