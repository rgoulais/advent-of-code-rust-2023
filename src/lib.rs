pub mod template;



#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Coord (pub isize, pub isize);

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}


impl Coord {
    pub fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Coord(self.0 - 1, self.1),
            Direction::Down => Coord(self.0 + 1, self.1),
            Direction::Left => Coord(self.0, self.1 - 1),
            Direction::Right => Coord(self.0, self.1 + 1),
        }
    }

    pub fn get_four_directions(&self) -> Vec<Self> {
        vec![
            self.go(Direction::Up),
            self.go(Direction::Down),
            self.go(Direction::Left),
            self.go(Direction::Right),
        ]
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    pub fn get_all() -> Vec<Self> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
}
