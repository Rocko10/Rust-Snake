pub trait Entity {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn get_size_on_x(&self) -> f32;
    fn get_size_on_y(&self) -> f32;
}
