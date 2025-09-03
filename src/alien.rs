use crate::assets::GameAssets;
use crate::audio;
use crate::components::{
    ClearCount, Collider, ColliderSource, Dead, GameEntity, GameSpeed, GameState, Velocity,
};
use crate::random::random_float;
use crate::resolution;
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        // assets must be loaded before aliens can spawn
        app.add_systems(OnEnter(GameState::Playing), spawn_aliens)
            .add_systems(
                Update,
                (
                    advance_aliens_horizontally,
                    adjust_alien_formation,
                    fire_alien_bullets,
                    animate_aliens,
                    check_all_aliens_dead,
                    test_wave_clear,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Alien {
    pub original_position: Vec3,
    pub animation_timer: f32,
    pub current_frame: bool, // false for alien_a, true for alien_b
}

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
    pub reset: bool,
    pub fire_timer: f32,
}

const WIDTH: i32 = 16;
const HEIGHT: i32 = 6;
const SPACING: f32 = 24.;
const ALIEN_SPEED: f32 = 1.0;
const ALIEN_SHIFT_AMOUNT: f32 = 12.;
const BULLET_SPEED: f32 = -200.; // Negative for downward movement
const FIRE_INTERVAL: f32 = 0.1; // Base interval between firing checks
const FIRE_PROBABILITY: f32 = 0.001; // % chance per alien per check
const ANIMATION_BASE_SPEED: f32 = 1.0; // Seconds per frame at game_speed = 1.0
const GAME_SPEED_INCREMENT: f32 = 0.1; // Increment game speed when aliens shift down

fn spawn_alien_grid(
    commands: &mut Commands,
    game_assets: &GameAssets,
    resolution: &resolution::Resolution,
) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * (1.5 * SPACING), 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * (3.5 * SPACING) * 0.5)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);
            commands.spawn((
                Sprite {
                    image: game_assets.alien_texture_a.clone(),
                    color: game_assets.palette.colors[2],
                    ..Default::default()
                },
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                Alien {
                    original_position: position,
                    animation_timer: 0.0,
                    current_frame: false,
                },
                Collider {
                    radius: 16.,
                    source: ColliderSource::Alien,
                },
                GameEntity,
            ));
        }
    }
}

fn spawn_aliens(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,
        fire_timer: 0.,
    });
    spawn_alien_grid(&mut commands, &game_assets, &resolution);
}

pub fn advance_aliens_horizontally(
    mut alien_query: Query<&mut Transform, (With<Alien>, Without<Dead>)>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
) {
    for mut transform in alien_query.iter_mut() {
        transform.translation.x +=
            time.delta_secs() * alien_manager.direction * ALIEN_SPEED * (30.0 * game_speed.value);
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary =
                resolution.screen_dimensions.x * alien_manager.direction * 0.5
                    - transform.translation.x;
        }
        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 {
            alien_manager.reset = true;
        }
    }
}

pub fn adjust_alien_formation(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
    mut game_speed: ResMut<GameSpeed>,
    game_assets: Res<GameAssets>,
    resolution: Res<resolution::Resolution>,
) {
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.0;
        game_speed.value += GAME_SPEED_INCREMENT;
        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }

    if alien_manager.reset {
        alien_manager.reset = false;
        alien_manager.direction = 1.0;
        // Despawn all existing aliens
        for (entity, _, _) in alien_query.iter_mut() {
            commands.entity(entity).despawn();
        }
        // Respawn a new wave of aliens
        spawn_alien_grid(&mut commands, &game_assets, &resolution);
    }
}

fn fire_alien_bullets(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut alien_manager: ResMut<AlienManager>,
    alien_query: Query<&Transform, (With<Alien>, Without<Dead>)>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut rng: GlobalEntropy<WyRand>,
) {
    alien_manager.fire_timer -= time.delta_secs() * game_speed.value;
    if alien_manager.fire_timer <= 0. {
        alien_manager.fire_timer = FIRE_INTERVAL;
        for transform in alien_query.iter() {
            if random_float(&mut rng) < FIRE_PROBABILITY * game_speed.value {
                commands.spawn((
                    Sprite {
                        image: game_assets.bullet_texture.clone(),
                        color: game_assets.palette.colors[3],
                        ..Default::default()
                    },
                    Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    Velocity {
                        velocity: Vec2::new(0., BULLET_SPEED),
                    },
                    Collider {
                        radius: 16.,
                        source: ColliderSource::AlienBullet,
                    },
                    GameEntity,
                ));
            }
        }
    }
}

fn animate_aliens(
    mut alien_query: Query<(&mut Alien, &mut Sprite), Without<Dead>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut commands: Commands,
) {
    let mut should_play_sound = false;
    let mut new_frame = false;

    // Check if any alien needs to switch frames (all aliens animate synchronously)
    if let Some((alien, _)) = alien_query.iter_mut().next() {
        let animation_timer = alien.animation_timer + time.delta_secs() * game_speed.value;
        if animation_timer >= ANIMATION_BASE_SPEED / game_speed.value {
            should_play_sound = true;
            new_frame = !alien.current_frame;
        }
    }

    // Update all aliens' animation state and sprite
    for (mut alien, mut sprite) in alien_query.iter_mut() {
        alien.animation_timer += time.delta_secs() * game_speed.value;
        if alien.animation_timer >= ANIMATION_BASE_SPEED / game_speed.value {
            alien.animation_timer = 0.0;
            alien.current_frame = new_frame;
            sprite.image = if new_frame {
                game_assets.alien_texture_b.clone()
            } else {
                game_assets.alien_texture_a.clone()
            };
        }
    }

    // Play sound once if frame changed
    if should_play_sound {
        let sound = if new_frame {
            game_assets.invader_move_1_sfx.clone()
        } else {
            game_assets.invader_move_2_sfx.clone()
        };
        audio::play_with_volume(&mut commands, sound, 0.5);
        info!("Game Speed {:?}", game_speed.value)
    }
}

fn check_all_aliens_dead(
    alien_query: Query<Entity, (With<Alien>, Without<Dead>)>,
    mut alien_manager: ResMut<AlienManager>,
    mut game_speed: ResMut<GameSpeed>,
    mut clear_count: ResMut<ClearCount>,
) {
    if alien_query.is_empty() {
        alien_manager.reset = true;
        clear_count.count += 1;
        game_speed.value = 1.0 + (clear_count.count as f32) / 2.0;
        info!("Wave cleared! Game Speed {:?}", game_speed.value)
    }
}

fn test_wave_clear(
    mut commands: Commands,
    alien_query: Query<Entity, With<Alien>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::End) {
        for entity in alien_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
