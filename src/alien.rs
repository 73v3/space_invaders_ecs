use crate::assets::GameAssets;
use crate::components::{Collider, ColliderSource, Dead, GameEntity, GameState, Velocity};
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
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Alien {
    pub original_position: Vec3,
}

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
    pub reset: bool,
    pub fire_timer: f32,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.0;
const ALIEN_SHIFT_AMOUNT: f32 = 12.;
const BULLET_SPEED: f32 = -200.; // Negative for downward movement
const FIRE_INTERVAL: f32 = 2.0; // Base interval between firing checks
const FIRE_PROBABILITY: f32 = 0.05; // 5% chance per alien per check

fn spawn_aliens(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    //asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,
        fire_timer: 0.,
    });
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * (1.5 * SPACING), 0.)
                - (Vec3::X * WIDTH as f32 * SPACING * 0.5)
                - (Vec3::Y * HEIGHT as f32 * (3.5 * SPACING) * 0.5)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);
            commands.spawn((
                Sprite {
                    image: game_assets.alien_texture.clone(),
                    ..Default::default()
                },
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                Alien {
                    original_position: position,
                },
                Collider {
                    radius: 24.,
                    source: ColliderSource::Alien,
                },
                GameEntity,
            ));
        }
    }
}

fn advance_aliens_horizontally(
    mut alien_query: Query<&mut Transform, (With<Alien>, Without<Dead>)>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    for mut transform in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
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

fn adjust_alien_formation(
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.0;
        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }

    if alien_manager.reset {
        alien_manager.reset = false;
        alien_manager.direction = 1.0;
        for (_entity, alien, mut transform) in alien_query.iter_mut() {
            transform.translation = alien.original_position;
        }
    }
}

fn fire_alien_bullets(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut alien_manager: ResMut<AlienManager>,
    alien_query: Query<&Transform, (With<Alien>, Without<Dead>)>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
    mut rng: GlobalEntropy<WyRand>,
) {
    alien_manager.fire_timer -= time.delta_secs();
    if alien_manager.fire_timer <= 0. {
        alien_manager.fire_timer = FIRE_INTERVAL;
        for transform in alien_query.iter() {
            //if (rng.next_u32() as f32) / (u32::MAX as f32) < FIRE_PROBABILITY {
            if random_float(&mut rng) < FIRE_PROBABILITY {
                commands.spawn((
                    Sprite {
                        image: game_assets.bullet_texture.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(transform.translation)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    Velocity {
                        velocity: Vec2::new(0., BULLET_SPEED),
                    },
                    Collider {
                        radius: 24.,
                        source: ColliderSource::AlienBullet,
                    },
                    GameEntity,
                ));
            }
        }
    }
}
