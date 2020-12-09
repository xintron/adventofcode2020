#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|d| d.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(data: &Vec<usize>) -> usize {
    xmas_weakness_scanner(25, data)
}

#[aoc(day9, part2)]
pub fn part2(data: &Vec<usize>) -> usize {
    xmas_weakness_range(xmas_weakness_scanner(25, data), data)
}

fn xmas_weakness_scanner(preamble: usize, data: &Vec<usize>) -> usize {
    let mut i = 0usize;
    loop {
        // We really should length-check the vector...
        let numbers = &data[i..(i + preamble)];
        let current = &data[i + preamble];
        i += 1;
        let mut found = false;
        for x in numbers {
            for y in numbers {
                if x == y {
                    continue;
                }
                if x + y == *current {
                    found = true;
                }
            }
        }
        if !found {
            return *current;
        }
    }
}

fn xmas_weakness_range(target: usize, data: &Vec<usize>) -> usize {
    data.iter()
        .enumerate()
        .filter_map(|(i1, &x)| {
            let res = data
                .iter()
                .enumerate()
                .skip(i1 + 1)
                .scan(x, |state, (i2, y)| {
                    *state += y;
                    if *state > target {
                        return None;
                    }
                    return Some((*state, &data[i1..i2 + 1]));
                })
                .last();
            match res {
                Some((sum, res_vec)) => {
                    if sum == target {
                        return Some(res_vec.iter().min().unwrap() + res_vec.iter().max().unwrap());
                    }
                    None
                }
                None => None,
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    pub fn test_day9_part1() {
        assert_eq!(
            xmas_weakness_scanner(5, &input_generator(SAMPLE_INPUT)),
            127
        );
    }

    #[test]
    pub fn test_day9_part2() {
        assert_eq!(xmas_weakness_range(127, &input_generator(SAMPLE_INPUT)), 62);
    }
}
