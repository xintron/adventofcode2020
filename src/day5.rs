use std::collections::BTreeMap;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|x| {
            let (first, last) = x.split_at(7);
            (first.to_string(), last.to_string())
        })
        .collect()
}

#[aoc_generator(day5, part1, bit_operation)]
#[aoc_generator(day5, part2, bit_operation)]
pub fn input_generator_lines(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

fn scanner(range: &mut (u32, u32), ch: char) -> Option<(u32, u32)> {
    *range = bsp(ch, *range);
    Some(*range)
}

fn generate_seats(input: &[(String, String)]) -> Vec<(u32, u32)> {
    input
        .iter()
        .map(|(row_input, col_input)| {
            (
                row_input.chars().scan((0, 127), scanner).last().unwrap().0,
                col_input.chars().scan((0, 7), scanner).last().unwrap().0,
            )
        })
        .collect()
}

fn calculate_seat_id(data: &str) -> u32 {
    let mut id = 0u32;
    for (i, v) in data.chars().enumerate() {
        if v == 'B' || v == 'R' {
            id |= 1 << (9 - i)
        }
    }

    id
}

#[aoc(day5, part1)]
pub fn part1(input: &[(String, String)]) -> u32 {
    generate_seats(input)
        .iter()
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap()
}

#[aoc(day5, part1, replace_bits)]
pub fn part1_replace_bits(input: &[(String, String)]) -> u32 {
    input
        .iter()
        .map(|(row, col)| {
            u32::from_str_radix(&row.replace("F", "0").replace("B", "1"), 2).unwrap() * 8
                + u32::from_str_radix(&col.replace("L", "0").replace("R", "1"), 2).unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day5, part1, bit_operation)]
pub fn part1_bit_operation(input: &[String]) -> u32 {
    input.iter().map(|x| calculate_seat_id(x)).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[(String, String)]) -> u32 {
    let mut seats: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    let data = generate_seats(input);
    for seat in data {
        if let Some(row) = seats.get_mut(&seat.0) {
            row.push(seat.1);
        } else {
            seats.insert(seat.0, vec![seat.1]);
        }
    }
    let max_row = seats
        .iter()
        .max_by(|(r1, _), (r2, _)| r1.cmp(r2))
        .unwrap()
        .0
        - 1;
    let my_seat = seats
        .iter()
        .filter(|(r, c)| **r > 0 && **r < max_row && c.len() < 8)
        .next()
        .unwrap();
    // Each row's sum equals 28. Since we filter on vector length above we can
    // be sure that the calculation ends up correct, even if we have the first
    // seat (0)
    my_seat.0 * 8 + (28 - my_seat.1.iter().sum::<u32>())
}

#[aoc(day5, part2, bit_operation)]
pub fn part2_bit_operation(input: &[String]) -> u32 {
    let mut seat_ids: Vec<u32> = input.iter().map(|x| calculate_seat_id(x)).collect();
    seat_ids.sort_unstable();
    let min = 0;
    let max = (seat_ids.last().unwrap() >> 3) - 1;

    seat_ids
        .iter()
        .filter(|id| (**id >> 3) > min && (**id >> 3) < max)
        .scan(0u32, |st, id| {
            if *st == 0 {
                *st = *id;
                return Some(id + 1);
            }
            // We've found the gap, end the scan
            if id - *st > 1 {
                return None;
            }
            *st = *id;
            Some(id + 1)
        })
        .last()
        .unwrap()
}

pub fn bsp(section: char, range: (u32, u32)) -> (u32, u32) {
    let delta = (range.1 + range.0) / 2;
    match section {
        'F' | 'L' => (range.0, delta),
        'B' | 'R' => (delta + 1, range.1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let input = "FBFBBFF";
        let mut r = input.chars().scan((0, 127), scanner);
        assert_eq!(r.next(), Some((0, 63)));
        assert_eq!(r.next(), Some((32, 63)));
        assert_eq!(r.next(), Some((32, 47)));
        assert_eq!(r.next(), Some((40, 47)));
        assert_eq!(r.next(), Some((44, 47)));
        assert_eq!(r.next(), Some((44, 45)));
        assert_eq!(r.next(), Some((44, 44)));

        let input = "BFFFBBF";
        let r = input.chars().scan((0, 127), scanner).last();
        assert_eq!(r, Some((70, 70)));

        let input = "FFFBBBF";
        let r = input.chars().scan((0, 127), scanner).last();
        assert_eq!(r, Some((14, 14)));

        let input = "BBFFBBF";
        let r = input.chars().scan((0, 127), scanner).last();
        assert_eq!(r, Some((102, 102)));
    }

    #[test]
    fn test_bits_part1() {
        let row = "FBFBBFF".to_string();
        let col = "RLR".to_string();
        let mut full = row.clone();
        full.push_str(&col);
        assert_eq!(part1_replace_bits(&vec![(row, col)]), 357);
        assert_eq!(part1_bit_operation(&vec![full]), 357);

        let row = "BFFFBBF".to_string();
        let col = "RRR".to_string();
        let mut full = row.clone();
        full.push_str(&col);
        assert_eq!(part1_replace_bits(&vec![(row, col)]), 567);
        assert_eq!(part1_bit_operation(&vec![full]), 567);

        let row = "FFFBBBF".to_string();
        let col = "RRR".to_string();
        let mut full = row.clone();
        full.push_str(&col);
        assert_eq!(part1_replace_bits(&vec![(row, col)]), 119);
        assert_eq!(part1_bit_operation(&vec![full]), 119);

        let row = "BBFFBBF".to_string();
        let col = "RLL".to_string();
        let mut full = row.clone();
        full.push_str(&col);
        assert_eq!(part1_replace_bits(&vec![(row, col)]), 820);
        assert_eq!(part1_bit_operation(&vec![full]), 820);
    }
}
