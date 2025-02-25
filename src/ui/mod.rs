use bevy::prelude::*;

pub mod bar;
use bar::StatusBarPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatusBarPlugin);
    }
}
