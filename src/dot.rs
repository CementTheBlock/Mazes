use maze::*;
// use mazes::node::*;
use direction::*;

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
            push_val = format!("\"{}-{}\"", x, y);
            x_vec.push(push_val);
        }
        ret_vec.push(x_vec);
    }
    ret_vec
}

// let mut result = String::new();
// result.push_str("digraph {\n");
// for e in edges {
// let line = format!("  \"{}\" -> \"{}\" [{}];\n", e.from, e.to, e.meta);
// result.push_str(&line);
// }
// result.push_str("}\n");
// result

pub fn dots(maze: &mut Maze) -> String {
    let names_of_nodes = name_nodes(maze);
    let subgraphs = subgraphs(maze);
    let mut result = String::new();
    result.push_str("digraph {{\n");
    {
        let vec_ref = &names_of_nodes;
        for names in vec_ref {
            result.push_str("subgraph {{\nrank = same; ");
            for name in names {
                let line = format!("{}; ", name);
                result.push_str(&line);
            }
            result.push_str("\n}}\n");
        }
    }
    let mut x = 0;
    let mut y = 0;
    let mut line;
    {
        let vec_ref = &names_of_nodes;
        for nodes in subgraphs {
            let nodes_ref = &nodes;
            for names in vec_ref {
                for node in nodes_ref {
                    for name in names {
                        line = format!("{} -> {{", name);
                        result.push_str(&line);
                        if node.has_no_wall(Direction::Up) {
                            line = format!(" {}", vec_ref[y - 1][x]);
                            result.push_str(&line);
                        }
                        if node.has_no_wall(Direction::Down) {
                            line = format!(" {}", vec_ref[y - 1][x]);
                            result.push_str(&line);
                        }
                        if node.has_no_wall(Direction::Left) {
                            line = format!(" {}", vec_ref[y][x - 1]);
                            result.push_str(&line);
                        }
                        if node.has_no_wall(Direction::Right) {
                            line = format!(" {}", vec_ref[y][x + 1]);
                            result.push_str(&line);
                        }
                        result.push_str(" }}\n");
                        x += 1;
                    }
                }
                y += 1;
            }
        }
    }
    result.push_str("}}\n");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_works() {
        let (x, y) = (4, 4);
        let mut maze = generate_maze(x, y);
        let maze_ref = &mut maze;
        assert_eq!(subgraphs(maze_ref).len(), y);
    }
}
