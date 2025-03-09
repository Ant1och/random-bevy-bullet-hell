use bevy::prelude::*;

use crate::player::{
    config::stats::PLAYER_STAMINA,
    stats::{ChangeStamina, PlayerStats},
    Player,
};

#[derive(Component, Default)]
pub struct StaminaBar;

#[derive(Bundle, Default)]
pub struct StaminaBarBundle {
    pub entity: StaminaBar,
    pub transform: Transform,
    pub color: BackgroundColor,
    pub node: Node,
    pub text: Text,
}

fn spawn(mut cmd: Commands) {
    cmd.spawn(StaminaBarBundle {
        node: Node {
            width: Val::Px(27.0),
            height: Val::Px(23.0),
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(8.0)),
            margin: UiRect::px(1000., 10.0, 10.0, 0.0),
            ..default()
        },
        color: BackgroundColor(Color::srgba(0.08, 0.08, 0.09, 0.7)),
        text: Text::new(PLAYER_STAMINA.to_string()),
        ..default()
    });
}

fn update(
    mut stamina_bars: Query<&mut Text, With<StaminaBar>>,
    stamina_events: EventReader<ChangeStamina>,
    player: Query<&PlayerStats, With<Player>>,
) {
    if stamina_events.is_empty() {
        return;
    };

    let Ok(PlayerStats { stamina, .. }) = player.get_single() else {
        return;
    };

    for mut text in &mut stamina_bars {
        **text = stamina.to_string();
    }
}

pub struct StaminaBarPlugin;

impl Plugin for StaminaBarPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn)
        app.add_systems(Update, update);
    }
}
