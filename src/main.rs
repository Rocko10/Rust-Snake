extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Rect};
use ggez::event::{Keycode, Mod};

mod snake;
use snake::Snake;

struct MainState {
    snake: Snake,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState {
            snake: Snake::new((10.0, 10.0))
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        if self.snake.alive {
            self.snake.ramble().unwrap();
        }

        if self.snake.x() > 799.0 || self.snake.x() < 1.0
        || self.snake.y() > 599.0 || self.snake.y() < 1.0 {
            self.snake.die();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let width = 10.0;
        let height = 10.0;
        let snake_shape = Rect::new(self.snake.location.0, self.snake.location.1, width, height);

        graphics::clear(ctx);
        graphics::rectangle(ctx, DrawMode::Fill, snake_shape)?;

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
