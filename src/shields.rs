use crate::alien::Alien;
use crate::assets::GameAssets;
use crate::components::{Collider, ColliderSource, Dead, GameEntity, GameState};
use crate::resolution::Resolution;
use bevy::prelude::*;

pub struct ShieldsPlugin;

impl Plugin for ShieldsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_shields)
            .add_systems(
                Update,
                check_alien_shield_collisions
                    .after(crate::alien::advance_aliens_horizontally)
                    .after(crate::alien::adjust_alien_formation)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct ShieldUnit;

const NUM_SHIELDS: usize = 4;
const SHIELD_GRID_WIDTH: i32 = 5;
const SHIELD_GRID_HEIGHT: i32 = 3;
const UNIT_PIXEL_SIZE: f32 = 8.0;

fn spawn_shields(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    resolution: Res<Resolution>,
) {
    let unit_spacing = UNIT_PIXEL_SIZE * resolution.pixel_ratio;
    let shield_radius = (UNIT_PIXEL_SIZE / 2.0) * resolution.pixel_ratio / 4.;
    let shield_y = -resolution.screen_dimensions.y * 0.4;
    let shield_spacing_x = resolution.screen_dimensions.x / NUM_SHIELDS as f32;

    for i in 0..NUM_SHIELDS {
        let shield_x = (i as f32 - (NUM_SHIELDS as f32 - 1.0) / 2.0) * shield_spacing_x;

        for gx in 0..SHIELD_GRID_WIDTH {
            let unit_x =
                shield_x + (gx as f32 - (SHIELD_GRID_WIDTH as f32 - 1.0) / 2.0) * unit_spacing;

            for gy in 0..SHIELD_GRID_HEIGHT {
                let unit_y =
                    shield_y + (gy as f32 - (SHIELD_GRID_HEIGHT as f32 - 1.0) / 2.0) * unit_spacing;

                commands.spawn((
                    Sprite {
                        image: game_assets.shield_texture.clone(),
                        color: game_assets.palette.colors[4],
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(unit_x, unit_y, 0.))
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    ShieldUnit,
                    Collider {
                        radius: shield_radius,
                        source: ColliderSource::Shield,
                    },
                    GameEntity,
                ));
            }
        }
    }
}

fn check_alien_shield_collisions(
    mut commands: Commands,
    alien_query: Query<(Entity, &Transform, &Collider), (With<Alien>, Without<Dead>)>,
    shield_query: Query<(Entity, &Transform, &Collider), (With<ShieldUnit>, Without<Dead>)>,
) {
    for (_alien_entity, alien_transform, alien_collider) in alien_query.iter() {
        for (shield_entity, shield_transform, shield_collider) in shield_query.iter() {
            let distance = (alien_transform.translation - shield_transform.translation).length();
            if distance < alien_collider.radius + shield_collider.radius {
                commands.entity(shield_entity).insert(Dead);
            }
        }
    }
}
