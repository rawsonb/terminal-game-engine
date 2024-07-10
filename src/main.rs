use std::vec;

use crossterm::{cursor::position, event::KeyCode};
use engine::{EntityData, Update, World};

mod engine;
mod graphics;

const MAP_HEIGHT: u16 = 15;
const MAP_WIDTH: u16 = 25; // in characters
const BULLET_SPEED: f64 = 3.5;
const PLAYER_SPEED: f64 = 4.5; // characters per second

fn main() {
    let mut world = World::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    world.add_entity(Ship {
        position: (12, 13),
        tilt: (0.0, 0.0),
    });
    world.add_entity(Wall {});
    world.add_entity(Barrier { position: (4, 12) });
    world.add_entity(Barrier { position: (5, 12) });
    world.add_entity(Barrier { position: (6, 12) });
    world.add_entity(Barrier { position: (11, 12) });
    world.add_entity(Barrier { position: (12, 12) });
    world.add_entity(Barrier { position: (13, 12) });
    world.add_entity(Barrier { position: (18, 12) });
    world.add_entity(Barrier { position: (19, 12) });
    world.add_entity(Barrier { position: (20, 12) });
    let _ = world.init();
}

struct Ship {
    position: (u16, u16),
    tilt: (f64, f64),
}

impl Update for Ship {
    fn update(&mut self, delta: f64, world: &mut World, id: i64) {
        world.debug_draw(format!("Tilt: {:?}", self.tilt).as_str());
        world.debug_draw(
            format!("\n X_Position: {:?}", self.position.0).as_str(),
        );
        world.debug_draw(
            format!("\n\n Last Input: {:?}", world.ui.last_input).as_str(),
        );
        world.debug_draw(
            format!("\n\n\n Current Input: {:?}", world.ui.current_input)
                .as_str(),
        );
        world.debug_draw(format!("\n\n\n\n Delta: {:?}", delta).as_str());
        match world.ui.current_input {
            Some(KeyCode::Left) => {
                if world.ui.last_input.is_some_and(|x| x == KeyCode::Right) {
                    self.tilt = (0.0, 0.0);
                    world.ui.current_input = None;
                } else {
                    self.tilt.0 -= PLAYER_SPEED * delta;
                }
            }
            Some(KeyCode::Right) => {
                if world.ui.last_input.is_some_and(|x| x == KeyCode::Left) {
                    self.tilt = (0.0, 0.0);
                    world.ui.current_input = None;
                } else {
                    self.tilt.0 += PLAYER_SPEED * delta;
                }
            }

            Some(KeyCode::Up) => {
                world.add_entity(Bullet {
                    position: (
                        self.position.0 as f64,
                        (self.position.1 - 1) as f64,
                    ),
                });
                self.tilt = (0.0, 0.0);
                world.ui.current_input = None;
            }
            _ => {}
        }
        if self.tilt.0 > 1.0 {
            self.position.0 += 1;
            self.tilt.0 -= 1.0;
        } else if self.tilt.0 < -1.0 {
            self.position.0 -= 1;
            self.tilt.0 += 1.0;
        }
        self.position.0 = self.position.0.clamp(1, MAP_WIDTH - 2);
        self.position.1 = self.position.1.clamp(1, MAP_HEIGHT - 2);

        world.draw(self.position, '^', crossterm::style::Color::Green, id);
    }
}

struct Wall {}

impl Update for Wall {
    fn update(&mut self, _delta: f64, world: &mut World, id: i64) {
        for r in 0..MAP_WIDTH {
            for c in 0..MAP_HEIGHT {
                if r == 0 || c == 0 || r == MAP_WIDTH - 1 || c == MAP_HEIGHT - 1
                {
                    world.draw((r, c), '#', crossterm::style::Color::Grey, id);
                }
            }
        }
    }
}

struct Bullet {
    position: (f64, f64),
}

impl Update for Bullet {
    fn update(&mut self, delta: f64, world: &mut World, id: i64) {
        self.position =
            (self.position.0, self.position.1 - delta * BULLET_SPEED);
        let target_pos =
            (self.position.0, self.position.1 - delta * BULLET_SPEED);
        if target_pos.1 < 1.0 {
            world.remove_entity(id);
        } else {
            world.draw(
                (
                    self.position.0.round() as u16,
                    self.position.1.round() as u16,
                ),
                '*',
                crossterm::style::Color::Red,
                id,
            );
        }
    }
}

struct Barrier {
    position: (u16, u16),
}

impl Update for Barrier {
    fn update(&mut self, _delta: f64, world: &mut World, id: i64) {
        world.draw(self.position, '#', crossterm::style::Color::Yellow, id);
    }
}
