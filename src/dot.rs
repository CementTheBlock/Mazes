use maze::*;
use direction::*;
// use node::*;
use std::mem;

type DotColor = String; // FIXME: maybe use an enum
type DotPosition = (i32, i32);

#[derive(Clone, PartialEq, Eq)]
pub struct DotNode {
    label: String,
    color: Option<DotColor>,
    position: Option<DotPosition>,
}

impl DotNode {
    pub fn make(label: &str) -> DotNode {
        DotNode {
            label: String::from(label),
            color: None,
            position: None,
        }
    }

    pub fn set_color(&mut self, color: DotColor) {
        self.color = Some(color);
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = Some((x, y));
    }

    pub fn render(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("\"{}\"", self.label));

        let mut attrs = Vec::new();

        if let Some(c) = self.color.clone() {
            attrs.push(format!("color=\"{}\"", c));
        }

        if let Some((x, y)) = self.position {
            attrs.push(format!("pos=\"{},{}!\"", x, y));
        }

        result.push_str(&format!(" [shape=point {}];", attrs.join(" ")));

        result
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DotEdge {
    source: String,
    target: String,
    parameters: Vec<String>,
}

impl DotEdge {
    pub fn make(source: &str, target: &str) -> DotEdge {
        let mut source = String::from(source);
        let mut target = String::from(target);

        if source > target {
            mem::swap(&mut source, &mut target);
        }

        DotEdge {
            source: source,
            target: target,
            parameters: vec![],
        }
    }

    pub fn add_parameter(&mut self, param: String) {
        self.parameters.push(param);
    }

    pub fn render(&self) -> String {
        format!("\"{}\" -- \"{}\" [{}];",
                self.source,
                self.target,
                self.parameters.join(" "))
    }
}

#[derive(Clone)]
pub struct DotGraph {
    nodes: Vec<DotNode>,
    edges: Vec<DotEdge>,
}

impl DotGraph {
    pub fn make() -> DotGraph {
        DotGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DotNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: DotEdge) {
        self.edges.push(edge);
    }

    pub fn render(&self) -> String {
        let mut result = String::new();

        {
            let mut add_line = |line: &str| {
                result.push_str(line);
                result.push_str("\n");
            };

            add_line("graph {");
            for node in self.nodes.clone() {
                add_line(&format!("    {}", node.render()));
            }
            for edge in self.edges.clone() {
                add_line(&format!("    {}", edge.render()));
            }
            add_line("}");
        }

        result
    }
}

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

pub fn name_nodes(maze: &mut Maze) -> Vec<Vec<String>> {
    let (size_x, size_y) = maze.size();
    let mut ret_vec = vec![];
    let mut push_val;
    for y in 0..size_y {
        let mut x_vec = vec![];
        for x in 0..size_x {
            push_val = format!("{}-{}", x, y);
            x_vec.push(push_val);
        }
        ret_vec.push(x_vec);
    }
    ret_vec
}

pub fn render_dot(maze: &mut Maze) -> String {
    let vec_ref = &name_nodes(maze);
    let subgraphs = subgraphs(maze);

    let mut graph = DotGraph::make();

    {
        let mut edges: Vec<DotEdge> = vec![];

        for (y, nodes) in subgraphs.iter().enumerate() {
            for (x, node) in nodes.iter().enumerate() {
                let node_name = &vec_ref[y][x];

                {
                    let mut dot_node = DotNode::make(node_name);
                    dot_node.set_position(x as i32, y as i32);
                    graph.add_node(dot_node);
                }

                let (x, y) = (x as i64, y as i64);

                let dirs = vec![(Direction::Up, y - 1, x),
                                (Direction::Down, y + 1, x),
                                (Direction::Left, y, x - 1),
                                (Direction::Right, y, x + 1)];

                for (dir, a, b) in dirs {
                    if node.has_no_wall(dir) {
                        let (a, b) = (a as usize, b as usize);
                        let neighbor_name = &vec_ref[a][b];
                        edges.push(DotEdge::make(&node_name, &neighbor_name));
                    }
                }
            }
        }

        edges.sort();

        edges.dedup();

        for edge in edges {
            graph.add_edge(edge);
        }
    }

    graph.render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_works() {
        let (x, y) = (8, 8);
        let mut maze = generate_maze(x, y);
        let maze_ref = &mut maze;
        assert_eq!(name_nodes(maze_ref)[0].len(), x);
        assert_eq!(name_nodes(maze_ref).len(), y);
        assert_eq!(subgraphs(maze_ref)[0].len(), x);
        assert_eq!(subgraphs(maze_ref).len(), y);
        println!("{}", render_dot(maze_ref));
    }
}
