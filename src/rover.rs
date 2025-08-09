use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use glam::Vec2;

use crate::physics::{GRAVITY, THRUST_POWER, MAX_FUEL, FUEL_CONSUMPTION_RATE};

pub struct Rover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub fuel: f32,
    pub radius: f32,
    pub on_ground: bool,
}

impl Rover {
    pub fn new(position: Vec2) -> Self {
        Rover {
            position,
            velocity: Vec2::ZERO,
            fuel: MAX_FUEL,
            radius: 15.0,
            on_ground: false,
        }
    }

    pub fn update(&mut self, dt: f32, thrust_up: bool, thrust_left: bool, thrust_right: bool) {
        if !self.on_ground {
            self.velocity.y += GRAVITY * dt;
        }
        
        if self.fuel > 0.0 {
            if thrust_up {
                self.velocity.y -= THRUST_POWER * dt;
                self.fuel -= FUEL_CONSUMPTION_RATE * dt;
                self.on_ground = false;
            }
            
            if thrust_left {
                self.velocity.x -= THRUST_POWER * 0.7 * dt;
                self.fuel -= FUEL_CONSUMPTION_RATE * 0.5 * dt;
            }
            
            if thrust_right {
                self.velocity.x += THRUST_POWER * 0.7 * dt;
                self.fuel -= FUEL_CONSUMPTION_RATE * 0.5 * dt;
            }
        }
        
        self.fuel = self.fuel.max(0.0);
        
        self.velocity.x *= 0.98;
        
        self.position += self.velocity * dt;
        
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x = 0.0;
        }
    }

    pub fn check_ground_collision(&self, ground_y: f32) -> bool {
        self.position.y + self.radius >= ground_y
    }

    pub fn land(&mut self, ground_y: f32) {
        self.position.y = ground_y - self.radius;
        self.velocity.y = 0.0;
        self.on_ground = true;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let body = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0.0, 0.0],
            self.radius,
            0.1,
            Color::from_rgb(200, 100, 50),
        )?;
        
        canvas.draw(&body, DrawParam::default().dest(self.position));
        
        let window = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [0.0, -5.0],
            self.radius * 0.4,
            0.1,
            Color::from_rgb(100, 150, 200),
        )?;
        
        canvas.draw(&window, DrawParam::default().dest(self.position));
        
        if self.velocity.y < -50.0 && self.fuel > 0.0 {
            let flame = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [0.0, self.radius + 10.0],
                8.0,
                0.1,
                Color::from_rgb(255, 150, 0),
            )?;
            canvas.draw(&flame, DrawParam::default().dest(self.position));
        }
        
        if (self.velocity.x < -50.0 || self.velocity.x > 50.0) && self.fuel > 0.0 {
            let side_flame_x = if self.velocity.x < 0.0 { self.radius + 5.0 } else { -self.radius - 5.0 };
            let side_flame = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                [side_flame_x, 0.0],
                5.0,
                0.1,
                Color::from_rgb(255, 200, 100),
            )?;
            canvas.draw(&side_flame, DrawParam::default().dest(self.position));
        }
        
        Ok(())
    }
}