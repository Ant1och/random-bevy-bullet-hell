use bevy::math::Vec2;
use LookDir::*;

#[derive(Default)]
pub enum LookDir {
    #[default]
    Right,
    Left,
}

impl From<&LookDir> for Vec2 {
    fn from(val: &LookDir) -> Vec2 {
        match val {
            Right => Vec2::X,
            Left => Vec2::NEG_X,
        }
    }
}

impl From<&LookDir> for f32 {
    fn from(val: &LookDir) -> f32 {
        match val {
            Right => 1.,
            Left => -1.,
        }
    }
}

impl From<&LookDir> for f64 {
    fn from(val: &LookDir) -> f64 {
        match val {
            Right => 1.,
            Left => -1.,
        }
    }
}

impl From<&LookDir> for i32 {
    fn from(val: &LookDir) -> i32 {
        match val {
            Right => 1,
            Left => -1,
        }
    }
}

impl From<&LookDir> for i8 {
    fn from(val: &LookDir) -> i8 {
        match val {
            Right => 1,
            Left => -1,
        }
    }
}

impl From<f32> for LookDir {
    fn from(val: f32) -> Self {
        match val {
            x if x > 0. => Right,
            x if x < 0. => Left,
            _ => LookDir::default(),
        }
    }
}

impl From<f64> for LookDir {
    fn from(val: f64) -> Self {
        match val {
            x if x > 0. => Right,
            x if x < 0. => Left,
            _ => LookDir::default(),
        }
    }
}

impl From<i32> for LookDir {
    fn from(val: i32) -> Self {
        match val {
            x if x > 0 => Right,
            x if x < 0 => Left,
            _ => LookDir::default(),
        }
    }
}

impl From<i8> for LookDir {
    fn from(val: i8) -> Self {
        match val {
            x if x > 0 => Right,
            x if x < 0 => Left,
            _ => LookDir::default(),
        }
    }
}
