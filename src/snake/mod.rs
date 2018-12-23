pub struct Snake {
    pub location: (f32, f32),
    pub size: u8,
    pub movement: String,
    pub alive: bool,
}

impl Snake {
    pub fn new(location: (f32, f32)) -> Snake {
        Snake {
            location,
            size: 1,
            alive: true,
            movement: String::from("+x")
        }
    }

    pub fn move_on(&mut self, axis: &str) -> Result<(), &str> {
        if axis == "+x" {
            self.location = (self.location.0 + 1.0, self.location.1);

            return Ok(());
        }

        if axis == "+y" {
            self.location = (self.location.0, self.location.1 + 1.0);

            return Ok(());
        }

        if axis == "-x" {
            self.location = (self.location.0 - 1.0, self.location.1);

            return Ok(());
        }

        if axis == "-y" {
            self.location = (self.location.0, self.location.1 - 1.0);

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

    pub fn x(&self) -> f32 {
        self.location.0
    }

    pub fn y(&self) -> f32 {
        self.location.1
    }

    pub fn set_movement(&mut self, movement: String) {
        self.movement = movement;
    }
}

#[cfg(test)]
mod test {
    use snake::Snake;

    #[test]
    fn test_creation() {
        let s = Snake::new((0.0, 0.0));

        assert_eq!(s.size, 1);
        assert_eq!(s.alive, true);
    }

    #[test]
    fn test_move_on_axis() {
        let mut s = Snake::new((0.0, 0.0));

        s.move_on("+x").unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 0.0);

        s.move_on("+y").unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 1.0);

        s.move_on("-x").unwrap();

        assert_eq!(s.location.0, 0.0);
        assert_eq!(s.location.1, 1.0);

        s.move_on("-y").unwrap();

        assert_eq!(s.location.0, 0.0);
        assert_eq!(s.location.1, 0.0);
    }

    #[test]
    fn test_get_error_on_wrong_axis() {
        let mut s = Snake::new((0.0, 0.0));

        if let Err(e) = s.move_on("w") {
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

        s.set_movement(String::from("+y"));

        assert_eq!(s.movement, String::from("+y"));
    }
}
