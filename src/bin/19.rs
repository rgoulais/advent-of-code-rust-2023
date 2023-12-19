advent_of_code::solution!(19);

const X: u8 = 'x' as u8;
const M: u8 = 'm' as u8;
const A: u8 = 'a' as u8;
const S: u8 = 's' as u8;

const GREATER: u8 = '>' as u8;
const LESSER: u8 = '<' as u8;

const START_VALUE: (u32,u32) = (1, 4000);

struct Instruction {
    attribut: u8,
    condition: u8,
    value: u32,
    action: String,
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(':').collect();
        let action = parts.last().unwrap().to_string();
        if parts[0].contains('>') || parts[0].contains('<') {
            let bytes = parts[0].as_bytes();
            let attribut = match bytes[0] {
                X => X,
                M => M,
                A => A,
                S => S,
                _ => panic!("Unknown attribut"),
            };
            let condition = match bytes[1] {
                GREATER => GREATER,
                LESSER => LESSER,
                _ => panic!("Unknown condition"),
            };
            let value = parts[0][2..].parse().unwrap();
            Self { attribut, condition, value, action }
        } else {
            Self { attribut: 0, condition: 0, value: 0, action }
        }
    }

    pub fn correspond(&self, part: &mut Part) -> bool {
        if self.attribut == 0 {
            part.label = self.action.clone();
            return true;
        }
        let part_value = match self.attribut {
            X => part.x,
            M => part.m,
            A => part.a,
            S => part.s,
            _ => panic!("Unknown field: {}", self.attribut),
        };
        let condition_satisfied = match self.condition {
            GREATER => part_value > self.value,
            LESSER => part_value < self.value,
            _ => panic!("Unknown operator: {}", self.condition),
        };
        if condition_satisfied {
            part.label = self.action.clone();
        }
        return condition_satisfied;
    }

    fn check_ranges(&self, range: &Ranges) -> Option<(Ranges, Ranges)> {
        let mut match_range = range.clone();
        match_range.label = self.action.clone();
        if self.attribut == 0 {
            return Some((match_range, Ranges::get_invalid()));
        }
        let mut match_range = range.clone();
        match_range.label = self.action.clone();
        let mut left_range = range.clone();
        let mut found = false;
        match self.attribut {
            X => {
                if self.condition == GREATER {
                    if self.value > range.x.0 {
                        match_range.x.0 = self.value + 1;
                        left_range.x.1 = self.value;
                        found = true;
                    }
                } else {
                    if self.value < range.x.1 {
                        match_range.x.1 = self.value - 1;
                        left_range.x.0 = self.value;
                        found = true;
                    }
                }
            }
            M => {
                if self.condition == GREATER {
                    if self.value > range.m.0 {
                        match_range.m.0 = self.value + 1;
                        left_range.m.1 = self.value;
                        found = true;
                    }
                } else {
                    if self.value < range.m.1 {
                        match_range.m.1 = self.value - 1;
                        left_range.m.0 = self.value;
                        found = true;
                    }
                }
            }
            A => {
                if self.condition == GREATER {
                    if self.value > range.a.0 {
                        match_range.a.0 = self.value + 1;
                        left_range.a.1 = self.value;
                        found = true;
                    }
                } else {
                    if self.value < range.a.1 {
                        match_range.a.1 = self.value - 1;
                        left_range.a.0 = self.value;
                        found = true;
                    }
                }
            }
            S => {
                if self.condition == GREATER {
                    if self.value > range.s.0 {
                        match_range.s.0 = self.value + 1;
                        left_range.s.1 = self.value;
                        found = true;
                    }
                } else {
                    if self.value < range.s.1 {
                        match_range.s.1 = self.value - 1;
                        left_range.s.0 = self.value;
                        found = true;
                    }
                }
            }
            _ => panic!("Unknown field: {}", self.attribut),
        }
        if found {
            return Some((match_range, left_range));
        } else {
            return None;
        }
    }
}


struct Workflow {
    label: String,
    actions: Vec<Instruction>,
}

struct Part {
    label: String,
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Clone)]
struct Ranges {
    label: String,
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl Ranges {
    fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1 && self.label != "R"
    }
    fn get_invalid() -> Self {
        Self { label: "R".to_string(), x: (1, 0), m: (0, 0), a: (0, 0), s: (0, 0) }
    }

    fn combinaisons(&self) -> u64 {
        if !self.is_valid() {
            return 0;
        }
        let mut combinaisons = 1;
        combinaisons *= self.x.1 as u64 - self.x.0 as u64 + 1;
        combinaisons *= self.m.1 as u64 - self.m.0 as u64 + 1;
        combinaisons *= self.a.1 as u64 - self.a.0 as u64 + 1;
        combinaisons *= self.s.1 as u64 - self.s.0 as u64 + 1;
        combinaisons
    }

}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

struct Solver {
    instructions: Vec<Workflow>,
    parts: Vec<Part>,
}


impl Solver {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            parts: Vec::new(),
        }
    }

    pub fn solve_part1(&mut self, input: &str) -> Option<u32> {
        self.parse_input(input);
        Some(self.process_parts())
    }

    pub fn solve_part2(&mut self, input: &str) -> Option<u64> {
        self.parse_input(input);
        Some(self.find_ranges())
    }

    fn parse_input(&mut self, input: &str) {
        let mut is_instruction = true;

        for line in input.lines() {
            if line.trim().is_empty() {
                is_instruction = false;
                continue;
            }
            if is_instruction {
                let mut split = line.splitn(2, '{');
                let label = split.next().unwrap().to_string();
                let actions = split.next().unwrap().trim_end_matches('}').split(',').map(|s| Instruction::new(s)).collect();
                self.instructions.push(Workflow { label, actions });
            } else {
                let values: Vec<u32> = line.trim_start_matches('{').trim_end_matches('}').split(',').map(|s| s[2..].trim().parse().unwrap()).collect();
                let part = Part { x: values[0], m: values[1], a: values[2], s: values[3], label: "in".to_string() };
                self.parts.push(part);
            }
        }
    }

    fn process_parts(&mut self) -> u32 {
        let mut somme = 0;
        while let Some(mut part) = self.parts.pop() {
            let instruction = self.instructions.iter().find(|i| i.label == part.label).unwrap();
            for action in &instruction.actions {
                if action.correspond(&mut part) {
                    break;
                }
            }
            if part.label == "A" {
                somme += part.sum();
            } else if part.label != "R" {
                self.parts.push(part);
            }
        }
        somme
    }

    fn find_ranges(&self) -> u64 {
        let range = Ranges { label: "in".to_string(), x: START_VALUE, m: START_VALUE, a: START_VALUE, s: START_VALUE };
        let mut ranges = vec![range];
        let mut somme = 0;
        while let Some(range) = ranges.pop() {
            let instruction = self.instructions.iter().find(|i| i.label == range.label).unwrap();
            let mut current_range = range.clone();
            for action in &instruction.actions {
                if let Some((match_range, left_range)) = action.check_ranges(&current_range) {
                    if match_range.is_valid() {
                        if match_range.label == "A" {
                            somme += match_range.combinaisons();
                        } else {
                            ranges.push(match_range);
                        }
                    }
                    if left_range.is_valid() {
                            current_range = left_range;
                    } else {
                        break;
                    }
                }
            }

        }
        somme
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut solver = Solver::new();
    solver.solve_part1(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut solver = Solver::new();
    solver.solve_part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn combinaisons_calculates_correctly_for_valid_ranges() {
        let ranges = Ranges { label: "in".to_string(), x: (1, 3), m: (1, 2), a: (1, 2), s: (1, 2) };
        assert_eq!(ranges.combinaisons(), 24);
    }

    #[test]
    fn combinaisons_calculates_correctly_for_zero_ranges() {
        let ranges = Ranges { label: "in".to_string(), x: (0, 0), m: (0, 0), a: (0, 0), s: (0, 0) };
        assert_eq!(ranges.combinaisons(), 1);
    }

    #[test]
    fn combinaisons_calculates_correctly_for_single_value_ranges() {
        let ranges = Ranges { label: "in".to_string(), x: (1, 1), m: (1, 1), a: (1, 1), s: (1, 1) };
        assert_eq!(ranges.combinaisons(), 1);
    }

    #[test]
    fn combinaisons_calculates_correctly_for_invalid_ranges() {
        let ranges = Ranges { label: "in".to_string(), x: (2, 1), m: (2, 1), a: (2, 1), s: (2, 1) };
        assert_eq!(ranges.combinaisons(), 0);
    }
}