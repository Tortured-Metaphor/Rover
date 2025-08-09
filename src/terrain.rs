use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use glam::Vec2;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::rover::Rover;

pub struct Obstacle {
    pub position: Vec2,
    pub radius: f32,
}

pub struct Terrain {
    ground_points: Vec<f32>,
    obstacles: Vec<Obstacle>,
    segment_width: f32,
}

impl Terrain {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(42);
        let segment_width = 50.0;
        let num_segments = 500;
        
        let mut ground_points = Vec::new();
        let mut current_y: f32 = 500.0;
        
        for i in 0..num_segments {
            if i > 0 && i % 10 == 0 {
                current_y += rng.gen_range(-50.0..50.0);
                current_y = current_y.clamp(400.0, 650.0);
            }
            ground_points.push(current_y);
        }
        
        let mut obstacles = Vec::new();
        for i in 5..num_segments - 5 {
            if rng.gen_bool(0.05) {
                let x = i as f32 * segment_width;
                let ground_y = ground_points[i];
                let obstacle_y = ground_y - rng.gen_range(30.0..80.0);
                let radius = rng.gen_range(20.0..40.0);
                
                obstacles.push(Obstacle {
                    position: Vec2::new(x, obstacle_y),
                    radius,
                });
            }
        }
        
        Terrain {
            ground_points,
            obstacles,
            segment_width,
        }
    }

    pub fn get_ground_y(&self, x: f32) -> f32 {
        let segment = (x / self.segment_width) as usize;
        if segment >= self.ground_points.len() {
            return 500.0;
        }
        
        if segment + 1 < self.ground_points.len() {
            let t = (x % self.segment_width) / self.segment_width;
            let y1 = self.ground_points[segment];
            let y2 = self.ground_points[segment + 1];
            y1 + (y2 - y1) * t
        } else {
            self.ground_points[segment]
        }
    }

    pub fn check_obstacle_collision(&self, rover: &Rover) -> bool {
        for obstacle in &self.obstacles {
            let distance = (rover.position - obstacle.position).length();
            if distance < rover.radius + obstacle.radius {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, camera_x: f32, window_width: f32) -> GameResult {
        let start_segment = ((camera_x / self.segment_width) as usize).saturating_sub(1);
        let end_segment = ((camera_x + window_width) / self.segment_width) as usize + 2;
        let end_segment = end_segment.min(self.ground_points.len());
        
        if start_segment < end_segment {
            let mut mb = MeshBuilder::new();
            
            for i in start_segment..end_segment - 1 {
                let x1 = i as f32 * self.segment_width;
                let x2 = (i + 1) as f32 * self.segment_width;
                let y1 = self.ground_points[i];
                let y2 = self.ground_points[i + 1];
                
                mb.polygon(
                    DrawMode::fill(),
                    &[
                        [x1, y1],
                        [x2, y2],
                        [x2, 800.0],
                        [x1, 800.0],
                    ],
                    Color::from_rgb(100, 80, 60),
                )?;
            }
            
            let ground_mesh = Mesh::from_data(ctx, mb.build());
            canvas.draw(&ground_mesh, DrawParam::default());
        }
        
        for obstacle in &self.obstacles {
            if obstacle.position.x >= camera_x - 100.0 && obstacle.position.x <= camera_x + window_width + 100.0 {
                let rock = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [0.0, 0.0],
                    obstacle.radius,
                    0.1,
                    Color::from_rgb(70, 60, 50),
                )?;
                
                canvas.draw(&rock, DrawParam::default().dest(obstacle.position));
                
                let highlight = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    [-obstacle.radius * 0.3, -obstacle.radius * 0.3],
                    obstacle.radius * 0.3,
                    0.1,
                    Color::from_rgb(90, 80, 70),
                )?;
                
                canvas.draw(&highlight, DrawParam::default().dest(obstacle.position));
            }
        }
        
        Ok(())
    }
}