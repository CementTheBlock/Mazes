use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum NodeType {
    Regular,
    Start,
    End,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Node {
    up: Option<Cursor>,
    down: Option<Cursor>,
    left: Option<Cursor>,
    right: Option<Cursor>,
    n_type: NodeType,
}

impl Node {
    fn print_node(&self) {
        println!("{4}\n{0}\n{1}\n{2}\n{3}",
                 opening_to_string(self.up), opening_to_string(self.down),
                 opening_to_string(self.left), opening_to_string(self.right),
                 node_type_to_string(self.n_type));
    }
}

fn opening_to_string(opening: Option<Cursor>) -> String {
    let ret_string; 
    match opening {
        Option::Some(cursor) => {
            ret_string = fmt::format(format_args!("Some(({}))", cursor.index));
            return ret_string;
        },
        Option::None => {
            ret_string = fmt::format(format_args!("None"));
            return ret_string;
        },
    }
}

fn node_type_to_string<'a>(n_type: NodeType) -> &'a str {
    match n_type {
        NodeType::Start => "Start",
        NodeType::Regular => "Regular",
        NodeType::End => "End",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cursor { index: usize, }

#[derive(Clone)]
struct Maze { nodes: Vec<Node> }

impl Maze {
    fn get(&self, cursor: Cursor) -> &Node {
        let i = cursor.index;
        &(self.nodes[i])
    }

    fn add_dir(&mut self, cursor: Cursor, dir: Direction, node_type: NodeType) -> Cursor {
        let node = *self.get(cursor);
        let new_node = new_node(node_type);
        let ret_cursor = Cursor {index: self.nodes.len()};
        self.nodes.push(update_node_in_dir(new_node, reverse_dir(dir), &cursor));
        self.nodes[cursor.index] = update_node_in_dir(node, dir, &ret_cursor);
        ret_cursor
    }
}

fn reverse_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn update_node_in_dir(node: Node, dir: Direction, cursor: & Cursor) -> Node {
    let new_cursor = Cursor { index: cursor.index };
    match dir {
        Direction::Up => Node {up: Some(new_cursor), .. node},
        Direction::Down => Node {down: Some(new_cursor), .. node},
        Direction::Left => Node {left: Some(new_cursor), .. node},
        Direction::Right => Node {right: Some(new_cursor), .. node},
    }
}

fn new_node(n_type: NodeType) -> Node {
    match n_type {
        NodeType::Regular => Node {up: None, down: None, left: None, right: None, n_type: NodeType::Regular},
        NodeType::End => Node {up: None, down: None, left: None, right: None, n_type: NodeType::End},
        NodeType::Start => panic!("This is impossible"),
    }
}

fn new() -> Maze {
    Maze { nodes: vec![Node { up: None, down: None, left: None, right: None, n_type: NodeType::Start }] }
}

fn get_root() -> Cursor {
    Cursor {index: 0}
}

fn make_plane(side_length: usize) -> Maze {
    let mut maze = new();
    let mut cursor;
    let y = 0;
    for x in 0 .. side_length {
        cursor = maze_coords(&maze, x, y);
        maze.add_dir(cursor, Direction::Right, NodeType::Regular);
        for y in 0 .. side_length - 1 {
            cursor = maze_coords(&maze, x, y);
            maze.add_dir(cursor, Direction::Down, NodeType::Regular);
        }
    }
    let mut cursor2;
    for x in 0 .. (side_length - 1) {
        for y in 1 .. side_length {
            cursor = maze_coords(&maze, x, y);
            cursor2 = maze_coords(&maze, x+1, y);
            maze = update_two_nodes(maze, cursor, cursor2, Direction::Right);
        }
    }
    cursor = maze_coords(&maze, side_length - 1, side_length - 1);
    let node = *maze.get(cursor);
    maze.nodes[cursor.index] = Node {n_type: NodeType::End, .. node};
    maze
}

fn update_two_nodes(maze: Maze, cursor_one: Cursor, cursor_two: Cursor, dir: Direction) -> Maze{
    let mut ret_maze = maze;
    ret_maze.nodes[cursor_one.index] = update_node_in_dir(*ret_maze.get(cursor_one), dir, &cursor_two);
    ret_maze.nodes[cursor_two.index] = update_node_in_dir(*ret_maze.get(cursor_two), reverse_dir(dir), &cursor_one);
    ret_maze
}

fn maze_next(maze: & Maze, cursor: Cursor, dir: Direction) -> Cursor {
    let node = maze.get(cursor);
    match dir {
        Direction::Up => node.up.expect("Invalid up"),
        Direction::Down => node.down.expect("Invalid down"),
        Direction::Left => node.left.expect("Invalid left"),
        Direction::Right => node.right.expect("Invalid right"),
    }
}

fn maze_coords(maze: & Maze, x: usize, y: usize) -> Cursor {
    let mut cursor = get_root();
    for _ in 0 .. x {
        cursor = maze_next(maze, cursor, Direction::Right);
    }
    for _ in 0 .. y {
        cursor = maze_next(maze, cursor, Direction::Down);
    }
    cursor
}

fn main() {
    let maze = make_plane(4);
    let cursor = maze_coords(&maze, 3, 3);
    maze.get(cursor).print_node();
}
