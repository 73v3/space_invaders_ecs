use crate::assets::GameAssets;
use crate::audio;
use crate::components::{
    Collider, ColliderSource, Dead, GameSpeed, GameState, PlayerDied, Velocity,
};
use crate::player::PlayerBulletCount;
use crate::resolution;
use crate::score::AlienKilled;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_collisions,).run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_collisions(
    mut commands: Commands,
    mut player_died_events: EventWriter<PlayerDied>,
    mut alien_killed_events: EventWriter<AlienKilled>,
    projectile_query: Query<(Entity, &Transform, &Collider), (With<Velocity>, Without<Dead>)>,
    collider_query: Query<
        (Entity, &Transform, &Collider),
        (With<Collider>, Without<Velocity>, Without<Dead>),
    >,
    resolution: Res<resolution::Resolution>,
    mut game_speed: ResMut<GameSpeed>,
    game_assets: Res<GameAssets>,
    mut bullet_count: ResMut<PlayerBulletCount>,
) {
    for (projectile_entity, projectile_transform, projectile_collider) in projectile_query.iter() {
        if projectile_transform.translation.y.abs() > resolution.screen_dimensions.y * 0.5 {
            commands.entity(projectile_entity).insert(Dead);
            if projectile_collider.source == ColliderSource::PlayerBullet {
                bullet_count.count -= 1;
            }
            continue;
        }

        for (collider_entity, collider_transform, collider_collider) in collider_query.iter() {
            if projectile_collider.source == ColliderSource::PlayerBullet
                && collider_collider.source == ColliderSource::Player
            {
                continue;
            }
            if projectile_collider.source == ColliderSource::AlienBullet
                && collider_collider.source == ColliderSource::Alien
            {
                continue;
            }
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y,
            );
            let collider_pos = Vec2::new(
                collider_transform.translation.x,
                collider_transform.translation.y,
            );
            if Vec2::distance(projectile_pos, collider_pos)
                < 0.5 * (projectile_collider.radius + collider_collider.radius)
            {
                commands.entity(projectile_entity).insert(Dead);
                if projectile_collider.source == ColliderSource::PlayerBullet {
                    bullet_count.count -= 1;
                }
                if collider_collider.source == ColliderSource::Player {
                    player_died_events.write(PlayerDied);
                } else {
                    commands.entity(collider_entity).insert(Dead);
                    if projectile_collider.source == ColliderSource::PlayerBullet
                        && collider_collider.source == ColliderSource::Alien
                    {
                        alien_killed_events.write(AlienKilled);
                        game_speed.value += 0.025;
                        audio::play(&mut commands, game_assets.alien_killed_sfx.clone());
                    }
                }
                break;
            }
        }
    }
}
