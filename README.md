# 🚀 Rover - A Planetary Exploration Game

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A physics-based 2D sidescroller game built in Rust where you pilot a rover across treacherous alien terrain. Navigate obstacles, manage your fuel, and see how far you can travel on this mysterious planet!

![Game Status](https://img.shields.io/badge/status-playable-green)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

## 🎮 Gameplay

Take control of a small rover on a distant planet's surface. The rover is affected by gravity and requires careful thruster management to navigate:

- **Physics-based movement** - Realistic gravity simulation pulls your rover down
- **Fuel management** - Limited fuel adds strategic depth to your exploration
- **Procedural terrain** - Every playthrough features unique terrain challenges
- **Obstacle avoidance** - Dodge randomly placed rocks and hazards
- **Distance tracking** - Compete for the longest distance traveled

## 🕹️ Controls

| Key | Action |
|-----|--------|
| **↑** / **W** | Main thruster (upward) |
| **←** / **A** | Left thruster |
| **→** / **D** | Right thruster |
| **R** | Restart (when game over) |

## 🚦 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/Tortured-Metaphor/Rover.git
cd Rover
```

2. Build the project:
```bash
cargo build --release
```

3. Run the game:
```bash
cargo run --release
```

## 🎯 Game Objectives

1. **Survive** - Avoid crashing into obstacles
2. **Explore** - Travel as far as possible across the alien landscape
3. **Conserve** - Manage your fuel efficiently for maximum distance
4. **Master** - Learn to control the rover's physics for precise navigation

## 🏗️ Technical Details

### Built With

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[ggez](https://ggez.rs/)** - Rust game framework for 2D graphics
- **[glam](https://github.com/bitshifter/glam-rs)** - Fast linear algebra library

### Architecture

The game is structured into modular components:

- `main.rs` - Game loop and state management
- `rover.rs` - Rover physics, controls, and rendering
- `terrain.rs` - Procedural terrain generation and obstacle placement
- `physics.rs` - Physics constants and configurations

### Key Features

- **Smooth side-scrolling camera** that follows the rover
- **Visual thrust indicators** showing active thrusters
- **Collision detection** for ground and obstacles
- **Procedural generation** using seeded random for consistent terrain
- **Real-time physics simulation** at 60 FPS

## 🎨 Visual Design

- **Rover**: Orange body with blue cockpit window
- **Thrusters**: Dynamic orange/yellow flames when active
- **Terrain**: Naturalistic brown ground with elevation changes
- **Obstacles**: Dark rocky formations scattered across the landscape
- **Sky**: Deep space atmosphere with dark blue background

## 🔧 Configuration

The game's physics can be adjusted in `src/physics.rs`:

```rust
pub const GRAVITY: f32 = 200.0;         // Gravitational force
pub const THRUST_POWER: f32 = 400.0;    // Thruster strength
pub const MAX_FUEL: f32 = 100.0;        // Starting fuel amount
pub const FUEL_CONSUMPTION_RATE: f32 = 10.0; // Fuel usage per second
```

## 📈 Performance

- Lightweight and efficient
- Runs smoothly on most hardware
- Minimal CPU and memory usage
- Native performance through Rust

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Ideas for Contributions

- [ ] Add power-ups and fuel pickups
- [ ] Implement different rover types
- [ ] Create multiple planets/environments
- [ ] Add multiplayer support
- [ ] Implement a level editor
- [ ] Add sound effects and music
- [ ] Create achievement system
- [ ] Add particle effects

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for excellent documentation and support
- The ggez team for creating an approachable game framework
- Inspired by classic games like Lunar Lander and Hill Climb Racing

## 📊 Project Status

The game is currently in **v0.1.0** and is fully playable. Future updates will include:
- Enhanced graphics and animations
- Additional gameplay mechanics
- Sound and music
- Leaderboard system
- More diverse terrain types

---

**Ready to explore?** Run `cargo run` and start your planetary adventure! 🌍🚀