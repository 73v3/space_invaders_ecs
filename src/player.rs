use crate::assets::GameAssets;
use crate::audio;
use crate::components::{Collider, ColliderSource, Dead, GameEntity, GameState, Velocity};
use crate::resolution;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(Update, update_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player {
    pub shoot_timer: f32,
}

#[derive(Resource)]
pub struct PlayerBulletCount {
    pub count: u32,
}

fn setup_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(PlayerBulletCount { count: 0 });
    commands.spawn((
        Sprite {
            image: game_assets.player_texture.clone(),
            color: game_assets.palette.colors[4],
            ..Default::default()
        },
        Transform::from_xyz(
            0.,
            -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0),
            0.,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player { shoot_timer: 2. },
        Collider {
            radius: 9.,
            source: ColliderSource::Player,
        },
        GameEntity,
    ));
}

const SPEED: f32 = 200.;
const BULLET_SPEED: f32 = 400.;
const SHOOT_COOLDOWN: f32 = 0.25;

fn update_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Dead>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
    mut bullet_count: ResMut<PlayerBulletCount>,
) {
    if let Ok((mut player, mut transform)) = player_query.single_mut() {
        let mut horizontal = 0.;

        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            horizontal += -1.;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            horizontal += 1.;
        }

        transform.translation.x += horizontal * time.delta_secs() * SPEED;

        let left_bound = -resolution.screen_dimensions.x * 0.5;
        let right_bound = resolution.screen_dimensions.x * 0.5;

        if transform.translation.x > right_bound {
            transform.translation.x = right_bound;
        }
        if transform.translation.x < left_bound {
            transform.translation.x = left_bound;
        }

        player.shoot_timer -= time.delta_secs();

        if bullet_count.count == 0 {
            player.shoot_timer = 0.;
        }

        if keys.just_pressed(KeyCode::Space) && player.shoot_timer <= 0. && bullet_count.count < 2 {
            player.shoot_timer = SHOOT_COOLDOWN;

            audio::play(&mut commands, game_assets.shoot_sfx.clone());

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
                    radius: 12.,
                    source: ColliderSource::PlayerBullet,
                },
                GameEntity,
            ));

            bullet_count.count += 1;
        }
    }
}
