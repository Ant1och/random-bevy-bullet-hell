use bevy::prelude::*;

#[cfg(debug_assertions)]
use super::GuiDebugLevel;

#[derive(Component, Default)]
pub struct GuiDebugColor(Color);

#[derive(Component, Default)]
pub struct GuiLeft;

#[derive(Bundle)]
pub struct GuiLeftBundle {
    pub entity: GuiLeft,
    pub node: Node,
    debug_color: GuiDebugColor,
}

impl Default for GuiLeftBundle {
    fn default() -> Self {
        GuiLeftBundle {
            entity: GuiLeft,
            node: Node {
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                justify_self: JustifySelf::Start,
                justify_content: JustifyContent::Start,
                align_self: AlignSelf::Start,
                ..default()
            },
            debug_color: GuiDebugColor(Color::srgba(0.38, 0.08, 0.08, 0.7)),
        }
    }
}

#[derive(Component, Default)]
pub struct GuiRight;

#[derive(Bundle)]
pub struct GuiRightBundle {
    pub entity: GuiRight,
    pub node: Node,
    debug_color: GuiDebugColor,
}

impl Default for GuiRightBundle {
    fn default() -> Self {
        GuiRightBundle {
            entity: GuiRight,
            node: Node {
                width: Val::Percent(23.),
                height: Val::Percent(100.),
                justify_self: JustifySelf::End,
                justify_content: JustifyContent::End,
                align_self: AlignSelf::Start,
                ..default()
            },
            debug_color: GuiDebugColor(Color::srgba(0.08, 0.08, 0.38, 0.7)),
        }
    }
}

#[derive(Component, Default)]
pub struct GuiTop;

#[derive(Bundle)]
pub struct GuiTopBundle {
    pub entity: GuiTop,
    pub node: Node,
    debug_color: GuiDebugColor,
}

impl Default for GuiTopBundle {
    fn default() -> Self {
        GuiTopBundle {
            entity: GuiTop,
            node: Node {
                width: Val::Percent(54.),
                height: Val::Percent(23.),
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Start,
                ..default()
            },
            debug_color: GuiDebugColor(Color::srgba(0.08, 0.38, 0.08, 0.7)),
        }
    }
}

#[derive(Component, Default)]
pub struct GuiBottom;

#[derive(Bundle)]
pub struct GuiBottomBundle {
    pub entity: GuiBottom,
    pub node: Node,
    debug_color: GuiDebugColor,
}

impl Default for GuiBottomBundle {
    fn default() -> Self {
        GuiBottomBundle {
            entity: GuiBottom,
            node: Node {
                width: Val::Percent(54.),
                height: Val::Percent(23.),
                justify_self: JustifySelf::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::End,
                ..default()
            },
            debug_color: GuiDebugColor(Color::srgba(0.38, 0.38, 0.08, 0.7)),
        }
    }
}

#[cfg(debug_assertions)]
fn insert_debug_gui_color(
    parts: Query<(Entity, &GuiDebugColor), With<GuiDebugColor>>,
    mut cmd: Commands,
) {
    for (part, color) in &parts {
        cmd.entity(part).insert(BackgroundColor(color.0));
    }
}

#[cfg(debug_assertions)]
fn remove_debug_gui_color(parts: Query<Entity, With<GuiDebugColor>>, mut cmd: Commands) {
    for part in &parts {
        cmd.entity(part).remove::<BackgroundColor>();
    }
}

pub struct GuiPartsPlugin;

impl Plugin for GuiPartsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (
                remove_debug_gui_color
                    .run_if(resource_equals::<GuiDebugLevel>(GuiDebugLevel::None)),
                insert_debug_gui_color
                    .run_if(resource_equals::<GuiDebugLevel>(GuiDebugLevel::Full)),
            )
                .chain(),
        );
    }
}
