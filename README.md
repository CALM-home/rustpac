# 🚀 RustPac

> **A Jetpac clone in Rust — recreated for the joy of coding and nostalgia.**

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Macroquad](https://img.shields.io/badge/macroquad-2D%20engine-orange?style=for-the-badge)

---

## 🎮 What is this?

**RustPac** is a recreation of the cult classic **Jetpac** (Ultimate Play the Game, 1983 on ZX Spectrum) in pure Rust.

Why? For the pleasure of coding a retro game with modern technologies, and because Jetpac is a perfect example of *gameplay over graphics* — a few sprites, simple physics, and hours of fun!

---

## 👥 Development Team

This project is developed collaboratively:

- **Titi** — The human developer, streamer of CALM (Comme A La Maison), Rust enthusiast
- **Shuri** — Wakandan AI incarnation running on OpenClaw with the **Kimi K2.5** model 🤖🖤

Together, we code, debug, and iterate through each feature. The AI assistant handles the implementation details while Titi guides the vision, tests the gameplay, and keeps the project fun!

> *"Wakanda forever!"* — Shuri

---

## 🎯 Goals

- **Mechanical fidelity**: reproduce the "floaty" physics and inertia of the original Jetpac
- **Idiomatic Rust**: no heavy frameworks, just Macroquad (minimalist 2D)
- **Step-by-step progression**: each feature is a playable victory
- **Clean, documented code**: to learn and share

---

## 📋 Roadmap

### ✅ STEP 0 — Setup
- [x] Functional Rust + Macroquad project
- [x] Window opens without crashing

### ✅ STEP 1 — Player
- [x] Controllable astronaut (left/right + thrust)
- [x] Physics with inertia and soft gravity
- [x] Screen boundaries
- [x] Flame effect during thrust

### ✅ STEP 2 — Platform
- [x] Landing zone on the ground
- [x] Landing possible (LANDED/FLYING state)

### ✅ STEP 3 — Rocket Assembly
- [x] 3 modules to collect: Bottom → Middle → Top
- [x] Automatic pickup by flying over
- [x] **Physical placement**: module detaches and falls with gravity
- [x] Progressive horizontal alignment during fall
- [x] Strict order enforced (no cheating!)

### ✅ STEP 4 — Fuel ⛽
- [x] Fuel capsule system
- [x] Sequential spawn: 1 capsule at a time, next appears after delivery
- [x] 3 capsules × 33.33% = 100%
- [x] Same falling mechanics as modules
- [x] UI gauge (fuel + capsule counter)

### ✅ STEP 5 — Launch and Levels
- [x] Launch animation when fuel is full
- [x] Player must board the rocket to leave
- [x] Transition to level 2 (harder)
- [x] Game loop: assemble → fuel → launch → repeat

### 🚧 STEP 6 — Aliens 👾
- [ ] Enemies spawning from screen edges
- [ ] Simple movement patterns
- [ ] Collision = lose a life

### 🚧 STEP 7 — Shooting System
- [ ] Laser to destroy aliens
- [ ] Score increases
- [ ] Fire rate limiting

### 🚧 STEP 8 — Complete UI
- [ ] Real-time score
- [ ] Lives counter
- [ ] Game Over / Restart
- [ ] Level transition screen

### 🚧 STEP 9 — Polish
- [ ] Sounds (thrust, shooting, explosions)
- [ ] Particles
- [ ] Animated starfield background

### 🚧 STEP 10 — Sprites
- [ ] Replace rectangles with real sprites
- [ ] Retro pixel art style

---

## 🕹️ Controls

| Key | Action |
|-----|--------|
| `←` `→` or `A` `D` | Horizontal movement |
| `↑` `SPACE` | Thrust (jetpack) |
| `ESC` | Quit |

### Current Gameplay
1. **Assemble the rocket**: fly over the 3 modules in order (bottom → middle → top), position them in the center column
2. **Collect fuel**: 3 capsules appear one by one, drop them on the rocket
3. **Coming soon**: launch to the next level!

---

## 🛠️ Tech Stack

- **Language**: Rust (2024 edition)
- **2D Framework**: [Macroquad](https://macroquad.rs/) — minimalist, no ECS, just a game loop
- **Architecture**: separate modules (player, rocket, fuel, platform)

---

## 📁 Structure

```
RustPac/
├── Cargo.toml
├── PLAN.md              # Original detailed plan
├── README.md            # This file
└── src/
    ├── main.rs          # Game loop and orchestration
    ├── player.rs        # Astronaut physics and controls
    ├── platform.rs      # Landing zone
    ├── rocket.rs        # Rocket and assembly modules
    └── fuel.rs          # Fuel system
```

---

## 🚀 Run the Game

```bash
cd rustpac
cargo run
```

> ⚠️ Requires Rust installed: <https://rustup.rs/>

---

## 💡 Why Macroquad?

Because it's **Rustic simplicity**:
- No complicated entity system
- No boilerplate
- Just `loop { update(); draw(); next_frame().await; }`

Perfect for a "for fun" project where we want to code gameplay, not architecture.

---

## 📝 Conventions

- Emoji commits:
  - `✨` Features
  - `🐛` Bug fixes
  - `📖` Documentation

---

## 🏆 Credits

- **Original**: *Jetpac* by Ultimate Play the Game (1983)
- **Recreation**: For the joy of coding and nostalgia
- **Co-developed by**: Titi (human) and Shuri (Wakandan AI via OpenClaw + Kimi K2.5)
- **Inspiration**: Retro games that prove gameplay trumps graphics

---

> *"For Wakanda!"* — Shuri 🖤

**Work in progress...**
