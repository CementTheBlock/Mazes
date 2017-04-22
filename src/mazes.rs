extern crate rand;

pub mod direction {
    use rand::{Rng, thread_rng, Rand};
    use rand::distributions::{IndependentSample, Range};

    #[derive(Clone, Copy, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Rand for Direction {
        fn rand<R: Rng>(rng: &mut R) -> Self {
            let between = Range::new(0, 4);
            match between.ind_sample(rng) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => panic!("Function rand in impl Rand for Direction in module node"),
            }
        }
    }

    impl Direction {
        pub fn reverse(&self) -> Self {
            match *self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }

        pub fn vec_elem(self, vec: Vec<Direction>) -> bool {
            for dirs in vec.into_iter() {
                if self == dirs {
                    return true;
                }
            }
            false
        }
    }

    pub fn reselect_direction(directions: Vec<Direction>) -> Direction {
        let mut rng = thread_rng();
        let mut ret_dir = Direction::rand(&mut rng);
        let mut clone = directions.clone();
        while ret_dir.vec_elem(clone) {
            ret_dir = Direction::rand(&mut rng);
            clone = directions.clone();
        }
        ret_dir
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

    pub type Wall = bool;

    #[derive(Clone, Copy)]
    pub struct CellData {
        up: Wall,
        down: Wall,
        left: Wall,
        right: Wall,
        visited: bool,
    }

    impl fmt::Display for CellData {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.visited {
                true => write!(f, "Visited"),
                false => write!(f, "Not Visited"),
            }
        }
    }

    impl CellData {
        pub fn display(&self) -> String {
            format!("{}", self)
        }
    }

    impl CellData {
        fn new() -> CellData {
            CellData {
                up: true,
                down: true,
                left: true,
                right: true,
                visited: false,
            }
        }

        pub fn visit(&mut self) {
            self.visited = true;
        }

        pub fn remove_wall(&mut self, dir: Direction) {
            match dir {
                Direction::Up => self.up = false,
                Direction::Down => self.down = false,
                Direction::Left => self.left = false,
                Direction::Right => self.right = false,
            }
        }
    }


    #[derive(Clone, Copy)]
    pub struct GNode<Neighbor> {
        up: Option<Neighbor>,
        down: Option<Neighbor>,
        left: Option<Neighbor>,
        right: Option<Neighbor>,
        n_type: NodeType,
        cell_data: CellData,
    }

    fn display_opening<N: fmt::Display>(opening: Option<N>) -> String {
        match opening {
            Option::Some(n) => format!("Some(({}))", n),
            Option::None => format!("None"),
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
            let cd = self.cell_data.display();
            write!(f, "{4}\n{0}\n{1}\n{2}\n{3}\n{5}", u, d, l, r, t, cd)
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
                cell_data: CellData::new(),
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

        pub fn get_neighbor_option(self, dir: Direction) -> Option<N> {
            match dir {
                Direction::Up => self.up,
                Direction::Down => self.down,
                Direction::Left => self.left,
                Direction::Right => self.right,
            }
        }

        pub fn update_neighbor(&mut self, dir: Direction, new: Option<N>) {
            match dir {
                Direction::Up => {
                    self.up = new;
                }
                Direction::Down => {
                    self.down = new;
                }
                Direction::Left => {
                    self.left = new;
                }
                Direction::Right => {
                    self.right = new;
                }
            }
        }

        pub fn set_neighbor(&mut self, dir: Direction, neighbor: N) {
            self.update_neighbor(dir, Some(neighbor));
            // cursor::Cursor { index: cursor.index() }
        }

        pub fn set_type(&mut self, n_type: NodeType) {
            self.n_type = n_type;
        }

        pub fn remove_wall(&mut self, dir: Direction) {
            self.cell_data.remove_wall(dir);
        }

        pub fn visit(&mut self) {
            self.cell_data.visit();
        }

        pub fn is_visited(&self) -> bool {
            self.cell_data.visited
        }

        pub fn is_not_visited(&self) -> bool {
            !(self.cell_data.visited)
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

    impl Cursor {
        pub fn display(&self) {
            println!("{}", self)
        }
    }

    pub type Node = GNode<Cursor>;

    #[derive(Clone)]
    pub struct Maze {
        nodes: Vec<Node>,
    }

    impl Maze {
        pub fn empty() -> Maze {
            Maze { nodes: vec![] }
        }
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

        pub fn remove_walls(&mut self, cur_a: Cursor, cur_b: Cursor, dir: Direction) {
            self.at(cur_a).remove_wall(dir);
            self.at(cur_b).remove_wall(dir.reverse());
        }

        pub fn get_visiteds(&self) -> Vec<bool> {
            let mut ret_vec = vec![];
            for node in self.nodes.iter() {
                ret_vec.push(node.is_visited());
            }
            ret_vec
        }

        pub fn get_neighbors(&mut self, cursor: Cursor) -> Maze {
            let mut ret_maze = Maze::empty();
            let mut option: Option<Cursor>;
            let mut maze_copy = self.clone();

            let mut helper =
                |option: Option<Cursor>, ret_maze_ref: &mut Maze| if let Some(cur_b) = option {
                    (*ret_maze_ref).nodes.push(*maze_copy.at(cur_b));
                } else {
                    let mut new_node = Node::new(NodeType::Regular);
                    new_node.visit();
                    (*ret_maze_ref).nodes.push(new_node);
                };

            {
                let ret_maze_ref = &mut ret_maze;
                option = self.at(cursor).get_neighbor_option(Direction::Up);
                helper(option, ret_maze_ref);
                option = self.at(cursor).get_neighbor_option(Direction::Down);
                helper(option, ret_maze_ref);
                option = self.at(cursor).get_neighbor_option(Direction::Left);
                helper(option, ret_maze_ref);
                option = self.at(cursor).get_neighbor_option(Direction::Right);
                helper(option, ret_maze_ref);
            }
            ret_maze
        }

        pub fn visited_directions(&mut self, cursor: Cursor) -> Vec<Direction> {
            let temp = self.get_neighbors(cursor).get_visiteds();
            let mut ret_vec = vec![];
            if temp[0] {
                ret_vec.push(Direction::Up);
            }
            if temp[1] {
                ret_vec.push(Direction::Down);
            }
            if temp[2] {
                ret_vec.push(Direction::Left);
            }
            if temp[3] {
                ret_vec.push(Direction::Right);
            }
            ret_vec
        }

        pub fn size(&mut self) -> (usize, usize) {
            let cursor = self.get_root();
            let mut x = 1;
            let mut y = 1;
            let mut next = self.at(cursor).get_neighbor_option(Direction::Right);
            while let Some(cursor) = next {
                x += 1;
                next = self.at(cursor).get_neighbor_option(Direction::Right);
            }
            next = self.at(cursor).get_neighbor_option(Direction::Down);
            while let Some(cursor) = next {
                y += 1;
                next = self.at(cursor).get_neighbor_option(Direction::Down);
            }
            (x, y)
        }
    }

    pub fn make_plane(side_length_x: usize, side_length_y: usize) -> Maze {
        let mut maze = Maze::new();
        let mut cursor;
        for x in 0..side_length_x {
            cursor = maze.by_coords(x, 0);
            if x < side_length_x - 1 {
                maze.add(cursor, Direction::Right, NodeType::Regular);
            }
            for y in 0..side_length_y - 1 {
                cursor = maze.by_coords(x, y);
                maze.add(cursor, Direction::Down, NodeType::Regular);
            }

        }

        for x in 0..side_length_x - 1 {
            for y in 1..side_length_y {
                let cursor_a = maze.by_coords(x, y);
                let cursor_b = maze.by_coords(x + 1, y);
                maze.add_edge(cursor_a, cursor_b, Direction::Right);
            }
        }

        let cursor = maze.by_coords(side_length_x - 1, side_length_y - 1);
        maze.at(cursor).set_type(NodeType::End);
        maze
    }

    pub fn make_square_plane(side_length: usize) -> Maze {
        make_plane(side_length, side_length)
    }

    pub fn generate_maze(side_length_x: usize, side_length_y: usize) -> Maze {
        let mut maze = make_plane(side_length_x, side_length_y);
        let mut cursor = maze.get_root();
        let mut stack = vec![];
        let mut new_dir;
        let mut new_cur;
        let mut dirs;
        while maze.get_visiteds().iter().any(|&x| !x) {
            maze.at(cursor).visit();
            dirs = maze.visited_directions(cursor);
            if dirs.len() < 4 {
                new_dir = reselect_direction(dirs);
                new_cur = maze.at(cursor).get_neighbor(new_dir);
                stack.push(cursor);
                maze.remove_walls(cursor, new_cur, new_dir);
                cursor = new_cur;
            } else if !stack.is_empty() {
                cursor = stack.pop().expect("generate maze, expect");
            }
        }
        maze
    }

    pub fn generate_square_maze(side_length: usize) -> Maze {
        generate_maze(side_length, side_length)
    }
}

pub mod dot {
    use super::maze::*;
    use super::node::*;
    use super::direction::*;

    pub fn subgraphs(maze: &mut Maze) -> Vec<Vec<Node>> {
        let mut ret_vec: Vec<Vec<Node>> = vec![];
        let (size_x, size_y) = maze.size();
        let mut cursor;
        for y in 0..size_y {
            let mut x_vec: Vec<Node> = vec![];
            for x in 0..size_x {
                cursor = maze.by_coords(x, y);
                x_vec.push(*maze.at(cursor));
            }
            ret_vec.push(x_vec);
        }
        ret_vec
    }

    pub fn name_nodes<'a>(maze: &mut Maze) -> Vec<Vec<&'a str>> {
        let (size_x, size_y) = maze.size();
        let mut ret_vec: Vec<Vec<&str>> = vec![];
        let mut push_val;
        for y in 0..size_y {
            let mut x_vec: Vec<&str> = vec![];
            for x in 0..size_x {
                push_val = &*format!("\"{}-{}\"", x, y);
                x_vec.push(push_val);
            }
            ret_vec.push(x_vec);
        }
        ret_vec
    }

    pub fn dots<'a>(maze: &mut Maze) -> &'a str {
        let names = name_nodes(maze);
        let nodes = subgraphs(maze);
        let mut build_string: String = "digraph {\n".to_string();
        for vecs in names {
            build_string = (build_string + "subgraph {\nrank = same;").to_string();
            for names in vecs {
                build_string = (build_string + names + "; ").to_string();
            }
            build_string = (build_string + "\n}\n").to_string();
        }
        let ret_string: &'a str = &build_string;
        ret_string
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let (x, y) = (6, 2);
        let mut maze = super::maze::generate_maze(x, y);
        assert_eq!(maze.get_visiteds().iter().len(), x * y);
        assert!(maze.get_visiteds().iter().all(|&x| x));
        assert_eq!(maze.size(), (x, y));
        {
            let maze_ref = &mut maze;
            assert_eq!(super::dot::subgraphs(maze_ref).len(), y);
        }
    }
}
