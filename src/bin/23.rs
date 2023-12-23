advent_of_code::solution!(23);

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use advent_of_code::{Coord, Direction};

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Path {
    start: Coord,
    end: Coord,
    distance: usize,
}


struct Maze {
    maze: Vec<Vec<char>>,
    pathes: Vec<Path>,
    slope: bool,
}

const PATHS: char = '.';
const WALL: char = '#';
const SLOPE_NORTH: char = '^';
const SLOPE_SOUTH: char = 'v';
const SLOPE_WEST: char = '<';
const SLOPE_EAST: char = '>';


impl Maze {
    pub fn new(slope: bool) -> Self {
        Self {
            maze: Vec::new(),
            pathes: Vec::new(),
            slope,
        }
    }

    pub fn read_input(&mut self, input: &str) {
        for row in input.lines() {
            self.maze.push(row.chars().collect());
        }
        for i in 0..self.maze.len() {
            if self.maze[0][i] == PATHS {
                self.find_pathes(Coord(0, i as isize));
            }
        }
    }

    pub fn find_pathes(&mut self, start: Coord) {
        let mut start_positions = VecDeque::from([start]);
        let mut handled_positions = Vec::new();
        while let Some(depart) = start_positions.pop_front() {
            if handled_positions.contains(&depart) {
                continue;
            }
            handled_positions.push(depart);
            for direction in Direction::get_all() {
                if let Some(path) = self.follow_path(depart, direction) {
                    self.pathes.push(path);
                    if path.end.0 == self.maze.len() as isize - 1 {
                        continue;
                    }
                    start_positions.push_back(path.end);
                }
            }
        }
    }

    pub fn follow_path(&mut self, start: Coord, direction: Direction) -> Option<Path> {
        let mut current_position = start.clone();
        let mut current_direction = direction;
        let mut distance = 0;
        while self.is_valid_plot(current_position, current_direction) {
            current_position = current_position.go(current_direction);
            distance += 1;
            let mut possibilities = Vec::new();
            for next_direction in Direction::get_all() {
                if next_direction != current_direction.get_opposite() && self.is_valid_plot(current_position, next_direction) {
                    possibilities.push(next_direction);
                }
            }
            if possibilities.len() == 0 {
                return None;
            } else if possibilities.len() == 1 {
                let next_position = current_position.go(possibilities[0]);
                if next_position.0 == self.maze.len() as isize - 1 {
                    return Some(Path {
                        start,
                        end: next_position,
                        distance: distance + 1,
                    });
                }
                current_direction = possibilities[0];
            } else {
                return Some(Path {
                    start,
                    end: current_position,
                    distance,
                });
            }
        }
        None
    }

    pub fn is_valid_plot(&self, origin: Coord, direction: Direction) -> bool {
        let coord = origin.go(direction);
        if coord.0 < 0 || coord.1 < 0 {
            return false;
        }
        if coord.0 >= self.maze.len() as isize || coord.1 >= self.maze[0].len() as isize {
            return false;
        }
        let c = self.maze[coord.0 as usize][coord.1 as usize];
        if c == WALL {
            return false;
        }
        if !self.slope {
            return true;
        }
        let o = self.maze[origin.0 as usize][origin.1 as usize];
        match o {
            SLOPE_NORTH => { if direction != Direction::Up { return false; } },
            SLOPE_SOUTH => { if direction != Direction::Down { return false; } },
            SLOPE_WEST => { if direction != Direction::Left { return false; } },
            SLOPE_EAST => { if direction != Direction::Right { return false; } },
            _ => {},
        }
        match direction {
            Direction::Up => c != SLOPE_SOUTH,
            Direction::Down => c != SLOPE_NORTH,
            Direction::Left => c != SLOPE_EAST,
            Direction::Right => c != SLOPE_WEST,
        }
    }

    pub fn solve_part1(&self) -> usize {
        let mut max_distance = 0;
        let mut heap: BinaryHeap<Chemin> = BinaryHeap::new();
        heap.push(Chemin::new(self.pathes[0]));
        let mut best_distances =  HashMap::new();

        while let Some(chemin) = heap.pop() {
            for path in self.pathes.iter().filter(|&x| x.start == chemin.end) {
                let mut new_chemin = chemin.clone();
                if !new_chemin.add_path(*path) {
                    continue;
                }
                let best_distance = best_distances.entry(path.end).or_insert(0);
                if *best_distance > new_chemin.distance {
                    continue;
                } else {
                    *best_distance = new_chemin.distance;
                }
                if new_chemin.end.0 == self.maze.len() as isize - 1 {
                    max_distance = std::cmp::max(max_distance, new_chemin.distance);
                    continue;
                }
                heap.push(new_chemin);
            }
        }
        max_distance
    }
    pub fn solve_part2(&self) -> usize {
        let mut max_distance = 0;
        let mut heap: BinaryHeap<Chemin> = BinaryHeap::new();
        heap.push(Chemin::new(self.pathes[0]));
        let mut best_distances =  HashMap::new();

        while let Some(chemin) = heap.pop() {
            for path in self.pathes.iter().filter(|&x| x.start == chemin.end) {
                let mut new_chemin = chemin.clone();
                if !new_chemin.add_path(*path) {
                    continue;
                }
                let best_for_end = best_distances.entry(path.end).or_insert(HashMap::new());
                let best_for_start = best_for_end.entry(path.start).or_insert(0);
                if *best_for_start > new_chemin.distance {
                    continue;
                } else {
                    *best_for_start = new_chemin.distance;
                }
                if new_chemin.end.0 == self.maze.len() as isize - 1 {
                    max_distance = std::cmp::max(max_distance, new_chemin.distance);
                    continue;
                }
                heap.push(new_chemin);
            }
        }
        max_distance
    }
}

#[derive( Clone, Eq, PartialEq)]
struct Chemin {
    pathes: Vec<Path>,
    distance: usize,
    end: Coord,
}

impl Chemin {
    fn new(path: Path) -> Self {
        Self {
            pathes: Vec::from([path]),
            distance: path.distance,
            end: path.end,
        }
    }

    fn add_path(&mut self, path: Path) -> bool {
        if self.pathes.iter().filter(|&x| x.end == path.end || x.start == path.end ).count() > 0 {
            return false;
        }
        self.pathes.push(path);
        self.distance += path.distance;
        self.end = path.end;
        true
    }

}

impl Ord for Chemin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Chemin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = Maze::new(true);
    maze.read_input(input);
    Some(maze.solve_part1())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maze = Maze::new(false);
    maze.read_input(input);
    // low 5538
    Some(maze.solve_part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }

    #[test]
    fn test_one_r() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2402));
    }

    #[test]
    fn test_two_r() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6450));
    }

}
