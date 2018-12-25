extern crate ggez;
extern crate rand;

use ggez::*;
use ggez::graphics::{DrawMode, Rect};
use ggez::event::{Keycode, Mod};

use rand::Rng;

mod entity;
use entity::Entity;

mod sys_interaction;
use sys_interaction::SysInt;

mod snake;
use snake::Snake;
use snake::food::Food;

struct MainState {
    snake: Snake,
    food: Food,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut snake = Snake::new((10.0, 10.0));
        snake.set_speed(5.0).unwrap();

        let mut rng = rand::thread_rng();
        let food = Food::new((rng.gen_range(5, 780) as f32, rng.gen_range(5, 580) as f32));

        let state = MainState {
            snake,
            food,
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        if self.snake.alive {
            self.snake.ramble().unwrap();
        }

        // TODO: Move this to the sys_interaction
        if self.snake.x() > 799.0 || self.snake.x() < 1.0
        || self.snake.y() > 599.0 || self.snake.y() < 1.0 {
            self.snake.die();
        }

        if SysInt::collide(&self.food, &self.snake) {
            let mut rng = rand::thread_rng();
            self.food.set_location((rng.gen_range(5, 780) as f32, rng.gen_range(5, 580) as f32));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let snake_shape = Rect::new(self.snake.location.0,
            self.snake.location.1,
            self.snake.get_size_on_x(), self.snake.get_size_on_y());
        let food_shape = Rect::new(self.food.x(),
            self.food.y(),
            self.snake.get_size_on_x(),
            self.snake.get_size_on_y());
        // TODO: Change color of the food
        // graphics::set_color(&mut food_shape, graphics::Color::new(255.0, 255.0, 0.0, 1.0));

        graphics::clear(ctx);

        graphics::rectangle(ctx, DrawMode::Fill, snake_shape)?;
        graphics::rectangle(ctx, DrawMode::Fill, food_shape)?;

        if !self.snake.alive {
            graphics::set_background_color(ctx, graphics::Color::new(255.0, 0.0, 0.0, 1.0));
        }

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up => self.snake.set_movement("-y"),
            Keycode::Down => self.snake.set_movement("+y"),
            Keycode::Left => self.snake.set_movement("-x"),
            Keycode::Right => self.snake.set_movement("+x"),
            _ => println!("Another key")
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("snake", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
