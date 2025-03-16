use bevy::prelude::*;
use boss::BossPlugin;
use spawner::SpawnerPlugin;
use std::time::Duration;
use still::StillEnemyPlugin;

pub mod boss;
pub mod spawner;
pub mod still;

#[derive(Component, Default)]
pub struct ShootTimer(Timer);

impl ShootTimer {
    fn from_duration(duration: Duration, phase: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.set_elapsed(phase);
        ShootTimer(timer)
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnerPlugin)
            .add_plugins(BossPlugin)
            .add_plugins(StillEnemyPlugin);
    }
}
