advent_of_code::solution!(16);

const VERTICAL: u32 = 1;
const HORIZONTAL: u32 = 2;
const BACKSLASH: u32 = 4;
const SLASH: u32 = 8;
const EMPTY: u32 = 0;
const L_VISITED: u32 = 16;
const T_VISITED: u32 = 32;
const R_VISITED: u32 = 64;
const B_VISITED: u32 = 128;

const TL_VISITED: u32 = 48;
const TR_VISITED: u32 = 96;
const LB_VISITED: u32 = 144;
const RB_VISITED: u32 = 192;
const TB_VISITED: u32 = T_VISITED | B_VISITED;
const LR_VISITED: u32 = L_VISITED | R_VISITED;
const ALL_VISITED: u32 = TB_VISITED | LR_VISITED;

pub struct Maze {
    data: Vec<Vec<u32>>,
    xmax: usize,
    ymax: usize,
}


impl Maze {
    pub fn new(input: &str) -> Self {
        let mut data = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                match char {
                    '|' => row.push(VERTICAL),
                    '-' => row.push(HORIZONTAL),
                    '\\' => row.push(BACKSLASH),
                    '/' => row.push(SLASH),
                    '.' => row.push(EMPTY),
                    _ => panic!("Invalid char"),
                }
            }
            data.push(row);
        }
        let xmax = data.len() - 1;
        let ymax = data[0].len() - 1;

        Self {
            data,
            xmax,
            ymax,
        }
    }

    pub fn get_direction(&self, x: usize, y: usize) -> u32 {
        self.data[x][y] & 15
    }

    pub fn get_visited(&self, x: usize, y: usize) -> u32 {
        self.data[x][y] & 240
    }

    pub fn print_maze(&self) {
        for row in &self.data {
            for cell in row {
                if cell & VERTICAL == VERTICAL {
                    print!("|");
                } else if cell & HORIZONTAL == HORIZONTAL {
                    print!("-");
                } else if cell & BACKSLASH == BACKSLASH {
                    print!("\\");
                } else if cell & SLASH == SLASH {
                    print!("/");
                } else if cell == &0_u32 {
                    print!(".");
                } else {
                    let cell2 = cell & 240;
                    match cell2 {
                        T_VISITED => print!("V"),
                        B_VISITED => print!("^"),
                        R_VISITED => print!("<"),
                        L_VISITED => print!(">"),
                        TR_VISITED => print!("L"),
                        LB_VISITED => print!("7"),
                        TL_VISITED => print!("J"),
                        RB_VISITED => print!("F"),
                        TB_VISITED => print!("l"),
                        LR_VISITED => print!("="),
                        ALL_VISITED => print!("+"),

                        _ => print!("2"),
                    }
                }
            }
            print!("\n");
        }
    }

    pub fn count_visited(&self) -> u32 {
        let mut count = 0;
        for row in &self.data {
            for cell in row {
                if cell & 240 > 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn move_up(&mut self, x_orig: usize, y: usize) {
        if x_orig == 0 {
            return;
        }
        let x = x_orig - 1;
        if self.data[x][y] & B_VISITED == B_VISITED {
            return;
        }
        //println!("move up x: {}, y: {}", x, y);

        self.data[x_orig][y] |= T_VISITED;

        self.move_to_up(x, y);
    }

    fn move_to_up(&mut self, x: usize, y: usize) {
        self.data[x][y] |= B_VISITED;

        match self.get_direction(x, y) {
            HORIZONTAL => {
                self.move_left(x, y);
                self.move_right(x, y);
            }
            VERTICAL | EMPTY => {
                self.move_up(x, y);
            }
            BACKSLASH => {
                self.move_left(x, y);
            }
            SLASH => {
                self.move_right(x, y);
            }
            _ => (),
        }
    }

    fn move_down(&mut self, x_orig: usize, y: usize) {
        if x_orig == self.xmax {
            return;
        }
        let x = x_orig + 1;
        if self.data[x][y] & T_VISITED == T_VISITED {
            return;
        }
        //println!("move down x: {}, y: {}", x, y);
        self.data[x_orig][y] |= B_VISITED;

        self.move_to_down(x, y);
    }

    fn move_to_down(&mut self, x: usize, y: usize) {
        self.data[x][y] |= T_VISITED;

        match self.get_direction(x, y) {
            HORIZONTAL => {
                self.move_left(x, y);
                self.move_right(x, y);
            }
            VERTICAL | EMPTY => {
                self.move_down(x, y);
            }
            BACKSLASH => {
                self.move_right(x, y);
            }
            SLASH => {
                self.move_left(x, y);
            }
            _ => (),
        }
    }

    fn move_left(&mut self, x: usize, y_orig: usize) {
        if y_orig == 0 {
            return;
        }
        let y = y_orig - 1;
        if self.data[x][y] & R_VISITED == R_VISITED {
            return;
        }

        self.data[x][y_orig] |= L_VISITED;

        self.move_to_left(x, y);
    }

    fn move_to_left(&mut self, x: usize, y: usize) {
        self.data[x][y] |= R_VISITED;
        //println!("move left x: {}, y: {}", x, y);

        match self.get_direction(x, y) {
            HORIZONTAL | EMPTY => {
                self.move_left(x, y);
            }
            VERTICAL => {
                self.move_up(x, y);
                self.move_down(x, y);
            }
            BACKSLASH => {
                self.move_up(x, y);
            }
            SLASH => {
                self.move_down(x, y);
            }
            _ => (),
        }
    }

    fn move_right(&mut self, x: usize, y_orig: usize) {
        if y_orig == self.ymax {
            return;
        }
        let y = y_orig + 1;
        if self.data[x][y] & L_VISITED == L_VISITED {
            return;
        }
        self.data[x][y_orig] |= R_VISITED;
        self.move_to_right(x, y);
    }

    fn move_to_right(&mut self, x: usize, y: usize) {
        self.data[x][y] |= L_VISITED;

        match self.get_direction(x, y) {
            HORIZONTAL | EMPTY => {
                self.move_right(x, y);
            }
            VERTICAL => {
                self.move_up(x, y);
                self.move_down(x, y);
            }
            BACKSLASH => {
                self.move_down(x, y);
            }
            SLASH => {
                self.move_up(x, y);
            }
            _ => (),
        }
    }

    pub fn clear_visited(&mut self) {
        for i in 0..self.xmax +1 {
            for j in 0..self.ymax +1 {
                self.data[i][j] &= 15;
            }
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut maze = Maze::new(input);
    maze.move_to_right(0, 0);
    return Some(maze.count_visited());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut maze = Maze::new(input);
    let mut max = 0;
    for i in 0..maze.xmax+1 {
        maze.clear_visited();
        maze.move_to_right(i, 0);
        max = max.max(maze.count_visited());
        maze.clear_visited();
        maze.move_to_left(i, maze.ymax);
        max = max.max(maze.count_visited());
    }
    for j in 0..maze.ymax+1 {
        maze.clear_visited();
        maze.move_to_down(0, j);
        max = max.max(maze.count_visited());
        maze.clear_visited();
        maze.move_to_up(maze.xmax, j);
        max = max.max(maze.count_visited());
    }
    return Some(max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
