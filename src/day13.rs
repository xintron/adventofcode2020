pub struct Timetable {
    time: usize,
    busses: Vec<usize>,
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
                        .filter_map(|x| x.parse().ok())
                        .collect::<Vec<usize>>()
                })
                .unwrap(),
        })
    }
}

impl Timetable {
    fn next_bus(&self) -> (usize, usize) {
        let mut delta: usize = usize::MAX;
        let mut next_bus: usize = self.busses[0];

        for bus in self.busses.to_owned() {
            let d = bus - (self.time % bus);
            if d < delta {
                delta = d;
                next_bus = bus;
            }
        }
        (next_bus, delta)
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

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_day13_input_generator() {
        let timetable: Timetable = input_generator(SAMPLE_INPUT);
        assert_eq!(timetable.time, 939);
        assert_eq!(timetable.busses, vec![7, 13, 59, 31, 19]);
    }

    #[test]
    fn test_day13_part1() {
        let timetable: Timetable = input_generator(SAMPLE_INPUT);
        assert_eq!(timetable.next_bus(), (59, 5));
        assert_eq!(part1(&timetable), 295);
    }
}
