// explosion.rs
use bevy::prelude::*;

use crate::alien::Alien;
use crate::assets::GameAssets;
use crate::components::{Dead, GameEntity, GameState};
use crate::random::random_colour;
use crate::resolution::Resolution;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_explosions, update_explosions)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Explosion {
    pub timer: f32,
}

const EXPLOSION_LIFETIME: f32 = 0.375;

fn spawn_explosions(
    mut commands: Commands,
    dead_aliens: Query<&Transform, (With<Alien>, Added<Dead>)>,
    game_assets: Res<GameAssets>,
    resolution: Res<Resolution>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for transform in dead_aliens.iter() {
        commands.spawn((
            Sprite {
                image: game_assets.explosion_texture.clone(),
                color: random_colour(&mut rng, &game_assets),
                ..Default::default()
            },
            Transform::from_translation(transform.translation)
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
            Explosion { timer: 0.0 },
            GameEntity,
        ));
    }
}

fn update_explosions(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Explosion, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut explosion, mut sprite) in query.iter_mut() {
        explosion.timer += time.delta_secs();
        if explosion.timer > EXPLOSION_LIFETIME {
            commands.entity(entity).despawn();
        } else {
            let alpha = if explosion.timer < EXPLOSION_LIFETIME / 2.0 {
                1.0
            } else {
                1.0 - (explosion.timer - EXPLOSION_LIFETIME / 2.0) / (EXPLOSION_LIFETIME / 2.0)
            };
            sprite.color = sprite.color.with_alpha(alpha);
        }
    }
}
