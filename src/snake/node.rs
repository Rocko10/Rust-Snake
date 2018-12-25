pub struct Node {
    loc: (f32, f32),
    tail_loc: (f32, f32),
    next: Option<Box<Node>>,
    size: f32,
}

impl Node {
    pub fn new(loc: (f32, f32), size: f32, mov: &str) -> Node {

        let tail_loc: (f32, f32) = match mov {
            "+x" => (loc.0 - (size * 2.0), loc.1),
            "-x" => (loc.0 + (size * 2.0), loc.1),
            "+y" => (loc.0, loc.1 - (size * 2.0)),
            "-y" => (loc.0, loc.1 + (size * 2.0)),
            _ => (0.0, 0.0)
        };

        Node { loc, size, tail_loc, next: None }
    }
}

#[cfg(test)]
mod test {
    use snake::node::Node;

    #[test]
    fn test_new_head() {
        let h = Node::new((5.0, 6.0), 2.0, "+x");

        assert_eq!(h.loc.0, 5.0);
        assert_eq!(h.loc.1, 6.0);
        assert_eq!(h.size, 2.0);
        assert_eq!(h.tail_loc, (1.0, 6.0));

        if let Some(_boxed_node) = &h.next {
            assert_eq!(0, 1)
        }

        if let None = h.next {
            assert_eq!(1, 1)
        }

    }
}
