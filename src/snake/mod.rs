pub struct Snake {
    pub location: (f32, f32),
    pub size: u8
}

impl Snake {
    fn new(location: (f32, f32)) -> Snake {
        Snake { location, size: 1 }
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
}
