extern crate mazes;
use mazes::maze::generate_maze;
use mazes::dot::dots;

fn main() {
    let (x, y) = (4, 4);
    let mut maze = generate_maze(x, y);
    let maze_ref = &mut maze;
    let lines = dots(maze_ref);
    print!("{}", lines);
}
