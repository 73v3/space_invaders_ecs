use crate::alien::Alien;
use crate::assets::GameAssets;
use crate::audio;
use crate::components::{Dead, GameEntity, GameSpeed, GameState, PlayerDied};
use crate::player::Player;
use crate::random::{random_colour, random_float};
use crate::resolution::Resolution;
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDied>();
        app.add_systems(
            Update,
            (
                spawn_alien_explosions,
                spawn_player_explosions,
                update_explosions,
                check_player_explosions,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Explosion {
    pub timer: f32,
}

#[derive(Component)]
pub struct PlayerExplosion;

const EXPLOSION_LIFETIME: f32 = 0.375;

// spawns an explosion at the position of any alien that has just died
fn spawn_alien_explosions(
    mut commands: Commands,
    dead_aliens: Query<&Transform, (With<Alien>, Added<Dead>)>,
    game_assets: Res<GameAssets>,
    resolution: Res<Resolution>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for transform in dead_aliens.iter() {
        commands.spawn((
            Sprite {
                image: game_assets.bright_explosion_texture.clone(),
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

const NUM_PLAYER_EXPLOSIONS: i32 = 16;

// hides the player ship
// and spawns multiple explosions in its place
fn spawn_player_explosions(
    mut commands: Commands,
    mut player_died_events: EventReader<PlayerDied>,
    mut player_query: Query<(Entity, &Transform, &mut Visibility), With<Player>>,
    game_assets: Res<GameAssets>,
    resolution: Res<Resolution>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for _ in player_died_events.read() {
        if let Ok((entity, transform, mut visibility)) = player_query.single_mut() {
            info!("player died");
            *visibility = Visibility::Hidden; // hide the player ship whilst the explosions process
            commands.entity(entity).insert(Dead);
            audio::play_with_volume(&mut commands, game_assets.player_explosion_sfx.clone(), 0.5);
            for _ in 0..NUM_PLAYER_EXPLOSIONS {
                let offset_x = (random_float(&mut rng) - 0.5) * 20.0;
                let offset_y = (random_float(&mut rng) - 0.5) * 20.0;
                commands.spawn((
                    Sprite {
                        image: game_assets.explosion_texture.clone(),
                        color: random_colour(&mut rng, &game_assets),
                        ..Default::default()
                    },
                    Transform::from_translation(
                        transform.translation + Vec3::new(offset_x, offset_y, 0.),
                    )
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    Explosion {
                        timer: -2. * random_float(&mut rng), // stagger the explosion dissipation over time
                    },
                    PlayerExplosion,
                    GameEntity,
                ));
            }
        }
    }
}

// fades out explosions over time, despawning when done
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

// checks if the player is dead and player explosions have finished,
// in which case, return to title screen
fn check_player_explosions(
    player_explosion_query: Query<Entity, With<PlayerExplosion>>,
    player_query: Query<&Player, With<Dead>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_speed: ResMut<GameSpeed>,
) {
    if !player_query.is_empty() && player_explosion_query.is_empty() {
        next_state.set(GameState::Title);
        game_speed.value = 1.0;
        info!("player dead::switching to title");
    }
}
