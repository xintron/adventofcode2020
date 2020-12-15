#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(data: &Vec<usize>) -> usize {
    get_spoken(&data, 2020)
}

#[aoc(day15, part2)]
pub fn part2(data: &Vec<usize>) -> usize {
    get_spoken(&data, 30000000)
}

fn get_spoken(data: &Vec<usize>, number: usize) -> usize {
    let mut spoken: Vec<usize> = Vec::with_capacity(number);
    // Initialize everything with 0
    spoken.resize(number, 0);
    for (i, v) in data.iter().enumerate() {
        spoken[*v] = i + 1 as usize;
    }

    (data.len()..number).fold(*data.last().unwrap(), |last, index| {
        let prev = spoken[last];
        let current = match prev {
            0 => 0,
            _ => index - prev,
        };
        spoken[last] = index;
        current
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0,3,6";
        let data = input_generator(&input);
        assert_eq!(data, vec![0, 3, 6]);
        assert_eq!(part1(&data), 436);

        assert_eq!(part1(&input_generator("1,3,2")), 1);
        assert_eq!(part1(&input_generator("2,1,3")), 10);
        assert_eq!(part1(&input_generator("1,2,3")), 27);
        assert_eq!(part1(&input_generator("2,3,1")), 78);
        assert_eq!(part1(&input_generator("3,2,1")), 438);
        assert_eq!(part1(&input_generator("3,1,2")), 1836);
    }

    #[test]
    fn test_part2() {
        // Tests are slow in unoptimized+debug. Enable one by one to test.
        assert_eq!(part2(&input_generator("0,3,6")), 175594);
        //assert_eq!(part2(&input_generator("1,3,2")), 2578);
        //assert_eq!(part2(&input_generator("2,1,3")), 3544142);
        //assert_eq!(part2(&input_generator("1,2,3")), 261214);
        //assert_eq!(part2(&input_generator("2,3,1")), 6895259);
        //assert_eq!(part2(&input_generator("3,2,1")), 18);
        //assert_eq!(part2(&input_generator("3,1,2")), 362);
    }
}
