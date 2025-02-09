use std::str::FromStr;

use super::SpellCardList;
use crate::colliders::SensorBundle;
use crate::spell_card::SpellCard;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Yuyuko;

#[derive(Bundle, Default, LdtkEntity)]
pub struct YuyukoBundle {
    pub entity: Yuyuko,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[with(SpellCardList::from_field)]
    pub spell_card_list: SpellCardList,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

impl SpellCardList {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        SpellCardList(
            entity_instance
                .get_maybe_enums_field("spell_cards")
                .expect("Yuyuko should have spell_cards field")
                .iter()
                .map(|card| card.clone().unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

fn yuyuko_fight(mut bosses: Query<(Entity, &mut SpellCardList), Added<Yuyuko>>, mut cmd: Commands) {
    for (yuyuko, mut card_list) in &mut bosses {
        let card = match card_list.0.first() {
            Some(val) => val.clone(),
            None => {
                return;
            }
        };

        card_list.0.retain(|elem| *elem != card);
        let Ok(card) = SpellCard::from_str(card.as_str()) else {
            println!("No more spell cards!");
            return;
        };
        let card = match card {
            SpellCard::CirclesOfFifth(val) => val,
        };

        let card_id = cmd.spawn(card).id();
        cmd.entity(yuyuko).add_child(card_id);
        println!("Yuyuko added!");
    }
}

pub struct YuyukoPlugin;

impl Plugin for YuyukoPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<YuyukoBundle>("Yuyuko")
            .add_systems(Update, yuyuko_fight);
    }
}
