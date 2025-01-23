use crate::player::player_damage;
use crate::{colliders::SensorBundle, player::Player};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod animation;
use animation::*;
use bevy_rapier2d::plugin::ReadRapierContext;

#[derive(PartialEq, Debug, Default, Component)]
pub struct Bullet;

#[derive(Default, Bundle, LdtkEntity)]
pub struct BulletBundle {
    #[sprite("bullet.png")]
    pub sprite: Sprite,
    // pub animation: AseSpriteAnimation,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub bullet: Bullet,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

fn bullet_player_collision(
    rapier_context: ReadRapierContext,
    player: Query<(Entity, &Player)>,
    bullets: Query<(Entity, &Bullet)>,
) -> bool {
    let Ok((player, _)) = player.get_single() else {
        return false;
    };
    for (bullet, _) in &bullets {
        let Some(bullet_in_player) = rapier_context.single().intersection_pair(player, bullet)
        else {
            return false;
        };

        if bullet_in_player {
            return true;
        }
    }
    false
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BulletBundle>("Bullet")
            // .add_plugins(PhysicsPlugin)
            // .add_systems(Update, bullet_player_collision)
            .add_systems(Update, player_damage.run_if(bullet_player_collision))
            .add_plugins(AnimationPlugin1);
    }
}
