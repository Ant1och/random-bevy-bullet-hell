use std::time::Duration;

use super::SpellCardList;
use crate::colliders::SensorBundle;
use crate::spell_card::circle_of_fifth::CirclesOfFifthBundle;
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

#[derive(Event)]
struct SpawnCardEvent {
    parent: Entity,
    card: SpellCard,
}

impl SpawnCardEvent {
    pub fn new(parent: Entity, card: SpellCard) -> Self {
        SpawnCardEvent { parent, card }
    }
}

impl Yuyuko {
    fn fight(
        mut bosses: Query<(Entity, &mut SpellCardList), Added<Yuyuko>>,
        mut card_events: EventWriter<SpawnCardEvent>,
    ) {
        for (yuyuko, mut card_list) in &mut bosses {
            let Some(card) = card_list.random() else {
                println!("No more spell cards!");
                return;
            };

            card_events.send(SpawnCardEvent::new(yuyuko, card));
            println!("Yuyuko added!");
        }
    }

    fn spawn_card(mut card_events: EventMutator<SpawnCardEvent>, mut cmd: Commands) {
        for event in card_events.read() {
            use SpellCard::*;
            match event.card {
                CirclesOfFifth => cmd
                    .spawn(Yuyuko::circle_of_fifth())
                    .set_parent(event.parent),
            };
        }
    }

    fn circle_of_fifth() -> CirclesOfFifthBundle {
        let frequency = Duration::from_secs_f64(1.);
        let length = Duration::from_secs_f64(100.);

        CirclesOfFifthBundle::new(frequency, length)
    }
}

pub struct YuyukoPlugin;

impl Plugin for YuyukoPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<YuyukoBundle>("Yuyuko")
            .add_event::<SpawnCardEvent>()
            .add_systems(Update, (Yuyuko::fight, Yuyuko::spawn_card));
    }
}
