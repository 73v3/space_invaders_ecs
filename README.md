# Space Invaders ECS
A Space Invaders game built with Bevy 0.16 and ECS in Rust, created as an educational exercise to demonstrate 2D game development with Bevy.

<div align="center">
  <img src="assets/screenshot.png" alt="Screenshot of Space Invaders ECS" width="50%"/>
  <p>Screenshot of the Space Invaders ECS game in action.</p>
</div>

WARNING: This is my first Rust project, created after 7 days of learning Rust and Bevy. The code is not a model of best practices but serves as a learning resource for others interested in Bevy and ECS.

This project was inspired by [Biped-Potato’s Bevy Space Invaders](https://github.com/Biped-Potato/bevy_space_invaders), used as a starting point and significantly expanded.

## Code Orientation

Copy [collated_src.txt](assets/collated_src.txt) into your favourite AI and discuss the code with it. For example, Grok identified the following weaknesses in the ECS design:

## Weaknesses

Overall I'm not entirely happy with the project organisation. I've come from years of object-orientation, and moving to ECS, I'm still not sure where everything belongs.

>> Collision Handling:

Brute-force nested queries in update_collisions are simple but inefficient.

Detection is proximity based rather than via bounding boxes, which has its accuracy limitations.

Detection could potentially be centralised to its own collisions plugin.

>> Event and State Management:

Events are underused.

Game speed increments are scattered (e.g., on alien kill, shift down, wave clear). A centralized speed-adjust system reading events would consolidate this.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Third-Party Assets

### Fonts
This project uses the space_invaders.ttf font by chriswal1200, licensed under the SIL Open Font License, Version 1.1. See `assets/fonts/space_invaders/license.txt` for license details.

### Sfx

Sound effects are not included in this repository due to unclear licensing. To run the game with sound, obtain the following files from a licensed source and place them in the assets/sfx folder.

See:

https://ronjeffries.com/articles/020-invaders-30ff/i-37/ (see Invaders.zip linked at very bottom of page)

or

https://www.classicgaming.cc/classics/space-invaders/sounds (download sounds individually)

### Palette

The gilt-8 palette by tomicit0 is used for colors.

https://lospec.com/palette-list/gilt-8

## Disclaimer

This project is for educational purposes only. The code and included assets are not intended for commercial use or public distribution outside of learning environments. Users must ensure compliance with the licenses of all third-party assets before using or distributing this project. The creator is not responsible for any misuse of the project or its assets.

If you plan to use this project beyond personal learning, replace all third-party assets with properly licensed alternatives and ensure compliance with the MIT License for the code.

## Installation

- Ensure [Rust](https://www.rust-lang.org/learn/get-started) and [Bevy 0.16](https://bevy.org/learn/quick-start/getting-started) dependencies are installed.
- Clone this repository.
- Place sound effect files in assets/sfx or use alternatives.
- Run the game with cargo run.

## Acknowledgments

Inspired by the classic Space Invaders game by Taito.
