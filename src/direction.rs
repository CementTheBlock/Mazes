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
