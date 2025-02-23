use bevy::prelude::*;
use boss::BossPlugin;
use spawner::SpawnerPlugin;
use std::time::Duration;

pub mod boss;
pub mod spawner;

#[derive(Component, Default)]
pub struct ShootTimer(Timer);

impl ShootTimer {
    fn from_duration(duration: Duration) -> Self {
        ShootTimer(Timer::new(duration, TimerMode::Repeating))
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnerPlugin).add_plugins(BossPlugin);
    }
}
