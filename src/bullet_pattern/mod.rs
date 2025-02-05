use bevy::prelude::*;

pub mod circle;

pub struct PatternsPlugin;

impl Plugin for PatternsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(circle::CirclePatternPlugin);
    }
}
