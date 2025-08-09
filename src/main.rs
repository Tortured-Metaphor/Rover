mod rover;
mod terrain;
mod physics;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Rect};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use glam::Vec2;

use rover::Rover;
use terrain::Terrain;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

struct GameState {
    rover: Rover,
    terrain: Terrain,
    camera_x: f32,
    game_over: bool,
    distance_traveled: f32,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let rover = Rover::new(Vec2::new(200.0, 300.0));
        let terrain = Terrain::new();
        
        Ok(GameState {
            rover,
            terrain,
            camera_x: 0.0,
            game_over: false,
            distance_traveled: 0.0,
        })
    }

    fn restart(&mut self) {
        self.rover = Rover::new(Vec2::new(200.0, 300.0));
        self.terrain = Terrain::new();
        self.camera_x = 0.0;
        self.game_over = false;
        self.distance_traveled = 0.0;
    }

    fn update_camera(&mut self) {
        let target_x = self.rover.position.x - WINDOW_WIDTH / 3.0;
        self.camera_x += (target_x - self.camera_x) * 0.1;
        
        if self.camera_x < 0.0 {
            self.camera_x = 0.0;
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            if ctx.keyboard.is_key_pressed(KeyCode::R) {
                self.restart();
            }
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();
        
        let thrust_up = ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W);
        let thrust_left = ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A);
        let thrust_right = ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D);
        
        self.rover.update(dt, thrust_up, thrust_left, thrust_right);
        
        let ground_y = self.terrain.get_ground_y(self.rover.position.x);
        if self.rover.check_ground_collision(ground_y) {
            self.rover.land(ground_y);
        }
        
        if self.terrain.check_obstacle_collision(&self.rover) {
            self.game_over = true;
        }
        
        self.update_camera();
        
        let new_distance = (self.rover.position.x / 100.0).max(self.distance_traveled);
        self.distance_traveled = new_distance;
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(20, 20, 40));
        
        canvas.set_screen_coordinates(Rect::new(
            self.camera_x,
            0.0,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ));
        
        self.terrain.draw(ctx, &mut canvas, self.camera_x, WINDOW_WIDTH)?;
        
        self.rover.draw(ctx, &mut canvas)?;
        
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT));
        
        let ui_text = if self.game_over {
            format!(
                "GAME OVER\nDistance: {:.1}m\nFuel: {:.0}\n\nPress R to restart",
                self.distance_traveled,
                self.rover.fuel
            )
        } else {
            format!(
                "Distance: {:.1}m\nFuel: {:.0}\n\nArrows/WASD to thrust",
                self.distance_traveled,
                self.rover.fuel
            )
        };
        
        let text = graphics::Text::new(ui_text);
        canvas.draw(&text, DrawParam::default().dest([10.0, 10.0]).color(Color::WHITE));
        
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("rover_game", "author")
        .window_setup(ggez::conf::WindowSetup::default().title("Rover Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    
    event::run(ctx, event_loop, state)
}