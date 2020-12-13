pub struct Timetable {
    time: usize,
    busses: Vec<Option<usize>>,
}

impl std::str::FromStr for Timetable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        Ok(Timetable {
            time: lines.next().map(|x| x.parse().unwrap()).unwrap(),
            busses: lines
                .next()
                .map(|line| {
                    line.split(',')
                        .map(|x| x.parse().ok())
                        .collect::<Vec<Option<usize>>>()
                })
                .unwrap(),
        })
    }
}

impl Timetable {
    fn next_bus(&self) -> (usize, usize) {
        let busses = self.busses.iter();

        busses
            .scan((0, usize::MAX), |state, bus| {
                match bus {
                    Some(b) => {
                        let delta = b - (self.time % b);
                        if delta < state.1 {
                            *state = (*b, delta)
                        }
                    }
                    None => (),
                }
                Some(*state)
            })
            .last()
            .unwrap()
            .to_owned()
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Timetable {
    input.parse().unwrap()
}

#[aoc(day13, part1)]
pub fn part1(timetable: &Timetable) -> usize {
    let (bus, delta) = timetable.next_bus();
    bus * delta
}

#[aoc(day13, part2)]
pub fn part2(timetable: &Timetable) -> usize {
    use num::Integer;

    let b: Vec<(usize, usize)> = timetable
        .busses
        .clone()
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| (i, x.unwrap()))
        .collect();
    let mut increment = b[0].1;
    let mut time = 0usize;
    let mut index_found: usize = 0;
    loop {
        time += increment;
        let mut all = true;
        for (offset, value) in &b {
            let num = time + offset;
            if num % value == 0 {
                if *offset > index_found {
                    increment = increment.lcm(value);
                    index_found = *offset;
                }
                continue;
            }
            all &= false;
            break;
        }

        // All matches, we're done
        if all {
            break;
        }
    }
    time
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_day13_input_generator() {
        let timetable: Timetable = input_generator(SAMPLE_INPUT);
        assert_eq!(timetable.time, 939);
        assert_eq!(
            timetable.busses,
            vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ]
        );
    }

    #[test]
    fn test_day13_part1() {
        let timetable: Timetable = input_generator(SAMPLE_INPUT);
        assert_eq!(timetable.next_bus(), (59, 5));
        assert_eq!(part1(&timetable), 295);
    }

    #[test]
    fn test_day13_part2() {
        let timetable: Timetable = input_generator(SAMPLE_INPUT);
        assert_eq!(part2(&timetable), 1068781);

        let timetable: Timetable = input_generator("0\n17,x,13,19");
        assert_eq!(part2(&timetable), 3417);

        let timetable: Timetable = input_generator("0\n67,7,59,61");
        assert_eq!(part2(&timetable), 754018);

        let timetable: Timetable = input_generator("0\n67,x,7,59,61");
        assert_eq!(part2(&timetable), 779210);

        let timetable: Timetable = input_generator("0\n67,7,x,59,61");
        assert_eq!(part2(&timetable), 1261476);

        let timetable: Timetable = input_generator("0\n1789,37,47,1889");
        assert_eq!(part2(&timetable), 1202161486);
    }
}
