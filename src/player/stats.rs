use super::{
    config::stats::{PLAYER_HEALTH, PLAYER_INVINCIBILITY_TIME},
    Player,
};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PlayerStats {
    health: i64,
}

#[derive(Resource)]
pub struct InvincibilityTimer(Timer);

fn set_player_default_stats(mut player: Query<&mut PlayerStats, Added<Player>>) {
    let Ok(mut stats) = player.get_single_mut() else {
        return;
    };

    stats.health = PLAYER_HEALTH;
}

fn player_invincibility_timer(mut invicibility_timer: ResMut<InvincibilityTimer>, time: Res<Time>) {
    invicibility_timer.0.tick(time.delta());
}

pub fn player_damage(
    mut player: Query<&mut PlayerStats, With<Player>>,
    mut invicibility_timer: ResMut<InvincibilityTimer>,
) {
    let Ok(mut stats) = player.get_single_mut() else {
        return;
    };

    if invicibility_timer.0.finished() {
        stats.health -= 1;
        invicibility_timer.0.reset();
    }

    println!("{}", stats.health);
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InvincibilityTimer(Timer::from_seconds(
            PLAYER_INVINCIBILITY_TIME,
            TimerMode::Once,
        )))
        .add_systems(Update, set_player_default_stats)
        .add_systems(Update, player_invincibility_timer);
    }
}
