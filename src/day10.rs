use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut d: Vec<usize> = input.lines().map(|d| d.parse().unwrap()).collect();
    d.push(0);
    d.sort_unstable();
    d.push(d.last().unwrap() + 3);
    d
}

#[aoc(day10, part1, scan)]
pub fn part1_scan(data: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::default();
    data.iter()
        .scan(0usize, |state, x| {
            let v = map.entry(x - *state).or_insert(0);
            *v += 1;
            *state = *x;
            Some(*state)
        })
        .last();
    map.get(&1).zip(map.get(&3)).map(|(x, y)| x * y).unwrap()
}

#[aoc(day10, part1, window)]
pub fn part1_window(data: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::default();
    for row in data.windows(2) {
        map.entry(row[1] - row[0])
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    map.get(&1).zip(map.get(&3)).map(|(x, y)| x * y).unwrap()
}

#[aoc(day10, part1, counting)]
pub fn part1_counting(data: &Vec<usize>) -> usize {
    let (mut ones, mut threes) = (0, 0);
    for row in data.windows(2) {
        match row[1] - row[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

#[aoc(day10, part2)]
pub fn part2(data: &Vec<usize>) -> usize {
    let mut path_count = vec![(0usize, 0usize); data.len()];
    path_count[0].1 = 1;
    for (i, &jolt) in data.iter().enumerate() {
        for j in 1..=3 {
            let current_count = path_count[i].1;
            let num = data.get(i + j);
            match num {
                Some(&x) if x - jolt <= 3 => {
                    path_count.get_mut(i + j).map(|count| {
                        count.0 = x;
                        count.1 += current_count;
                    });
                    ()
                }
                _ => (),
            };
        }
    }
    path_count.last().unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "16
10
15
5
1
11
7
19
6
12
4";
    #[test]
    pub fn test_day10_part1() {
        let parsed = input_generator(SAMPLE_INPUT);
        assert_eq!(
            parsed[1], 1,
            "First parsed element `{}` did not match",
            parsed[0]
        );

        assert_eq!(
            part1_scan(&parsed),
            7 * 5,
            "Product of 1-jolt diff * 3-jolt diff did not match `scan`."
        );
        assert_eq!(
            part1_window(&parsed),
            7 * 5,
            "Product of 1-jolt diff * 3-jolt diff did not match `window`."
        );
        assert_eq!(
            part1_counting(&parsed),
            7 * 5,
            "Product of 1-jolt diff * 3-jolt diff did not match `counting`."
        );
    }

    #[test]
    pub fn test_day10_part2() {
        let parsed = input_generator(SAMPLE_INPUT);
        let sum = part2(&parsed);

        assert_eq!(sum, 8, "Small sample arrangements.");

        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part2(&input_generator(input)), 19208);
    }
}
