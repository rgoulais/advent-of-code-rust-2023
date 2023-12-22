advent_of_code::solution!(21);

use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    position: (usize, usize),
    steps: usize,
}

pub struct Maze {
    data: Vec<Vec<Spot>>,
    xlen: usize,
    ylen: usize,
    first_position: (usize, usize),
    position_list: Vec<Position>,
}

enum Spot {
    Rock = 1,
    Plot = 2,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let mut data = Vec::new();
        let mut position = (0, 0);
        let mut first_position: (usize, usize) = (0, 0);
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                match char {
                    '#' => {
                        row.push(Spot::Rock);
                    }
                    '.' => row.push(Spot::Plot),
                    'S' => {
                        row.push(Spot::Plot);
                        first_position = position;
                    }
                    _ => panic!("Invalid char"),
                }
                position.1 += 1;
            }
            data.push(row);
            position.0 += 1;
            position.1 = 0;
        }
        let xmax = data.len();
        let ymax = data[0].len();
        Self {
            data,
            xlen: xmax,
            ylen: ymax,
            first_position,
            position_list: Vec::new(),
        }
    }

    fn is_valid_plot(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x >= self.xlen as i32 || y >= self.ylen as i32 {
            return false;
        }
        let real_x = (x) as usize;
        let real_y = (y) as usize;
        match self.data[real_x][real_y] {
            Spot::Plot => true,
            _ => false,
        }
    }

    fn get_four_directions(&self, position: &Position) -> Vec<Position> {
        let mut four_elements : Vec<Position> = Vec::new();
        let mut new_pos = position.clone();
        new_pos.steps += 1;
        new_pos.position.0 += 1;
        four_elements.push(new_pos.clone());
        new_pos.position.0 -= 2;
        four_elements.push(new_pos.clone());
        new_pos.position.0 += 1;
        new_pos.position.1 += 1;
        four_elements.push(new_pos.clone());
        new_pos.position.1 -= 2;
        four_elements.push(new_pos.clone());
        four_elements
    }

    fn move_elf(&mut self) {
        let mut pile: VecDeque<Position> = VecDeque::new();
        pile.push_back(Position { position: self.first_position, steps: 0 });
        while let Some(position) = pile.pop_front() {
            for position in self.get_four_directions(&position) {
                if self.is_valid_plot(position.position.0 as i32, position.position.1 as i32) {
                    if self.position_list.iter().filter(|&x| x.position == position.position).count() > 0 {
                        continue;
                    }
                    self.position_list.push(position);
                    pile.push_back(position);
                }
            }
        }
    }

    fn part1(&mut self) -> usize {
        self.position_list.iter().filter(|&x| x.steps < 65 && x.steps % 2 == 0).count()
    }

    fn part2(&mut self) -> usize{
        let n = 202300;
        let even_corners = self.position_list.iter().filter(|&x| x.steps % 2 == 0 && x.steps > 65).count();
        let odd_corners = self.position_list.iter().filter(|&x| x.steps % 2 == 1 && x.steps > 65).count();
        let even = self.position_list.iter().filter(|&k| k.steps % 2 == 1).count();
        let odd = self.position_list.iter().filter(|&k| k.steps % 2 == 0).count();
        (n + 1) * (n + 1) * even + n * n * odd - ((n + 1) * odd_corners) + (n * even_corners)
    }
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    maze.move_elf();
    Some(maze.part1())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    maze.move_elf();
    Some(maze.part2())
}

#[cfg(test)]
mod tests {


}
