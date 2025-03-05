use bevy::prelude::*;

use crate::player::{
    config::stats::PLAYER_HEALTH,
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

fn spawn_health_bar(mut cmd: Commands) {
    cmd.spawn(HealthBarBundle {
        node: Node {
            width: Val::Px(27.0),
            height: Val::Px(23.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(8.0)),
            margin: UiRect::px(10.0, 10.0, 10.0, 0.0),

            ..default()
        },
        color: BackgroundColor(Color::srgba(0.08, 0.08, 0.09, 0.7)),
        text: Text::new(PLAYER_HEALTH.to_string()),
        ..default()
    });
}

fn update_health_bar(
    mut health_bars: Query<&mut Text, With<HealthBar>>,
    health_events: EventReader<ChangeHealth>,
    player: Query<&PlayerStats, With<Player>>,
) {
    if health_events.is_empty() {
        return;
    };

    let Ok(PlayerStats { health, .. }) = player.get_single() else {
        return;
    };

    for mut text in &mut health_bars {
        **text = health.to_string();
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_health_bar)
            .add_systems(Update, update_health_bar);
    }
}
