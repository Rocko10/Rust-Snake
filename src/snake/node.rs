pub struct Node<'a> {
    loc: (f32, f32),
    tail_loc: (f32, f32),
    next: Option<&'a Node<'a>>,
    size: f32,
    mov: &'a str
}

impl<'a> Node<'a> {
    pub fn new_head(loc: (f32, f32), size: f32, mov: &str) -> Node {
        Node {
            loc,
            size,
            tail_loc: Node::calc_tail_loc(loc, mov, size),
            next: None,
            mov,
        }
    }

    pub fn new_part(next: &'a Node) -> Node<'a> {
        let size = next.size;
        let mov = next.mov;

        Node {
            loc: next.tail_loc,
            tail_loc: Node::calc_tail_loc(next.tail_loc, mov, size),
            next: Some(next),
            size,
            mov
        }
    }

    fn calc_tail_loc(loc: (f32, f32), mov: &str, size: f32) -> (f32, f32) {
        match mov {
            "+x" => (loc.0 - (size * 2.0), loc.1),
            "-x" => (loc.0 + (size * 2.0), loc.1),
            "+y" => (loc.0, loc.1 - (size * 2.0)),
            "-y" => (loc.0, loc.1 + (size * 2.0)),
            _ => (0.0, 0.0)
        }
    }
}

#[cfg(test)]
mod test {
    use snake::node::Node;

    #[test]
    fn test_new_head() {
        let h = Node::new_head((5.0, 6.0), 2.0, "+x");

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

    #[test]
    fn test_head_with_tail() {
        let h = Node::new_head((5.0, 6.0), 2.0, "+x");
        let p = Node::new_part(&h);

        assert_eq!(p.loc.0, h.tail_loc.0);
        assert_eq!(p.loc.1, h.tail_loc.1);
    }
}
