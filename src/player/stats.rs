use super::{config::stats::*, physics::Dash, Player};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerStats {
    pub health: i64,
    pub stamina: i64,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            health: PLAYER_HEALTH,
            stamina: PLAYER_STAMINA,
        }
    }
}

#[derive(Event, Default)]
pub struct ChangeHealth(pub i64);

#[derive(Event, Default)]
pub struct ChangeStamina(pub i64);

#[derive(Resource)]
pub struct InvincibilityTimer(Timer);

#[derive(Resource)]
pub struct StaminaTimer(Timer);

fn player_invincibility_timer(mut invicibility_timer: ResMut<InvincibilityTimer>, time: Res<Time>) {
    invicibility_timer.0.tick(time.delta());
}

fn decrease_stamina_on_dash(
    mut dash_event: EventReader<Dash>,
    mut stamina_timer: ResMut<StaminaTimer>,
    mut event_writer: EventWriter<ChangeStamina>,
) {
    for _ in dash_event.read() {
        event_writer.write(ChangeStamina(-1));
        stamina_timer.0.reset();
    }
}

fn stamina_regen(
    stats: Single<&PlayerStats, With<Player>>,
    mut timer: ResMut<StaminaTimer>,
    mut event_writer: EventWriter<ChangeStamina>,
    time: Res<Time>,
) {
    let stamina = &stats.stamina;

    if stamina >= &PLAYER_STAMINA {
        return;
    };

    if timer.0.tick(time.delta()).just_finished() {
        event_writer.write(ChangeStamina(1));
        timer.0.reset();
    };
}

pub fn stamina_change_event(
    mut player: Single<&mut PlayerStats, With<Player>>,
    mut reader: EventReader<ChangeStamina>,
) {
    for event in reader.read() {
        player.stamina += event.0;
        info!("Stamina: {}", player.stamina);
    }
}

fn health_change_event(
    mut player: Single<&mut PlayerStats, With<Player>>,
    mut invicibility_timer: ResMut<InvincibilityTimer>,
    mut reader: EventReader<ChangeHealth>,
) {
    if invicibility_timer.0.finished() {
        for event in reader.read() {
            player.health += event.0;
            invicibility_timer.0.reset();
            info!("Health: {}", player.health);
        }
    }
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeHealth>()
            .add_event::<ChangeStamina>()
            .insert_resource(InvincibilityTimer(Timer::from_seconds(
                PLAYER_INVINCIBILITY_TIME,
                TimerMode::Once,
            )))
            .insert_resource(StaminaTimer(Timer::from_seconds(
                PLAYER_STAMINA_REGEN_TIME,
                TimerMode::Once,
            )))
            .add_systems(Update, health_change_event)
            .add_systems(Update, player_invincibility_timer)
            .add_systems(Update, decrease_stamina_on_dash)
            .add_systems(Update, stamina_change_event)
            .add_systems(Update, stamina_regen);
    }
}
