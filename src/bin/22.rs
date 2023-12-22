advent_of_code::solution!(22);

use std::collections::{HashMap, HashSet, VecDeque};
use crate::utils::Coord;

mod utils;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Brique {
    x: Coord,
    y: Coord,
    z: Coord,
}

struct Mur {
    briques: HashMap<usize, Brique>,
    briques_en_dessous: HashMap<usize, Vec<usize>>,
    briques_au_dessus: HashMap<usize, Vec<usize>>,
    max_z: usize,
}

impl Mur {
    fn new(input: &str) -> Self {
        let mut briques = HashMap::new();
        let mut index = 0;
        let mut max_z = 0;
        for line in input.lines() {
            let mut parts = line.split(|c| c == '~' || c == ',');
            let x1 = parts.next().unwrap().parse().unwrap();
            let y1 = parts.next().unwrap().parse().unwrap();
            let z1 = parts.next().unwrap().parse().unwrap();
            let x2 = parts.next().unwrap().parse().unwrap();
            let y2 = parts.next().unwrap().parse().unwrap();
            let z2 = parts.next().unwrap().parse().unwrap();
            let brique = Brique {
                x: Coord(x1, x2),
                y: Coord(y1, y2),
                z: Coord(z1, z2),
            };
            briques.insert(index, brique);
            max_z = max_z.max(z1);
            max_z = max_z.max(z2);
            index += 1;
        };
        Self {
            briques,
            briques_en_dessous: HashMap::new(),
            briques_au_dessus: HashMap::new(),
            max_z
        }
    }

    fn move_down(&mut self) {
        let mut moved_briques = true;
        while moved_briques {
            moved_briques = false;
            let mut etage = 1;
            loop {
                let mut cases_prises: HashSet<(Coord, Coord)> = HashSet::new();
                for (_ , brique) in &self.briques {
                    if brique.z.0 <= etage && brique.z.1 >= etage {
                        cases_prises.insert((brique.x, brique.y));
                    }
                }
                for (_, brique) in &mut self.briques {
                    if brique.z.0 == etage + 1 {
                        if !intersect_horizontally(&brique, &cases_prises) {
                            brique.z.0 -= 1;
                            brique.z.1 -= 1;
                            moved_briques = true;
                        }
                    }
                }
                if etage > self.max_z {
                    break;
                }
                etage += 1;
            }
        }
        for (index, brique) in &self.briques {
            let supporting = self.find_supporting_bricks(&brique);
            let supported = self.find_supported_bricks(&brique);
            self.briques_en_dessous.insert(*index, supporting);
            self.briques_au_dessus.insert(*index, supported);
        }
    }


    fn find_supported_bricks(&self, brique: &Brique) -> Vec<usize> {
        let mut  res = Vec::new();
        for (index, brique2) in &self.briques {
            if brique2.z.0 == brique.z.1 + 1 &&
                intersect(&(brique2.x, brique2.y), &(brique.x, brique.y)) {
                res.push(*index);
            }
        }
        res
    }

    fn find_supporting_bricks(&self, brique: &Brique) -> Vec<usize> {
        let mut  res = Vec::new();
        for (index, brique2) in &self.briques {
            if brique2.z.1 == brique.z.0 -1 &&
                intersect(&(brique2.x, brique2.y), &(brique.x, brique.y)) {
                res.push(*index);
            }
        }
        res
    }

    fn count_movable(&self) -> usize {
        self.get_movable().len()
    }

    fn get_movable(&self) -> Vec<usize> {
        let mut res = Vec::new();
        for (index, _) in &self.briques {
            if self.briques_au_dessus[index].len() == 0 {
                res.push(*index);
            } else {
                let mut moveable = true;
                for index2 in &self.briques_au_dessus[&index.clone()] {
                    if self.briques_en_dessous[&index2].len() == 1 {
                        moveable = false;
                        break;
                    }
                }
                if moveable {
                    res.push(*index);
                }
            }
        }
        res
    }

    fn count_would_fall(&self) -> usize {
        let movables = self.get_movable();
        let mut count = 0;
        for (index, _) in &self.briques {
            if movables.contains(index) {
                continue;
            }
            let mut fallen = Vec::new();
            let mut pile = VecDeque::from(vec![*index]);
            while  let  Some(current_index) = pile.pop_front() {
                fallen.push(current_index);
                for i in self.briques_au_dessus[&current_index].iter() {
                    let mut fall = true;
                    for j in self.briques_en_dessous[i].iter() {
                        if !fallen.contains(j) {
                            fall = false;
                            break;
                        }
                    }
                    if fall {
                        pile.push_back(*i);
                    }
                }
            }
            count += fallen.len() - 1;
        }
        count
    }
}

fn intersect(coords1: &(Coord, Coord), coords2: &(Coord, Coord)) -> bool {
    if coords2.0.0 <= coords1.0.1 && coords2.0.1 >= coords1.0.0 && coords2.1.0 <= coords1.1.1 && coords2.1.1 >= coords1.1.0 {
        return true;
    }
    false
}

fn intersect_horizontally(brique1: &Brique, cases: &HashSet<(Coord, Coord)>) -> bool {
    for coords in cases {
        if intersect(&(brique1.x, brique1.y), coords) {
            return true;
        }
    }
    false
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut mur = Mur::new(input);
    mur.move_down();
    Some(mur.count_movable())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut mur = Mur::new(input);
    mur.move_down();
    Some(mur.count_would_fall())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
