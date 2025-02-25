use bevy::prelude::*;
use health::HealthBarPlugin;

pub mod health;

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthBarPlugin);
    }
}
