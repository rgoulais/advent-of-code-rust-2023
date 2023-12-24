use std::collections::HashSet;
advent_of_code::solution!(24);

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul(&self, t: i64) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}


#[derive(Clone, Copy, Debug)]
struct Grelon {
    coordinates: Point,
    velocity: Point,
    a: f64,
    b: f64,
}

impl Grelon {
    pub fn new(coordinates: Point, velocity: Point) -> Self {
        let a = velocity.y as f64 / velocity.x as f64;
        let b = coordinates.y as f64 - a * coordinates.x as f64;
        Self { coordinates, velocity, a, b }
    }

    pub fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if let Some((x, y)) = intersection(self.a, self.b, other.a, other.b) {
            if self.velocity.x > 0 && x < self.coordinates.x as f64 {
                return None;
            }
            if self.velocity.x < 0 && x > self.coordinates.x as f64 {
                return None;
            }
            if other.velocity.x > 0 && x < other.coordinates.x as f64 {
                return None;
            }
            if other.velocity.x < 0 && x > other.coordinates.x as f64 {
                return None;
            }
            return Some((x, y));
        }
        None
    }


    pub fn position(&self, t: i64) -> Point {
        self.coordinates.add(&self.velocity.mul(t))
    }
}

fn intersection(a: f64, b: f64, c: f64, d: f64) -> Option<(f64, f64)> {
    if a == c {
        None // Les droites sont parallèles
    } else {
        let x = (d - b) / (a - c);
        let y = a * x + b;
        Some((x, y))
    }
}

struct Solver {
    data: Vec<Grelon>,
    y_velocity: i64,
    start_y: i64,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            y_velocity: 1,
            start_y: 13,
        }
    }

    pub fn read_input(&mut self, input: &str) {
        let mut unique_y = HashSet::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(|c| c == ',' || c == '@').collect();
            let x = parts[0].trim().parse::<i64>().unwrap();
            let y = parts[1].trim().parse::<i64>().unwrap();
            let z = parts[2].trim().parse::<i64>().unwrap();
            let vx = parts[3].trim().parse::<i64>().unwrap();
            let vy = parts[4].trim().parse::<i64>().unwrap();
            let vz = parts[5].trim().parse::<i64>().unwrap();
            self.data.push(Grelon::new(Point { x, y, z }, Point { x: vx, y: vy, z: vz }));
            if unique_y.contains(&(y, vy)) {
                self.start_y = y;
                self.y_velocity = vy;
            } else {
                unique_y.insert((y, vy));
            }
        }
    }

    pub fn solve_part1(&self, min_area: f64, max_area: f64) -> Option<u32> {
        let mut count = 0;
        for i in 0..self.data.len() {
            let grelon1 = &self.data[i];
            for j in i + 1..self.data.len() {
                let grelon2 = &self.data[j];
                if let Some((x, y)) = grelon1.intersect(grelon2) {
                    if x >= min_area && x <= max_area && y >= min_area && y <= max_area {
                        count += 1;
                    }
                }
            }
        }
        Some(count)
    }

    fn when_will_it_collide(&self, grelon: &Grelon) -> Option<i64> {
        if grelon.coordinates.y == self.start_y {
            return None;
        }
        let x = intersection(self.y_velocity as f64, self.start_y as f64, grelon.velocity.y as f64, grelon.coordinates.y as f64);
        if let Some((x, _)) = x {
            if x < 0.0 {
                return None;
            }
            if x.fract() != 0.0 {
                return None;
            }
            return Some(x as i64);
        } else {
            return None;
        }
    }


    pub fn solve_part2(&self) -> Option<i64> {
        let mut positions = Vec::new();
        while positions.len() < 2 {
            for grelon in &self.data {
                if let Some(time) = self.when_will_it_collide(grelon) {
                    let x = grelon.position(time).x;
                    let z = grelon.position(time).z;
                    positions.push((time, x, z));
                }
            }
        }
        let ecart_temps = positions[1].0 - positions[0].0;
        let ecart_x = positions[1].1 - positions[0].1;
        let ecart_z = positions[1].2 - positions[0].2;
        let velocity_x = ecart_x as f64 / ecart_temps as f64;
        let velocity_z = ecart_z as f64 / ecart_temps as f64;
        let x = positions[0].1 - (positions[0].0 as f64 * velocity_x) as i64;
        let z = positions[0].2 - (positions[0].0 as f64 * velocity_z) as i64;
        Some(x + z + self.start_y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut solver = Solver::new();
    solver.read_input(input);
    solver.solve_part1(200000000000000.0, 400000000000000.0)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut solver = Solver::new();
    solver.read_input(input);
    solver.solve_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let x = intersection(-1.0, 19.0, 1.0, 13.0);
        assert_eq!(x, Some((3.0, 16.0)));
        assert_eq!(intersection(-2.0, 25.0, 1.0, 13.0), Some((4.0, 17.0)));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut solver = Solver::new();
        solver.read_input(&input);
        let result = solver.solve_part1(7.0, 27.0);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut solver = Solver::new();
        solver.read_input(&input);
        let result = solver.solve_part2();
        assert_eq!(result, Some(47));
    }
}
