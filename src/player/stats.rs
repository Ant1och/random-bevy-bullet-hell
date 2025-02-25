use super::{
    config::stats::{PLAYER_HEALTH, PLAYER_INVINCIBILITY_TIME},
    Player,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerStats {
    pub health: i64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: PLAYER_HEALTH,
        }
    }
}

#[derive(Event, Default)]
pub struct ChangeHealth(pub i64);

#[derive(Resource)]
pub struct InvincibilityTimer(Timer);

// fn set_player_default_stats(mut player: Query<&mut PlayerStats, Added<Player>>) {
//     let Ok(mut stats) = player.get_single_mut() else {
//         return;
//     };

//     stats.health = PLAYER_HEALTH;
// }

fn player_invincibility_timer(mut invicibility_timer: ResMut<InvincibilityTimer>, time: Res<Time>) {
    invicibility_timer.0.tick(time.delta());
}

pub fn player_damage(
    mut player: Query<&mut PlayerStats, With<Player>>,
    mut invicibility_timer: ResMut<InvincibilityTimer>,
    mut reader: EventReader<ChangeHealth>,
) {
    let Ok(mut stats) = player.get_single_mut() else {
        return;
    };

    if invicibility_timer.0.finished() {
        for event in reader.read() {
            stats.health += event.0;
            invicibility_timer.0.reset();
            info!("Health: {}", stats.health);
        }
    }
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeHealth>()
            .insert_resource(InvincibilityTimer(Timer::from_seconds(
                PLAYER_INVINCIBILITY_TIME,
                TimerMode::Once,
            )))
            .add_systems(Update, player_damage)
            .add_systems(Update, player_invincibility_timer);
    }
}
