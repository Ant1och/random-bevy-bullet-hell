use crate::input::Action;
use crate::physics::looking_direction::LookDir;
use crate::player::config::animation::*;
use crate::player::{LookingDirection, Player};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use leafwing_input_manager::prelude::ActionState;

pub fn player_animation(
    player: Single<
        (
            &ActionState<Action>,
            &mut AseAnimation,
            &mut Sprite,
            &LookingDirection,
        ),
        With<Player>,
    >,
) {
    let (input, mut asesprite, mut sprite, looking_direction) = player.into_inner();
    let direction = input.axis_pair(&Action::Direction);

    sprite.flip_x = match looking_direction.0 {
        LookDir::Right => false,
        LookDir::Left => true,
    };

    let animation = match asesprite.animation.tag.clone() {
        Some(val) => val,
        None => DEFAULT.to_string(),
    };

    let new_animation = match direction.x {
        0. => STAND,
        _ => WALK,
    };

    if animation != new_animation {
        asesprite.animation = Animation::tag(new_animation).with_speed(ANIMATION_SPEED);
    }
}

pub fn set_player_sprite(
    player: Single<(&mut AseAnimation, &mut Transform), Added<Player>>,
    server: Res<AssetServer>,
) {
    let (mut animation, mut transform) = player.into_inner();

    // Resize player and their sprite
    transform.scale.y *= 18. / 48.;

    animation.aseprite = server.load("reimu.aseprite");
    animation.animation = Animation::default().with_tag(DEFAULT);
}

// fn events(mut events: EventReader<AnimationEvents>, mut cmd: Commands) {
//     for event in events.read() {
//         match event {
//             AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn(),
//             AnimationEvents::LoopCycleFinished(_entity) => (),
//         };
//     }
// }

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_animation, set_player_sprite));
    }
}
