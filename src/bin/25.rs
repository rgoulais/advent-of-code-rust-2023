use std::collections::{ HashSet};
advent_of_code::solution!(25);

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Wire {
    c1: usize,
    c2: usize,
}


struct Solver {
    data: Vec<Wire>,
    group1: HashSet<usize>,
    group2: HashSet<usize>,
}


impl Solver {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            group1: HashSet::new(),
            group2: HashSet::new(),
        }
    }

    fn solve_part1(&mut self, input: &str) -> std::io::Result<usize> {
        let file = File::create("wires.dot")?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "graph wires {{")?;
        for line in input.lines() {
            let mut parts = line.split(":");
            writeln!(writer, "{} -- {{ {} }}", parts.next().unwrap(), parts.next().unwrap())?;
        }
        writeln!(writer, "}}")?;
        //println!("Wrote wires.dot .. Use neato -Tsvg wires.dot > wires.svg to generate a picture");
        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let lhs = parts.next().unwrap().strip_suffix(':').unwrap();
            let c1 = usize::from_str_radix(lhs, 36).unwrap();
            while let Some(c) = parts.next() {
                let c2 = usize::from_str_radix(c, 36).unwrap();
                let wire = Wire { c1, c2 };
                self.data.push(wire);
            }
        }
        //println!("Found {} wires", self.data.len());
        self.add_to_group_1(usize::from_str_radix("lms", 36).unwrap());
        self.add_to_group_2(usize::from_str_radix("tmc", 36).unwrap());
        return Ok(self.group1.len() * self.group2.len());
    }

    fn add_to_group_1(&mut self, val: usize) {
        if val == usize::from_str_radix("tmc", 36).unwrap() {
            return;
        }
        if val == usize::from_str_radix("nhg", 36).unwrap() {
            return;
        }
        if val == usize::from_str_radix("xnn", 36).unwrap() {
            return;
        }
        self.group1.insert(val);
        for i in 0..self.data.len() {
            let wire = self.data[i];
            if wire.c2 == val && !self.group1.contains(&wire.c1){
                self.add_to_group_1(wire.c1);
            } else if wire.c1 == val  && !self.group1.contains(&wire.c2){
                self.add_to_group_1(wire.c2);
            }
        }
    }

    fn add_to_group_2(&mut self, val: usize) {
        if val == usize::from_str_radix("txf", 36).unwrap() {
            return;
        }
        if val == usize::from_str_radix("jjn", 36).unwrap() {
            return;
        }
        if val == usize::from_str_radix("lms", 36).unwrap() {
            return;
        }
        self.group2.insert(val);
        for i in 0..self.data.len() {
            let wire = self.data[i];
            if wire.c2 == val && !self.group2.contains(&wire.c1){
                self.add_to_group_2(wire.c1);
            } else if wire.c1 == val && !self.group2.contains(&wire.c2){
                self.add_to_group_2(wire.c2);
            }
        }
    }

}

pub fn part_one(input: &str) -> Option<usize> {
    let mut solver = Solver::new();
    Some(solver.solve_part1(input).unwrap())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
}
