use entity::Entity;

pub struct SysInt {}

impl SysInt {
    pub fn collide(e1: &Entity, e2: &Entity) -> bool {

        if e1.x() < e2.x() + e2.get_size_on_x() &&
        e1.x() + e1.get_size_on_x() > e2.x() &&
        e1.y() < e2.y() + e2.get_size_on_y() &&
        e1.y() + e1.get_size_on_y() > e2.y() {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {

    use snake::Snake;
    use snake::food::Food;
    use entity::Entity;
    use sys_interaction::SysInt;

    #[test]
    fn test_not_collide() {
        let s: &Entity = &Snake::new((50.0, 50.0));
        let f: &Entity = &Food::new((10.0, 10.0));

        let collide: bool = SysInt::collide(s, f);

        assert_eq!(collide, false)

    }

    #[test]
    fn test_collide() {
        let s: &Entity = &Snake::new((10.0, 10.0));
        let f: &Entity = &Food::new((10.0, 10.0));

        let collide: bool = SysInt::collide(s, f);

        assert_eq!(collide, true)
    }

}
