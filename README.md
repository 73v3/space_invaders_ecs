# Space Invaders ECS
A Space Invaders game built with Bevy 0.16 and ECS in Rust.

WARNING: This is my very first Rust project. I know NOTHING! Do not mistake the atrocities within for best practice.

At the time of writing, I have been learning Rust and Bevy for a grand total of 7 days.

This was originally a fork of:

https://github.com/Biped-Potato/bevy_space_invaders/tree/master

But I never actually forked. I just used it as a starter and then expanded on it and rearranged a bunch of stuff.

I share this an educational exercise as it will hopefully end up covering a lot of basic ground common to creating a 2d game in Bevy.

## Code Orientation

Copy collated_src.txt into your favourite AI and discuss the code with it. For example, Grok identified the following weaknesses in the ECS design:

## Weaknesses

>> Centralization in Resources and Systems:

AlienManager resource acts like a "manager class," holding formation state (direction, shift flags, boundary distance). Systems like advance_aliens_horizontally derive state from entities but store it back in the resource. This violates pure ECS by introducing a singleton-like coordinator.

Critique: In stricter ECS, formation state should emerge from entity data (e.g., query all aliens to compute min/max X for boundaries). The current approach works but reduces parallelism (systems chain on the resource) and makes debugging harder (state isn't localized to entities).

Some systems are multi-responsibility: adjust_alien_formation handles shifting, direction reversal, speed increment, and full wave reset (despawn/respawn). This could be split (e.g., separate shift_formation and reset_wave systems) for better maintainability.

Animation in animate_aliens syncs all aliens by checking one and applying to all—a pragmatic hack for synchronization, but it assumes uniform state. If aliens had individual animations, this would break.

>> Collision Handling:

Brute-force nested queries in update_collisions are simple but inefficient for growth. No spatial partitioning (e.g., quadtree via resource or component).

Critique: While fine here, it doesn't scale ECS-style—larger games need systems that batch collisions (e.g., via Bevy's rapier plugin). Also, it skips certain checks (e.g., player bullets vs. player) with if-guards, which could be query filters instead.

Collision resolution is imperative: Inserts Dead, adjusts bullet counts, sends events in a loop. This mixes detection and response; separating into detection (gather pairs) and resolution systems would be more ECS-pure.

>> Component Granularity and Data Duplication:

Alien stores original_position, but aliens move collectively—why per-entity? Could derive from a resource or formation component.

Sprite and Transform are on every entity, which is fine, but color palettes are duplicated (e.g., game_assets.palette.colors[2] hardcoded). A PaletteColor component referencing an index could reduce repetition.

Player has shoot_timer, but bullet limiting uses a separate PlayerBulletCount resource. This splits related state—could be a Player field or a query on active bullets.

>> Event and State Management:

Events are underused: Wave clearing relies on check_all_aliens_dead querying for empty aliens, setting a reset flag in AlienManager. An event (e.g., WaveCleared) would decouple this from the manager.

Game speed increments are scattered (e.g., on alien kill, shift down, wave clear). A centralized speed-adjust system reading events would consolidate this.
Testing hacks like test_wave_clear (despawn on End key) pollute production code—better as a debug plugin.

## Fonts
This project uses the space_invaders.ttf font by chriswal1200, licensed under the SIL Open Font License, Version 1.1. See `assets/fonts/space_invaders/license.txt` for license details.

## Sfx
https://ronjeffries.com/articles/020-invaders-30ff/i-37/ (see download zip for sfx files to be placed in assets/sfx folder)

or

https://www.classicgaming.cc/classics/space-invaders/sounds

## Palette

https://lospec.com/palette-list/gilt-8

## Disclaimer

This project is for educational purposes only. The assets and code are not intended for commercial use or public distribution outside of learning environments. The creator is not responsible for any misuse of the project or its assets. If you plan to use this project beyond personal learning, ensure all assets are replaced with properly licensed materials.

## Acknowledgments

Inspired by the classic Space Invaders game by Taito.
