use std::collections::HashMap;
use std::collections::VecDeque;
use crate::ModuleKind::{Broadcaster, Conjunction, FlipFlop, Output};

advent_of_code::solution!(20);


struct PulseQueue {
    queue: VecDeque<Pulse>,
    count_high: usize,
    count_low: usize,
}

impl PulseQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            count_high: 0,
            count_low: 0,
        }
    }

    fn push(&mut self, pulse: Pulse) {
        if pulse.high {
            self.count_high += 1;
        } else {
            self.count_low += 1;
        }
        self.queue.push_back(pulse);
    }

    fn pop(&mut self) -> Option<Pulse> {
        self.queue.pop_front()
    }
}

struct Pulse {
    high: bool,
    dest: usize,
    orig: usize,
}

enum ModuleKind {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Output,
}

struct Module {
    kind: ModuleKind,
    dests: Vec<usize>,
    sources: HashMap<usize, bool>,
    state: bool,
}

impl Module {
    fn new(kind: ModuleKind, dests: Vec<usize>) -> Self {
        Self {
            kind,
            state: false,
            dests,
            sources: HashMap::new(),
        }
    }
    fn send_pulse(&self, emetteur: &Pulse, high: bool) -> Vec<Pulse> {
        self.dests.iter().map(|&dest| Pulse { orig: emetteur.dest, high, dest }).collect()
    }

    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        match self.kind {
            FlipFlop => self.handle_pulse_flip_flop(pulse),
            Conjunction => self.handle_pulse_conjunction(pulse),
            Broadcaster => self.handle_pulse_broadcaster(pulse),
            Output => Vec::new(),
        }
    }
    fn handle_pulse_flip_flop(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.high {
            Vec::new()
        } else {
            self.state = !self.state;
            self.dests.iter().map(|&d| Pulse { orig: pulse.dest, high: self.state, dest: d }).collect()
        }
    }
    fn get_dests(&self) -> &Vec<usize> {
        &self.dests
    }

    fn handle_pulse_conjunction(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.sources.insert(pulse.orig, pulse.high);
        let has_false = self.sources.iter().any(|(_, high)| !high);
        self.send_pulse(&pulse, has_false)
    }
    fn update_references(&mut self, sources: &Vec<usize>) {
        for source in sources {
            self.sources.insert(*source, false);
        }
    }
    fn handle_pulse_broadcaster(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.dests.iter().map(|&dest| Pulse { orig: pulse.dest, high: pulse.high, dest }).collect()
    }
}

struct Propagator {
    modules: HashMap<usize, Module>,
    module_names: HashMap<String, usize>,
    number_of_runs: u64,
    index_36: u64,
    index_58 : u64,
    index_16 : u64,
    index_19 : u64,
}

impl Propagator {
    fn new() -> Self {
        let mut m_names = HashMap::new();
        m_names.insert("button".to_string(), 0);
        m_names.insert("output".to_string(), 1);
        m_names.insert("rx".to_string(), 2);
        Self {
            modules: HashMap::new(),
            module_names: m_names,
            number_of_runs: 0,
            index_36: 0,
            index_58: 0,
            index_16: 0,
            index_19: 0,
        }
    }
    fn parse_input(&mut self, input: &str) {
        self.modules.insert(2, Module::new(Output,  Vec::new()));
        for line in input.lines() {
            let (f_part, dest_str) = line.split_at(line.find(" -> ").unwrap());
            let module_id = self.get_module_id(&f_part[1..]);
            let dests = self.get_dests(&dest_str[4..]);
            match f_part.chars().next().unwrap() {
                '%' => {
                    self.modules.insert(module_id, Module::new(FlipFlop, dests));
                }
                '&' => {
                    self.modules.insert(module_id, Module::new(Conjunction, dests));
                }
                'b' => {
                    self.modules.insert(module_id, Module::new(Broadcaster, dests));
                }
                _ => panic!("Invalid line"),
            }
        }
        let mut data: HashMap<usize, Vec<usize>> = HashMap::new();
        let keys: Vec<usize> = self.modules.keys().copied().collect();
        for id in keys.clone() {
            let module = self.modules.get_mut(&id).unwrap();
            for dest in module.get_dests() {
                let sources = data.entry(*dest).or_insert(Vec::new());
                sources.push(id);
            }
        }
        self.update_references(data);
    }


    fn update_references(&mut self, sources: HashMap<usize, Vec<usize>>) {
        for (id, sources) in sources {
            if id < 3 {
                continue;
            }
            let toto = self.modules.get_mut(&id);
            if toto.is_some() {
                toto.unwrap().update_references(&sources);
            }
        }
    }

    fn get_dests(&mut self, input: &str) -> Vec<usize> {
        let mut dests = Vec::new();
        for word in input.split(",") {
            dests.push(self.get_module_id(word.trim()));
        }
        dests
    }

    fn get_module_id(&mut self, name: &str) -> usize {
        self.module_names.get(name)
            .copied().unwrap_or_else(|| {
            let val = self.module_names.len();
            self.module_names.insert(name.to_string(), val);
            val
        })
    }

    fn part_one(&mut self) -> u64 {
        let mut low_tot: u64 = 0;
        let mut high_tot: u64 = 0;
        for _ in 0..1000 {
            let (low, high) = self.run_one();
            low_tot += low as u64;
            high_tot += high as u64;
        }
        return high_tot * low_tot;
    }

    fn part_two(&mut self) -> u64 {
        loop {
            self.run_one();
            if self.index_36 != 0 && self.index_58 != 0 && self.index_16 != 0 && self.index_19 != 0 {
                break;
            }
        }
        return lcm(self.index_36, lcm(self.index_58, lcm(self.index_16, self.index_19)));
    }

    fn run_one(&mut self) -> (usize, usize) {
        self.number_of_runs += 1;
        let mut queue = PulseQueue::new();
        queue.push(Pulse { orig: 0, high: false, dest: self.get_module_id("roadcaster") });
        while let Some(pulse) = queue.pop() {
            if pulse.dest == 20 && pulse.high {
                match pulse.orig {
                    36 => {
                        if self.index_36 == 0 {
                            self.index_36 = self.number_of_runs ;
                        }
                    }
                    58 => {
                        if self.index_58 == 0 {
                            self.index_58 = self.number_of_runs ;
                        }
                    }
                    16 => {
                        if self.index_16 == 0 {
                            self.index_16 = self.number_of_runs ;
                        }
                    }
                    19 => {
                        if self.index_19 == 0 {
                            self.index_19 = self.number_of_runs ;
                        }
                    }
                    _ => (),
                }
            }
            if pulse.dest == 1 {
                continue;
            }
            if let Some(module) = self.modules.get_mut(&pulse.dest) {
                let pulses = module.handle_pulse(pulse);
                for d in pulses {
                    queue.push(d);
                }
            } else {
                if !pulse.high {
                    return (0, 0);
                }
            }
        }
        (queue.count_low, queue.count_high)
    }

}


fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}


pub fn part_one(input: &str) -> Option<u64> {
    let mut propagator = Propagator::new();
    propagator.parse_input(input);
    Some(propagator.part_one())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut propagator = Propagator::new();
    propagator.parse_input(input);
    Some(propagator.part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_oneb() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(11687500));
    }

}
