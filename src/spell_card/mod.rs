use std::str::FromStr;

use bevy::prelude::*;
use thiserror::Error;

pub mod circles_of_fifth;
use circles_of_fifth::circles_of_fifth;

#[derive(Debug, Error)]
#[error("the given SpellCard doesn't exist")]
pub struct NoSuchSpellCard;

pub enum SpellCard {
    CirclesOfFifth,
}

impl FromStr for SpellCard {
    type Err = NoSuchSpellCard;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use SpellCard::*;
        match string {
            "CirclesOfFifth" => Ok(CirclesOfFifth),
            _ => Err(NoSuchSpellCard),
        }
    }
}

pub struct SpellCardPlugin;

impl Plugin for SpellCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, circles_of_fifth);
        // .add_systems(Update, cof_setup_circles_after_construction);
    }
}
