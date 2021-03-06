pub mod food;

use entity::Entity;
use node::Node;

pub struct Snake {
    pub location: (f32, f32),
    pub size: u8,
    pub movement: String,
    pub speed: f32,
    pub alive: bool,
    pub size_on_x: f32,
    pub size_on_y: f32,
    pub tail: Vec<(f32, f32)>
}

impl Snake {
    pub fn new(location: (f32, f32)) -> Snake {
        Snake {
            location,
            size: 1,
            alive: true,
            movement: String::from("+x"),
            speed: 1.0,
            size_on_x: 10.0,
            size_on_y: 10.0,
            tail: Vec::new(),
        }
    }

    pub fn ramble(&mut self) -> Result<(), &str> {
        if self.movement == "+x" {
            self.location = (self.location.0 + self.speed, self.location.1);

            return Ok(());
        }

        if self.movement == "+y" {
            self.location = (self.location.0, self.location.1 + self.speed);

            return Ok(());
        }

        if self.movement == "-x" {
            self.location = (self.location.0 - self.speed, self.location.1);

            return Ok(());
        }

        if self.movement == "-y" {
            self.location = (self.location.0, self.location.1 - self.speed);

            return Ok(());
        }

        Err("No valid axis")
    }

    pub fn grow(&mut self) {
        self.size = self.size + 1;
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn set_movement(&mut self, movement: &str) {
        self.movement = String::from(movement);
    }

    pub fn set_speed(&mut self, speed: f32) -> Result<(), &str> {
        if speed <= 0.0 {
            return Err("Only positive speed");
        }

        self.speed = speed;

        Ok(())
    }

    pub fn add_tail(&mut self, tail: (f32, f32)) {
        self.tail.push(tail);
    }

    pub fn eval_ramble(&mut self) {
        let mut death = false;

        for t in &self.tail {
            if self.location == *t {
                death = true;
                break;
            }
        }

        if death { self.die() };
    }
}

impl Entity for Snake {
    fn x(&self) -> f32 {
        self.location.0
    }

    fn y(&self) -> f32 {
        self.location.1
    }

    fn get_size_on_x(&self) -> f32 {
        self.size_on_x
    }

    fn get_size_on_y(&self) -> f32 {
        self.size_on_y
    }
}

impl Node for Snake {
    fn get_location(&self) -> (f32, f32) {
        (self.location.0, self.location.1)
    }

    fn get_size(&self) -> f32 {
        self.get_size_on_x()
    }

    fn get_movement(&self) -> &str {
        &self.movement
    }
}

#[cfg(test)]
mod test {
    use snake::Snake;
    use entity::Entity;

    #[test]
    fn test_creation() {
        let s = Snake::new((0.0, 0.0));

        assert_eq!(s.size, 1);
        assert_eq!(s.alive, true);
    }

    #[test]
    fn test_move_on_axis() {
        let mut s = Snake::new((0.0, 0.0));

        s.ramble().unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 0.0);

        s.set_movement("+y");
        s.ramble().unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 1.0);

        s.set_movement("-x");
        s.ramble().unwrap();

        assert_eq!(s.location.0, 0.0);
        assert_eq!(s.location.1, 1.0);

        s.set_movement("-y");
        s.ramble().unwrap();

        assert_eq!(s.location.0, 0.0);
        assert_eq!(s.location.1, 0.0);
    }

    #[test]
    fn test_get_error_on_wrong_axis() {
        let mut s = Snake::new((0.0, 0.0));

        s.set_movement("w");

        if let Err(e) = s.ramble() {
            assert_eq!(e, "No valid axis");
        }
    }

    #[test]
    fn test_grow() {
        let mut s = Snake::new((0.0, 0.0));

        s.grow();
        assert_eq!(s.size, 2);

        s.grow();
        assert_eq!(s.size, 3);
    }

    #[test]
    fn test_die() {
        let mut s = Snake::new((0.0, 0.0));

        s.die();

        assert_eq!(s.alive, false);
    }

    #[test]
    fn test_axis_shortcuts() {
        let s = Snake::new((2.0, 5.0));

        assert_eq!(s.x(), 2.0);
        assert_eq!(s.y(), 5.0);
    }

    #[test]
    fn test_set_movement() {
        let mut s = Snake::new((0.0, 0.0));

        assert_eq!(s.movement, String::from("+x"));

        s.set_movement("+y");

        assert_eq!(s.movement, String::from("+y"));
    }

    #[test]
    fn test_error_on_set_speed() {
        let mut s = Snake::new((0.0, 0.0));

        assert_eq!(s.speed, 1.0);

        if let Err(e) = s.set_speed(-1.0) {
            assert_eq!("Only positive speed", e);
        }
    }

    #[test]
    fn test_set_speed() {
        let mut s = Snake::new((0.0, 0.0));

        assert_eq!(s.speed, 1.0);

        s.set_speed(10.0).unwrap();

        assert_eq!(s.speed, 10.0);
    }

    #[test]
    fn test_tail() {
        let mut s = Snake::new((0.0, 10.0));

        s.add_tail((12.0, 0.0));
        s.add_tail((16.0, 10.0));

        let tail = s.tail;

        assert_eq!(tail[0].0, 12.0);
        assert_eq!(tail[0].1, 0.0);
        assert_eq!(tail[1].0, 16.0);
        assert_eq!(tail[1].1, 10.0);
    }

    #[test]
    fn test_eval_ramble() {
        let mut s = Snake::new((0.0, 0.0));

        let tail = vec![(15.0, 7.0), (11.0, 3.0), (4.0, 3.0)];
        s.tail = tail;

        s.eval_ramble();

        assert_eq!(s.alive, true);

        s.location = (11.0, 3.0);

        s.eval_ramble();

        assert_eq!(s.alive, false);
    }
}
