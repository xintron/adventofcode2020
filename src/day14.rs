use std::collections::HashMap;

#[derive(Clone)]
pub struct Instruction {
    mask: String,
    mem: Vec<(usize, u64)>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            mask: String::default(),
            mem: Vec::default(),
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::default();
    let mut ins: Instruction = Instruction::default();
    for line in input.lines() {
        let mut parts = line.split(" = ");
        let op = parts.next().unwrap();
        let value = parts.next().unwrap();
        if op == "mask" {
            if !ins.mask.is_empty() {
                instructions.push(ins);
            }
            ins = Instruction {
                mask: value.to_string(),
                ..Instruction::default()
            };
        } else {
            let registry: usize = op
                .chars()
                .filter(|&c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            ins.mem.push((registry, value.parse().unwrap()));
        }
    }
    instructions.push(ins);
    instructions
}

#[aoc(day14, part1)]
pub fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut memory = HashMap::default();
    for ins in instructions {
        run_instruction(&mut memory, &ins);
    }
    memory.values().sum::<u64>()
}

#[aoc(day14, part2)]
pub fn part2(instructions: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::default();
    for ins in instructions {
        let (set_bits, _) = get_masks(&ins);
        // Get the position of all the X's
        let ones: Vec<usize> = ins
            .mask
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| match c {
                'X' => Some(i),
                _ => None,
            })
            .collect();
        for (addr, value) in &ins.mem {
            // Start with all the bits set
            let mut addresses = vec![*addr as u64 | set_bits];
            for i in &ones {
                // For each set bit in &ones, create a variant with it set and
                // one with it off for each item in addresses
                addresses = addresses
                    .iter()
                    .flat_map(|a| vec![a & !(1 << i), a | (1 << i)])
                    .collect();
            }

            for a in &addresses {
                memory.insert(*a, *value);
            }
        }
    }
    memory.values().sum::<u64>()
}

fn get_masks(instruction: &Instruction) -> (u64, u64) {
    (
        u64::from_str_radix(&instruction.mask.replace("X", "1"), 2).unwrap(),
        u64::from_str_radix(&instruction.mask.replace("X", "0"), 2).unwrap(),
    )
}

fn run_instruction(memory: &mut HashMap<usize, u64>, instruction: &Instruction) {
    let (set_bits, clear_bits) = get_masks(&instruction);
    for (key, value) in &instruction.mem {
        memory.insert(*key, value & set_bits | clear_bits);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_input_generator() {
        let ins = input_generator(SAMPLE_INPUT);
        assert_eq!(ins.len(), 1);
        assert_eq!(
            ins[0].mask,
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()
        );
        assert_eq!(ins[0].mem, vec![(8, 11), (7, 101), (8, 0)]);
    }

    #[test]
    fn test_run_instruction() {
        let ins = &input_generator(SAMPLE_INPUT)[0];
        let mut mem = HashMap::default();
        run_instruction(&mut mem, ins);
        assert_eq!(mem.get(&7), Some(&101u64));
        assert_eq!(mem.get(&8), Some(&64u64));
    }

    #[test]
    fn test_part1() {
        let ins = &input_generator(SAMPLE_INPUT);
        assert_eq!(part1(ins), 165);
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let ins = &input_generator(input);
        assert_eq!(part2(ins), 208);
    }
}
