use crate::bullet::Bullet;
use crate::player::config::animation::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub fn set_bullet_sprite(
    mut bullet: Query<&mut AseSpriteAnimation, Added<Bullet>>,
    server: Res<AssetServer>,
) {
    for mut animation in &mut bullet {
        animation.aseprite = server.load("reimu.aseprite");
        animation.animation = Animation::default().with_tag(DEFAULT);
    }
}

// fn events(mut events: EventReader<AnimationEvents>, mut cmd: Commands) {
//     for event in events.read() {
//         match event {
//             AnimationEvents::Finished(entity) => cmd.entity(*entity).despawn_recursive(),
//             AnimationEvents::LoopCycleFinished(_entity) => (),
//         };
//     }
// }
//
pub struct AnimationPlugin1;

impl Plugin for AnimationPlugin1 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_bullet_sprite);
    }
}
