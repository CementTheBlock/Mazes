pub mod direction {
    #[derive(Clone, Copy)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        pub fn reverse(&self) -> Direction {
            match *self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }
}

pub mod node {
    use std::fmt;
    use super::direction::*;

    #[derive(Clone, Copy, PartialEq)]
    pub enum NodeType {
        Regular,
        Start,
        End,
    }

    impl fmt::Display for NodeType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                NodeType::Start => write!(f, "Start"),
                NodeType::Regular => write!(f, "Regular"),
                NodeType::End => write!(f, "End"),
            }
        }
    }

    impl NodeType {
        pub fn display(&self) -> String {
            format!("{}", self)
        }
    }

    // TODO: add wall semantics
    #[derive(Clone, Copy)]
    pub struct GNode<Neighbor> {
        up: Option<Neighbor>,
        down: Option<Neighbor>,
        left: Option<Neighbor>,
        right: Option<Neighbor>,
        n_type: NodeType,
    }

    fn display_opening<N: fmt::Display>(opening: Option<N>) -> String {
        match opening {
            Option::Some(n) => format!("Some(({}))", n),
            Option::None => format!("None")
        }
    }

    impl<N: fmt::Display + Clone> fmt::Display for GNode<N> {
        // TODO: add digraph styled pretty-printing
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let u = display_opening(self.up.clone());
            let d = display_opening(self.down.clone());
            let l = display_opening(self.left.clone());
            let r = display_opening(self.right.clone());
            let t = self.n_type.display();
            write!(f, "{4}\n{0}\n{1}\n{2}\n{3}", u, d, l, r, t)
        }
    }

    impl<N: fmt::Display + Clone> GNode<N> {
        pub fn display(&self) -> String {
            format!("{}", self)
        }

        pub fn print(&self) {
            println!("{}", self.display())
        }
    }

    impl<N> GNode<N> {
        pub fn unsafe_new(n_type: NodeType) -> GNode<N> {
            GNode {
                up: None,
                down: None,
                left: None,
                right: None,
                n_type: n_type,
            }
        }

        pub fn new(n_type: NodeType) -> GNode<N> {
            if n_type == NodeType::Start {
                panic!("This is impossible");
            }
            GNode::unsafe_new(n_type)
        }

        pub fn get_neighbor(self, dir: Direction) -> N {
            match dir {
                Direction::Up => self.up.expect("Invalid up"),
                Direction::Down => self.down.expect("Invalid down"),
                Direction::Left => self.left.expect("Invalid left"),
                Direction::Right => self.right.expect("Invalid right"),
            }
        }

        pub fn update_neighbor(&mut self, dir: Direction, new: Option<N>) {
            match dir {
                Direction::Up => { self.up = new; },
                Direction::Down => { self.down = new; },
                Direction::Left => { self.left = new; },
                Direction::Right => { self.right = new; },
            }
        }

        pub fn set_neighbor(&mut self, dir: Direction, neighbor: N) {
            self.update_neighbor(dir, Some(neighbor));
            // cursor::Cursor { index: cursor.index() }
        }

        pub fn set_type(&mut self, n_type: NodeType) {
            self.n_type = n_type;
        }
    }
}

pub mod maze {
    use std::fmt;
    use super::direction::*;
    use super::node::*;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Cursor {
        index: usize,
    }

    impl Cursor {
        pub fn unsafe_new(index: usize) -> Cursor {
            Cursor { index: index }
        }

        pub fn index(&self) -> usize {
            self.index
        }
    }

    impl fmt::Display for Cursor {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.index)
        }
    }

    pub type Node = GNode<Cursor>;

    #[derive(Clone)]
    pub struct Maze {
        nodes: Vec<Node>,
    }

    impl Maze {
        pub fn new() -> Maze {
            Maze { nodes: vec![Node::unsafe_new(NodeType::Start)] }
        }

        pub fn at(&mut self, cursor: Cursor) -> &mut Node {
            &mut (self.nodes[cursor.index()])
        }

        pub fn get(&self, cursor: Cursor) -> &Node {
            &(self.nodes[cursor.index()])
        }

        pub fn get_root(&self) -> Cursor {
            Cursor { index: 0 }
        }

        pub fn add(&mut self, cursor: Cursor, dir: Direction, node_type: NodeType) -> Cursor {
            let mut new_node = Node::new(node_type);
            let ret_cursor = Cursor::unsafe_new(self.nodes.len());
            new_node.set_neighbor(dir.reverse(), cursor);
            self.nodes.push(new_node);
            self.at(cursor).set_neighbor(dir, ret_cursor);
            ret_cursor
        }

        pub fn cursor_move(&self, cursor: &mut Cursor, dir: Direction) {
            *cursor = self.get(*cursor).get_neighbor(dir);
        }

        pub fn by_coords(&self, x: usize, y: usize) -> Cursor {
            let mut cursor = self.get_root();
            for _ in 0..x {
                self.cursor_move(&mut cursor, Direction::Right);
            }
            for _ in 0..y {
                self.cursor_move(&mut cursor, Direction::Down);
            }
            cursor
        }

        pub fn add_edge(&mut self, cur_a: Cursor, cur_b: Cursor, dir: Direction) {
            self.at(cur_a).set_neighbor(dir, cur_b);
            self.at(cur_b).set_neighbor(dir.reverse(), cur_a);
        }
    }

    pub fn make_plane(side_length: usize) -> Maze {
        let mut maze = Maze::new();

        for x in 0..side_length {
            for y in 0..side_length {
                let cursor = maze.by_coords(x, y);
                maze.add(cursor, Direction::Down, NodeType::Regular);
                if x < side_length - 1 {
                    maze.add(cursor, Direction::Right, NodeType::Regular);
                }
            }
        }
        
        for x in 0..side_length - 1 {
            for y in 1..side_length {
                let cursor_a = maze.by_coords(x, y);
                let cursor_b = maze.by_coords(x + 1, y);
                maze.add_edge(cursor_a, cursor_b, Direction::Right);
            }
        }

        let cursor = maze.by_coords(side_length - 1, side_length - 1);
        maze.at(cursor).set_type(NodeType::End);
        maze
    }
}

// TODO: add wall-related utility functions
// TODO: add actual maze generation function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let maze = super::maze::make_plane(4);
        let cursor = maze.by_coords(3, 3);
        // TODO: replace with assert!
        maze.get(cursor).print();
    }
}
