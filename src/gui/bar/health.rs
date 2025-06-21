use bevy::prelude::*;

use crate::player::{
    stats::{ChangeHealth, PlayerStats},
    Player,
};

#[derive(Component, Default)]
pub struct HealthBar;

#[derive(Bundle, Default)]
pub struct HealthBarBundle {
    pub entity: HealthBar,
    pub transform: Transform,
    pub color: BackgroundColor,
    pub node: Node,
    pub text: Text,
}

fn update_health_bar(
    mut health_bars: Query<&mut Text, With<HealthBar>>,
    health_events: EventReader<ChangeHealth>,
    player: Single<&PlayerStats, With<Player>>,
) {
    if health_events.is_empty() {
        return;
    };

    let PlayerStats { health, .. } = player.into_inner();

    for mut text in &mut health_bars {
        **text = health.to_string();
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_health_bar);
    }
}
