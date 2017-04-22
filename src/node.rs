use std::fmt;
use direction::*;

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

type Wall = bool;

#[derive(Clone, Copy)]
pub struct CellData {
    up: Wall,
    down: Wall,
    left: Wall,
    right: Wall,
    visited: bool,
}
// impl<N: fmt::Display + Clone> fmt::Display for GNode<N> {
// TODO: add digraph styled pretty-printing
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// let u = display_opening(self.up.clone());
// let d = display_opening(self.down.clone());
// let l = display_opening(self.left.clone());
// let r = display_opening(self.right.clone());
// let t = self.n_type.display();
// let cd = self.cell_data.display();
// write!(f, "{4}\n{0}\n{1}\n{2}\n{3}\n{5}", u, d, l, r, t, cd)
// }
// }

impl fmt::Display for CellData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let u = display_wall(self.up.clone());
        let d = display_wall(self.down.clone());
        let l = display_wall(self.left.clone());
        let r = display_wall(self.right.clone());
        let v = display_visited(self.visited.clone());
        write!(f, "{0}\n{1}\n{2}\n{3}\n{4}", v, u, d, l, r)
    }
}

fn display_wall(wall: Wall) -> String {
    match wall {
        true => format!("Wall"),
        false => format!("No Wall"),
    }
}

fn display_visited(visited: bool) -> String {
    match visited {
        true => format!("Visited"),
        false => format!("Not Visited"),
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

    pub fn has_wall(&self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self.cell_data.up,
            Direction::Down => self.cell_data.down,
            Direction::Left => self.cell_data.left,
            Direction::Right => self.cell_data.right,
        }
    }

    pub fn has_no_wall(&self, dir: Direction) -> bool {
        !(self.has_wall(dir))
    }
}
