use crate::input::{CustomInput, KeysPressed};
use bevy::prelude::*;

pub mod bar;
use bar::{health::HealthBarBundle, stamina::StaminaBarBundle, StatusBarPlugin};

pub mod gui_parts;
use gui_parts::*;

use crate::player::config::stats::{PLAYER_HEALTH, PLAYER_STAMINA};

fn setup_gui(mut cmd: Commands) {
    cmd.spawn(GuiLeftBundle::default()).with_children(|parent| {
        parent.spawn(HealthBarBundle {
            node: Node {
                width: Val::Px(27.0),
                height: Val::Px(23.0),
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Start,
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::px(10.0, 10.0, 10.0, 10.0),
                ..default()
            },
            color: BackgroundColor(Color::srgba(0.09, 0.09, 0.1, 0.65)),
            text: Text::new(PLAYER_HEALTH.to_string()),
            ..default()
        });
    });

    cmd.spawn(GuiTopBundle::default());
    cmd.spawn(GuiBottomBundle::default());

    cmd.spawn(GuiRightBundle::default())
        .with_children(|parent| {
            parent.spawn(StaminaBarBundle {
                node: Node {
                    width: Val::Px(27.0),
                    height: Val::Px(23.0),
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::End,
                    justify_self: JustifySelf::Center,
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::px(10.0, 10.0, 10.0, 10.0),
                    ..default()
                },
                color: BackgroundColor(Color::srgba(0.09, 0.09, 0.1, 0.65)),
                text: Text::new(PLAYER_STAMINA.to_string()),
                ..default()
            });
        });
}

pub struct GuiPlugin;

#[cfg(debug_assertions)]
#[derive(Resource, Default, PartialEq, Debug)]
pub enum GuiDebugLevel {
    Full,
    #[default]
    None,
}

impl GuiDebugLevel {
    fn next(&mut self) {
        use GuiDebugLevel::*;
        *self = match self {
            None => Full,
            Full => None,
        };
    }
}

#[cfg(debug_assertions)]
fn toggle_gui_debug(
    input: Query<&KeysPressed, With<CustomInput>>,
    mut gui_debug: ResMut<GuiDebugLevel>,
) {
    use crate::input::config::KeyType;

    let Ok(keys) = input.get_single() else {
        return;
    };

    if keys.just_pressed(KeyType::GuiDebugToggle) {
        gui_debug.next();
        info!("Gui Debug level: {:?}", *gui_debug);
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiScale(3.))
            .add_systems(Startup, setup_gui)
            .add_plugins(GuiPartsPlugin)
            .add_plugins(StatusBarPlugin);

        #[cfg(debug_assertions)]
        app.insert_resource(GuiDebugLevel::default())
            .add_systems(Update, toggle_gui_debug);
    }
}
