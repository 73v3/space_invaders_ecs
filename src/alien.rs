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
                    check_side_bounds,
                    fire_alien_bullets,
                    animate_aliens,
                    handle_wave_reset,
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

#[derive(Component)]
pub struct FormationDirection(pub f32);

#[derive(Resource)]
pub struct AlienFireTimer(pub f32);

const WIDTH: i32 = 12;
const HEIGHT: i32 = 5;
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
                + (Vec3::Y * resolution.screen_dimensions.y * 0.4);
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
                FormationDirection(1.0),
                GameEntity,
            ));
        }
    }
}

// called at the start of each new game
// spawns a grid of aliens
fn spawn_aliens(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienFireTimer(0.));
    spawn_alien_grid(&mut commands, &game_assets, &resolution);
}

pub fn advance_aliens_horizontally(
    mut alien_query: Query<(&mut Transform, &FormationDirection), (With<Alien>, Without<Dead>)>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
) {
    // Grab direction from the alien entity.
    // Explanation:
    //      The map method transforms the Option returned by next().
    //      If next() returns Some((transform, formation_direction)), the closure |(_, d)| d.0 is applied:
    //      The closure uses pattern matching to destructure the tuple (transform, formation_direction).
    //      The underscore (_) ignores the Transform component (since we only care about the direction),
    //      and d binds to the FormationDirection reference.
    if let Some(dir) = alien_query.iter().next().map(|(_, d)| d.0) {
        for (mut transform, _) in alien_query.iter_mut() {
            transform.translation.x +=
                time.delta_secs() * dir * ALIEN_SPEED * (30.0 * game_speed.value);
        }
    }
}

// if any aliens hits the side border, drop all aliens down a row, increase speed and reverse their direction
pub fn check_side_bounds(
    mut alien_query: Query<(&mut Transform, &mut FormationDirection), (With<Alien>, Without<Dead>)>,
    mut game_speed: ResMut<GameSpeed>,
    resolution: Res<resolution::Resolution>,
) {
    if alien_query.is_empty() {
        return;
    }

    // The iter().next() method fetches the first alien entity element, and unwrap() grabs its values.
    // The tuple's second element (.1) is the FormationDirection component, and .0 extracts its inner f32 value, representing the direction (1.0 for right, -1.0 for left).
    let dir = alien_query.iter().next().unwrap().1 .0;

    // find min and max x positions of the aliens
    let (mut min_x, mut max_x) = (f32::MAX, f32::MIN);
    for (transform, _) in alien_query.iter() {
        let x = transform.translation.x;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
    }

    // grab our edge and bound values from screen dimensions and alien min/max x positions
    let left_bound = -resolution.screen_dimensions.x * 0.5 * 0.95;
    let right_bound = resolution.screen_dimensions.x * 0.5 * 0.95;

    let (edge, bound) = if dir > 0. {
        (max_x, right_bound)
    } else {
        (min_x, left_bound)
    };

    // if an alien has stepped over an edge
    if (edge - bound) * dir > 0. {
        let overstep = edge - bound;
        let new_dir = -dir;
        game_speed.value += GAME_SPEED_INCREMENT;

        // drop a row and flip movement direction of all aliens
        for (mut transform, mut formation_dir) in alien_query.iter_mut() {
            transform.translation.x -= overstep;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
            formation_dir.0 = new_dir;
        }
    }
}

fn handle_wave_reset(
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform), (With<Alien>, Without<Dead>)>,
    mut clear_count: ResMut<ClearCount>,
    mut game_speed: ResMut<GameSpeed>,
    game_assets: Res<GameAssets>,
    resolution: Res<resolution::Resolution>,
) {
    let is_empty = alien_query.is_empty();

    // find the smallest y-coordinate among all non-dead aliens, representing the lowest point of the alien formation
    let min_y = if is_empty {
        f32::MAX
    } else {
        alien_query
            .iter()
            .map(|(_, t)| t.translation.y) // Transforms each tuple into the y-coordinate (f32) of the alien’s Transform
            .fold(f32::MAX, f32::min) // Reduces the iterator of y-coordinates to a single value by finding the minimum.
    };

    // has_landed = aliens have hit bottom of screen
    let has_landed = !is_empty && min_y < -resolution.screen_dimensions.y * 0.5;

    if has_landed || is_empty {
        // In practice, the aliens should hit the player before they hit the bottom of the screen, resulting in game over
        if has_landed {
            for (entity, _) in alien_query.iter() {
                commands.entity(entity).despawn();
            }
        }

        if is_empty {
            clear_count.count += 1;
            game_speed.value = 1.0 + (clear_count.count as f32) / 2.0;
            info!(
                "Wave {} cleared! Game Speed {:?}",
                clear_count.count, game_speed.value
            )
        }

        spawn_alien_grid(&mut commands, &game_assets, &resolution);
    }
}

fn fire_alien_bullets(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut alien_fire_timer: ResMut<AlienFireTimer>,
    alien_query: Query<&Transform, (With<Alien>, Without<Dead>)>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut rng: GlobalEntropy<WyRand>,
) {
    alien_fire_timer.0 -= time.delta_secs() * game_speed.value;
    if alien_fire_timer.0 <= 0. {
        alien_fire_timer.0 = FIRE_INTERVAL;
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

const MAX_ANIMATION_SPEED: f32 = 4.0;

fn animate_aliens(
    mut alien_query: Query<(&mut Alien, &mut Sprite), Without<Dead>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut commands: Commands,
) {
    let mut should_play_sound = false;
    let mut new_frame = false;
    let anim_speed = f32::min(MAX_ANIMATION_SPEED, game_speed.value); // sanity check on max animation speed

    // Check if any alien needs to switch frames (all aliens animate synchronously)
    if let Some((alien, _)) = alien_query.iter_mut().next() {
        let animation_timer = alien.animation_timer + time.delta_secs() * anim_speed;
        if animation_timer >= ANIMATION_BASE_SPEED / anim_speed {
            should_play_sound = true;
            new_frame = !alien.current_frame;
        }
    }

    // Update all aliens' animation state and sprite
    for (mut alien, mut sprite) in alien_query.iter_mut() {
        alien.animation_timer += time.delta_secs() * anim_speed;
        if alien.animation_timer >= ANIMATION_BASE_SPEED / anim_speed {
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
    }
}
