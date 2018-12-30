pub trait Node {
    fn get_location(&self) -> (f32, f32);
    fn get_size(&self) -> f32;
    fn get_movement(&self) -> &str;    
}
