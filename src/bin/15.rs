advent_of_code::solution!(15);

use std::collections::HashMap;

pub struct Boxes {
    keys: Vec<String>,
    map: HashMap<String, u32>,
}

impl Boxes {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: u32) {
        if !self.map.contains_key(&key) {
            self.keys.push(key.clone());
        }
        self.map.insert(key, value);
    }

    pub fn update(&mut self, key: String, value: u32) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: &String) {
        self.keys.retain(|k| k != key);
        self.map.remove(key);
    }
}


fn hash_algo(input_string: &str) -> u32 {
    let mut current_value: u32 = 0;
    for char in input_string.chars() {
        let ascii_value = char as u32;
        current_value += ascii_value;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_value: u32 = 0;
    for line in input.split(",") {
        current_value += hash_algo(line);
    }
    Some(current_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_value: u32 = 0;
    let mut boxes = Boxes::new();
    for line in input.split(",") {
        if line.ends_with('-') {
            boxes.remove(&line[0..line.len() - 1].to_string());
        } else {
            let (key, value) = line.split_at(line.find('=').unwrap());
            boxes.insert(key.to_string(), value[1..].parse::<u32>().unwrap());
        }
    }
    let mut slots: HashMap<u32, u32> = HashMap::with_capacity(256);
    for key in boxes.keys {
        let val = boxes.map.get(&key).unwrap();
        let box_number = hash_algo(&key);
        let slot = slots.entry(box_number).or_insert(0);
        *slot += 1;
        current_value += (box_number + 1) * *slot * val;
    }
    Some(current_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
