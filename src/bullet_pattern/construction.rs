use bevy::prelude::*;
use bevy_inspector_egui::egui::emath::Numeric;
use std::f32::consts::PI;

#[derive(Component, Default, PartialEq, Debug, Clone)]
pub enum ConstructionType {
    #[default]
    Circle,
    RegularPolygon(u64),
    Shuriken(u64),
}

impl ConstructionType {
    pub fn translation(&self, progress: u64, bullet_amount: u64) -> Vec2 {
        use ConstructionType::*;
        match self {
            Circle => Vec2::from_angle((progress as f32 * 2. * PI) / bullet_amount as f32),

            RegularPolygon(n) => {
                let n = n.to_f64();
                let polygon_angle: f64 = 180. * (n - 2.) / n;
                let phi: f32 = (180. - polygon_angle) as f32 / 2.;

                let phi = (((360 * progress / bullet_amount) as f32 + phi) % (2. * phi) - phi)
                    as f32
                    / 180.
                    * PI;

                Vec2::from_angle((progress as f32 * 2. * PI) / bullet_amount as f32)
                    * (1. / phi.cos())
            }

            Shuriken(n) => {
                let n = n.to_f64();
                let polygon_angle: f64 = 180. * (n - 2.) / n;
                let phi: f32 = (180. - polygon_angle) as f32 / 2.;

                let phi = (((360 * progress / bullet_amount) as f32 + phi) % (2. * phi) - phi)
                    as f32
                    / 180.
                    * PI;

                Vec2::from_angle((progress as f32 * 2. * PI) / bullet_amount as f32 + phi)
                    * (1. / phi.cos())
            }
        }
    }
}
