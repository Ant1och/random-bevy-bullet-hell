use bevy::prelude::*;
pub mod turret;
use turret::TurretPlugin;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TurretPlugin);
    }
}
