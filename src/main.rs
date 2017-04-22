extern crate mazes;
use mazes::maze::generate_maze;
use mazes::dot::render_dot;

fn main() {
    let (x, y) = (8, 8);
    let mut maze = generate_maze(x, y);
    let maze_ref = &mut maze;
    let lines = render_dot(maze_ref);
    print!("{}", lines);
}
