use node::Node;

#[derive(Clone)]
pub struct Tail {
    pub location: (f32, f32),
    pub size: f32,
    pub movement: String
}

impl Tail {
    pub fn new(movement: &str, location: (f32, f32), size: f32) -> Tail {
        Tail {
            location: Tail::set_location(movement, location, size),
            size,
            movement: String::from(movement)
        }
    }

    pub fn set_location(movement: &str, location: (f32, f32), size: f32) -> (f32, f32) {
        match movement {
            "+x" => (location.0 - (size * 2.0), location.1),
            "-x" => (location.0 + (size * 2.0), location.1),
            "+y" => (location.0, location.1 - (size * 2.0)),
            "-y" => (location.0, location.1 + (size * 2.0)),
            _ => (0.0, 0.0)
        }
    }

    pub fn set_loc(&mut self, location: (f32, f32)) {
        self.location = location;
    }
}

impl Node for Tail {
    fn get_location(&self) -> (f32, f32) {
        (self.location.0, self.location.1)
    }

    fn get_size(&self) -> f32 {
        self.size
    }

    fn get_movement(&self) -> &str {
        &self.movement
    }
}

#[cfg(test)]
mod test {}
