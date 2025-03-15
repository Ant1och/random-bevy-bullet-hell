use bevy::prelude::*;
use dao_plant::DaoPlantPlugin;

mod dao_plant;

pub struct StillEnemyPlugin;

impl Plugin for StillEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DaoPlantPlugin);
    }
}
