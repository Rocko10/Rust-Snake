pub struct Snake {
    pub location: (f32, f32),
    pub size: u8
}

impl Snake {
    pub fn new(location: (f32, f32)) -> Snake {
        Snake { location, size: 1 }
    }

    pub fn move_on(&mut self, axis: &str) -> Result<(), &str> {
        if axis == "x" {
            self.location = (self.location.0 + 1.0, self.location.1);

            return Ok(());
        }

        if axis == "y" {
            self.location = (self.location.0, self.location.1 + 1.0);

            return Ok(());
        }

        Err("No valid axis")
    }
}

#[cfg(test)]
mod test {
    use snake::Snake;

    #[test]
    fn test_creation() {
        let s = Snake::new((0.0, 0.0));

        assert_eq!(s.size, 1);
    }

    #[test]
    fn test_move_on_axis() {
        let mut s = Snake::new((0.0, 0.0));

        s.move_on("x").unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 0.0);

        s.move_on("y").unwrap();

        assert_eq!(s.location.0, 1.0);
        assert_eq!(s.location.1, 1.0);
    }

    #[test]
    fn test_get_error_on_wrong_axis() {
        let mut s = Snake::new((0.0, 0.0));

        if let Err(e) = s.move_on("w") {
            assert_eq!(e, "No valid axis");
        }

    }
}
