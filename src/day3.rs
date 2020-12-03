#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1, simple_loop)]
pub fn part1_simple_loop(input: &[String]) -> usize {
    let mut pos = 0;
    let mut counter = 0;

    // Skip first line as it's not relevant
    for line in input.iter().skip(1) {
        pos += 3;
        if line.chars().nth(pos % line.len()).unwrap() == '#' {
            counter += 1;
        }
    }
    counter
}

#[aoc(day3, part1, ride_fn)]
pub fn part1_ride_fn(input: &[String]) -> usize {
    ride(&(3, 1), input)
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> usize {
    // (right, down)
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|slope| ride(slope, input)).product()
}

fn ride(slope: &(usize, usize), data: &[String]) -> usize {
    // Starting position
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    loop {
        x += slope.0;
        y += slope.1;
        // We're at the bottom
        if y >= data.len() {
            break;
        }

        let row = &data[y];
        if row.chars().nth(x % row.len()).unwrap() == '#' {
            trees += 1;
        }
    }
    return trees;
}
