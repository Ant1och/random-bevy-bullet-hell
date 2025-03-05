use bevy::prelude::*;
use health::HealthBarPlugin;
use stamina::StaminaBarPlugin;

pub mod health;
pub mod stamina;

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthBarPlugin)
            .add_plugins(StaminaBarPlugin);
    }
}
