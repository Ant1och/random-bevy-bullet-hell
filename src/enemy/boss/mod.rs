use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::prelude::*;
use yuyuko::YuyukoPlugin;

use crate::spell_card::SpellCard;

pub mod yuyuko;

#[derive(Component, Default)]
pub struct SpellCardList(Vec<SpellCard>);

impl SpellCardList {
    fn random(&mut self) -> Option<SpellCard> {
        let i = (0..self.0.len()).choose(&mut rand::rng())?;
        Some(self.0.swap_remove(i))
    }

    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        SpellCardList(
            entity_instance
                .get_maybe_enums_field("spell_cards")
                .expect("Entity should have spell_cards field")
                .iter()
                .map(|card| {
                    SpellCard::from_str(card.clone().unwrap().as_str())
                        .expect("SpellCard with specified name should exist")
                })
                .collect::<Vec<_>>(),
        )
    }
}
pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YuyukoPlugin);
    }
}
