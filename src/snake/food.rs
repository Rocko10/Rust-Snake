use entity::Entity;

pub struct Food {
    location: (f32, f32),
    pub size_on_x: f32,
    pub size_on_y: f32,
}

impl Food {
    pub fn new(location: (f32, f32)) -> Food {
        Food {
            location,
            size_on_x: 10.0,
            size_on_y: 10.0,
        }
    }

    pub fn set_location(&mut self, location: (f32, f32)) {
        self.location = location
    }
}

impl Entity for Food {
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

#[cfg(test)]
mod test {

    use snake::food::Food;
    use entity::Entity;

    #[test]
    fn test_new() {
        let f = Food::new((1.0, 2.0));

        assert_eq!(f.x(), 1.0);
        assert_eq!(f.y(), 2.0);
    }

    #[test]
    fn test_set_location() {
        let mut f = Food::new((0.0, 0.0));

        assert_eq!(f.x(), 0.0);
        assert_eq!(f.y(), 0.0);

        f.set_location((2.0, 3.0));

        assert_eq!(f.x(), 2.0);
        assert_eq!(f.y(), 3.0);
    }

}
