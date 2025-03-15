use bevy::prelude::*;

use crate::player::{
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
        app.add_systems(Update, update);
    }
}
