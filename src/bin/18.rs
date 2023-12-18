advent_of_code::solution!(18);

enum Angle {
    HautGauche,
    HautDroite,
    BasGauche,
    BasDroite,
}


struct MazeContructor {
    sommets: Vec<(i64, i64)>,
    last_x: i64,
    last_y: i64,
    current_angle: Angle,
}

impl MazeContructor {
    pub fn new() -> Self {
        Self {
            sommets: vec![(0, 0)],
            last_x: 0,
            last_y: 0,
            current_angle: Angle::HautGauche,
        }
    }

    fn add_trou(&mut self, direction: &str, distance: i64) {
        match direction {
            "R" => {
                match self.current_angle {
                    Angle::HautGauche => {
                        self.last_y += distance + 1;
                    }
                    Angle::HautDroite => {
                        self.last_y += distance;
                    }
                    Angle::BasDroite => {
                        self.sommets.pop();
                        self.last_x -= 1;
                        self.sommets.push((self.last_x, self.last_y));
                        self.last_y += distance;
                    }
                    Angle::BasGauche => {
                        panic!("Invalid direction");
                    }
                }
                self.current_angle = Angle::HautDroite;
                self.sommets.push((self.last_x, self.last_y));
            }
            "L" => {
                match self.current_angle {
                    Angle::HautGauche => {
                        self.sommets.pop();
                        self.last_x += 1;
                        self.sommets.push((self.last_x, self.last_y));
                        self.last_y -= distance;
                    }
                    Angle::HautDroite => {
                        panic!("Invalid direction");
                    }
                    Angle::BasDroite => {
                        self.last_y -= distance + 1;
                    }
                    Angle::BasGauche => {
                        self.last_y -= distance;
                    }
                }
                self.current_angle = Angle::BasGauche;
                self.sommets.push((self.last_x, self.last_y));
            }
            "U" => {
                match self.current_angle {
                    Angle::HautGauche => {
                        self.last_x -= distance;
                    }
                    Angle::HautDroite => {
                        self.sommets.pop();
                        self.last_y -= 1;
                        self.sommets.push((self.last_x, self.last_y));
                        self.last_x -= distance;
                    }
                    Angle::BasDroite => {
                        panic!("Invalid direction");
                    }
                    Angle::BasGauche => {
                        self.last_x -= distance + 1;
                    }
                }
                self.current_angle = Angle::HautGauche;
                self.sommets.push((self.last_x, self.last_y));
            }
            "D" => {
                match self.current_angle {
                    Angle::HautGauche => {
                        panic!("Invalid direction");
                    }
                    Angle::HautDroite => {
                        self.last_x += distance + 1;
                    }
                    Angle::BasDroite => {
                        self.last_x += distance;
                    }
                    Angle::BasGauche => {
                        self.sommets.pop();
                        self.last_y += 1;
                        self.sommets.push((self.last_x, self.last_y));
                        self.last_x += distance;
                    }
                }
                self.current_angle = Angle::BasDroite;
                self.sommets.push((self.last_x, self.last_y));
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn read_input_part1(&mut self, input: &str) {
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                let distance = parts[1].parse::<i64>().unwrap();
                self.add_trou(parts[0], distance);
            }
        }
    }

    fn read_input_part2(&mut self, input: &str) {
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let color_code = &parts[2][2..7]; // Get the 5 characters after #
            let distance = i64::from_str_radix(color_code, 16).unwrap(); // Convert from base 16 to base 10
            let direction = parts[2].chars().nth(7).unwrap(); // Get the 6th character
            match direction {
                '0' => { self.add_trou("R", distance); }
                '1' => { self.add_trou("D", distance); }
                '2' => { self.add_trou("L", distance); }
                '3' => { self.add_trou("U", distance); }
                _ => panic!("Invalid direction"),
            }
        }
    }

    fn count(&self) -> f64 {
        let len = self.sommets.len();
        let mut sum = 0_f64;
        for i in 0..len {
            let (x1, y1) = self.sommets[i];
            let (x2, y2) = self.sommets[(i + 1) % len];
            sum += ((x1 * y2) - (x2 * y1)) as f64;
        }
        sum.abs() / 2.0
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut maze_constructor = MazeContructor::new();
    maze_constructor.read_input_part1(input);
    let maze = maze_constructor.count();
    Some(maze as u64)
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut maze_constructor = MazeContructor::new();
    maze_constructor.read_input_part2(input);
    let maze = maze_constructor.count();
    Some(maze as u64)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }

}