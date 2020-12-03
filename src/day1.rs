#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    for x in input {
        for y in input {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    for x in input {
        for y in input {
            for z in input {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!()
}
