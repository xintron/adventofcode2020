use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    operation: Operation,
    argument: isize,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let d: Vec<&str> = line.splitn(2, ' ').collect();
            let argument = d[1].parse::<isize>().unwrap();
            let operation = match d[0] {
                "nop" => Operation::Nop,
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                _ => unreachable!(),
            };
            Instruction {
                operation: operation,
                argument: argument,
            }
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day8, part1)]
pub fn part1_accumulator(instructions: &Vec<Instruction>) -> isize {
    let (_, acc) = accumulator(instructions);
    acc
}

/// Runs the instruction set and returns if it terminated early (true) and the
/// accumulated value
fn accumulator(instructions: &Vec<Instruction>) -> (bool, isize) {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut i: isize = 0;
    let mut acc: isize = 0;
    loop {
        // If this already exists, break the loop
        if !visited.insert(i as usize) {
            return (true, acc);
        }
        let instruction = &instructions[i as usize];
        match instruction.operation {
            Operation::Nop => i += 1,
            Operation::Acc => {
                i += 1;
                acc += instruction.argument;
            }
            Operation::Jmp => i += instruction.argument,
        }

        // End reached, return early-termination: false and the accumulated
        // value.
        // This needs to happen after the instruction has been executed above to
        // correctly accumulate the last instruction.
        if (i) as usize >= instructions.len() {
            return (false, acc);
        }
    }
}

#[aoc(day8, part2)]
pub fn part2_operation_fix(data: &Vec<Instruction>) -> isize {
    let mut offset: usize = 0;
    loop {
        let mut instructions: Vec<Instruction> = data.to_vec();
        let (index, mut instruction): (usize, &mut Instruction) = instructions
            .iter_mut()
            .rev()
            .enumerate()
            .filter(|(i, v)| {
                // index must be after the last offest (item modified).
                *i >= offset && (v.operation == Operation::Nop || v.operation == Operation::Jmp)
            })
            .next()
            .unwrap();
        // +1 to not modify the same instruction each loop. We want to avoid infinite loops, not help create them :)
        offset = index + 1;
        instruction.operation = match instruction.operation {
            Operation::Nop => Operation::Jmp,
            Operation::Jmp => Operation::Nop,
            _ => unreachable!(),
        };
        let (early, acc) = accumulator(&instructions);
        if !early {
            return acc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE_INPUT_P1: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    pub fn test_day8_parse_input() {
        let instructions = input_generator(SAMPLE_INPUT_P1);
        assert_eq!(
            instructions[0],
            Instruction {
                operation: Operation::Nop,
                argument: 0
            }
        );
        assert_eq!(
            instructions[1],
            Instruction {
                operation: Operation::Acc,
                argument: 1
            }
        );
        assert_eq!(
            instructions[2],
            Instruction {
                operation: Operation::Jmp,
                argument: 4
            }
        );
    }

    #[test]
    pub fn test_day8_part1_accumulator() {
        let instructions = input_generator(SAMPLE_INPUT_P1);
        assert_eq!(part1_accumulator(&instructions), 5);
    }

    #[test]
    pub fn test_day8_part2_operation_fix() {
        let instructions = input_generator(SAMPLE_INPUT_P1);
        assert_eq!(part2_operation_fix(&instructions), 8);
    }
}
