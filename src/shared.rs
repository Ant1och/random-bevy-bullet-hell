use bevy::prelude::Vec2;

pub fn move_toward_f32(from: f32, to: f32, delta: f64) -> f32 {
    let diff = to - from;

    match diff.abs() as f64 <= delta || diff.abs() <= 1e-8 {
        true => to,
        false => from + diff.signum() * delta as f32,
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

pub fn move_toward_exp_vec2(from: Vec2, to: Vec2, easing: f64, min_speed: f64, delta: f64) -> Vec2 {
    let distance = from.distance(to) as f64;
    move_toward_vec2(from, to, (easing * distance).exp().max(min_speed) * delta)
}
