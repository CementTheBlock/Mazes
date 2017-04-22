use std::fmt;
use direction::*;
use node::*;

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

        let mut helper = |option: Option<Cursor>, ret_maze_ref: &mut Maze| {
            let mut push = |node| (*ret_maze_ref).nodes.push(node);
            if let Some(cur_b) = option {
                push(*maze_copy.at(cur_b));
            } else {
                let mut new_node = Node::new(NodeType::Regular);
                new_node.visit();
                push(new_node);
            }
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

    pub fn print_maze(&mut self) {
        let (size_x, size_y) = self.size();
        let mut cursor;
        for x in 0..size_x {
            for y in 0..size_y {
                cursor = self.by_coords(x, y);
                self.at(cursor).print();
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_works() {
        let (x, y) = (4, 4);
        let mut maze = generate_maze(x, y);
        assert_eq!(maze.get_visiteds().iter().len(), x * y);
        assert!(maze.get_visiteds().iter().all(|&x| x));
        assert_eq!(maze.size(), (x, y));
    }
}
