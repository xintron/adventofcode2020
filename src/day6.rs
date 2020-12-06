use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|x| x.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|lines| {
            let sets: Vec<HashSet<char>> = lines
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect();
            match sets.len() {
                1 => sets[0].len(),
                _ => {
                    let sslice = &sets[1..];
                    sets[0]
                        .iter()
                        .filter(|&c| sslice.iter().all(|s| s.contains(c)))
                        .collect::<HashSet<&char>>()
                        .len()
                }
            }
        })
        .sum()
}
