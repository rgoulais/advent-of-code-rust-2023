advent_of_code::solution!(17);


use std::collections::BinaryHeap;
use std::cmp::Ordering;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u16,
    position: usize,
    direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    data: Vec<u16>,
    xlen: usize,
    ylen: usize,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut data = Vec::new();
        let mut ylen: usize = 0;
        let mut xlen: usize = 0;
        for line in input.lines() {
            for char in line.chars() {
                data.push(char.to_string().parse::<u16>().unwrap_or(u16::MAX));
            }
            xlen += 1;
            if ylen == 0 {
                ylen = line.len();
            }
        }

        Self {
            data,
            xlen,
            ylen,
        }
    }

    pub fn deplace(&self, direction: usize, position: usize) -> Option<usize> {
        let (x, y) = (position / self.ylen, position % self.ylen);
        match direction {
            UP => if x > 0 { Some(position - self.ylen) } else { None },
            DOWN => if x < self.xlen - 1 { Some(position + self.ylen) } else { None },
            LEFT => if y > 0 { Some(position - 1) } else { None },
            RIGHT => if y < self.ylen - 1 { Some(position + 1) } else { None },
            _ => panic!("Invalid direction"),
        }
    }
}

fn get_new_direction(direction: usize) -> [usize; 2] {
    match direction {
        UP | DOWN => [LEFT, RIGHT],
        LEFT | RIGHT => [UP, DOWN],
        _ => panic!("Invalid direction"),
    }
}

pub fn find_path(input: &str, pas_min: usize, pas_max: usize) -> Option<u16> {
    let grid = Maze::new(input);
    let ecart = pas_max - pas_min;

    let mut dist = [
        vec![vec![u16::MAX; grid.data.len() * 3 * 4]; ecart],
        vec![vec![u16::MAX; grid.data.len() * 3 * 4]; ecart],
        vec![vec![u16::MAX; grid.data.len() * 3 * 4]; ecart],
        vec![vec![u16::MAX; grid.data.len() * 3 * 4]; ecart]
    ];
    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, position: 0, direction: RIGHT });
    heap.push(State { cost: 0, position: 0, direction: DOWN });

    'prochain: while let Some(state) = heap.pop() {
        let mut next_position = state.position;
        let mut next_cost = state.cost;
        for _ in 0..pas_min {
            next_position = match grid.deplace(state.direction, next_position) {
                Some(pos) => pos,
                None => continue 'prochain,
            };
            next_cost = next_cost + grid.data[next_position];
        }
        for i in 1..(1 + ecart) {
            next_position = match grid.deplace(state.direction, next_position) {
                Some(pos) => pos,
                None => continue 'prochain,
            };
            next_cost = next_cost + grid.data[next_position];
            if next_cost < dist[state.direction][i - 1][next_position] {
                for new_direction in get_new_direction(state.direction) {
                    heap.push(State { cost: next_cost, position: next_position, direction: new_direction });
                }
                dist[state.direction][i - 1][next_position] = next_cost;
            }
        }
    }
    let j = grid.data.len() - 1;
    let mut minval = u16::MAX;
    for i in 0..ecart {
        for k in 0..4 {
            minval = minval.min(dist[k][i][j]);
        }
    }
    Some(minval)
}

pub fn part_one(input: &str) -> Option<u16> {
    find_path(input, 0, 3)
}

pub fn part_two(input: &str) -> Option<u16> {
    find_path(input, 3, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let grid = Maze::new(&input);

        let mut pos = 0_usize;
        assert_eq!(grid.data[pos], 2);
        pos = grid.deplace(DOWN, pos).unwrap();
        assert_eq!(grid.data[pos], 3);
        pos = grid.deplace(RIGHT, pos).unwrap();
        assert_eq!(grid.data[pos], 2);
        pos = grid.deplace(UP, pos).unwrap();
        assert_eq!(grid.data[pos], 4);
        pos = grid.deplace(LEFT, pos).unwrap();
        assert_eq!(grid.data[pos], 2);
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

}
