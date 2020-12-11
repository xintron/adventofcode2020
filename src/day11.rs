type Position = (usize, usize);
type Step = (isize, isize);

static NEIGHBORS: [Step; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone, Debug, PartialEq)]
pub struct SeatGrid {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<Seat>>,
}

impl SeatGrid {
    // Fetch a specific seat position
    fn get(&self, position: Position) -> Option<&Seat> {
        self.grid
            .get(position.0)
            .and_then(|row| row.get(position.1))
    }

    fn set(&mut self, (r, c): Position, seat: Seat) {
        self.grid[r][c] = seat;
    }

    // Count occupied seats
    fn count_occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|col| col.iter())
            .filter_map(|seat| match seat {
                Seat::Occupied => Some(seat),
                _ => None,
            })
            .count()
    }

    fn new(grid: &Vec<Vec<Seat>>) -> SeatGrid {
        SeatGrid {
            rows: grid.len(),
            columns: grid[0].len(),
            grid: grid.to_owned(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Default for Seat {
    fn default() -> Self {
        Seat::Floor
    }
}

impl std::str::FromStr for Seat {
    type Err = &'static str;

    fn from_str(seat_state: &str) -> Result<Self, Self::Err> {
        Ok(match seat_state {
            "L" => Self::Empty,
            "#" => Self::Occupied,
            "." | _ => Self::Floor,
        })
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> SeatGrid {
    let grid: Vec<Vec<Seat>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    SeatGrid::new(&grid)
}

#[aoc(day11, part1)]
pub fn part1(grid: &SeatGrid) -> usize {
    let final_seats = find_match(grid, true, 4);
    final_seats.count_occupied_seats()
}

#[aoc(day11, part2)]
pub fn part2(grid: &SeatGrid) -> usize {
    let final_seats = find_match(grid, false, 5);
    final_seats.count_occupied_seats()
}

fn find_match(grid: &SeatGrid, adjacent: bool, distance: usize) -> SeatGrid {
    let mut current = SeatGrid::new(&grid.grid);
    let mut next = current.to_owned();
    loop {
        for r in 0..current.rows {
            for c in 0..current.columns {
                let pos = (r, c);
                let seat = current.get(pos).unwrap();
                // Floor, no need to do any calculations
                if *seat == Seat::Floor {
                    continue;
                }
                let neighbors = occupied_neighbors(&current, (r, c), adjacent);

                let seat = match seat {
                    Seat::Empty if neighbors == 0 => Seat::Occupied,
                    Seat::Occupied if neighbors >= distance => Seat::Empty,
                    _ => *seat,
                };
                next.set(pos, seat);
            }
        }
        if next == current {
            break;
        }
        std::mem::swap(&mut current, &mut next);
    }
    current
}

/// Find neighbors.
/// For `adjacent: true` only go one step in any direction to the closest
/// neighbor.
/// For `adjacent: false` traverse until the first Seat (non-Floor).
fn occupied_neighbors(grid: &SeatGrid, (r, c): Position, adjacent: bool) -> usize {
    NEIGHBORS
        .iter()
        .filter_map(|step| find_seat(grid, adjacent, (r as usize, c as usize), *step))
        .filter(|seat| **seat == Seat::Occupied)
        .count()
}

fn within_range(grid: &SeatGrid, (r, c): (isize, isize)) -> bool {
    r >= 0 && c >= 0 && r as usize <= grid.rows && c as usize <= grid.columns
}

fn find_seat(grid: &SeatGrid, adjacent: bool, (r, c): Position, (dr, dc): Step) -> Option<&Seat> {
    let (mut r, mut c) = (r as isize, c as isize);
    loop {
        r += dr;
        c += dc;

        if !within_range(grid, (r, c)) {
            break;
        }

        match grid.get((r as usize, c as usize)) {
            // We don't care about the floors
            Some(Seat::Floor) => (),
            Some(s) => return Some(&s),
            // We've reached the
            None => break,
        }

        if adjacent {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    pub fn test_day11_input_generator() {
        let seats = input_generator(SAMPLE_INPUT);
        assert_eq!(seats.get((0, 0)), Some(&Seat::Empty));
        assert_eq!(seats.get((2, 1)), Some(&Seat::Floor));
        /*
        let (row_max, column_max) = (seats.grid.len(), seats.grid[0].len());
        assert_eq!(seats.get((row_max, 1)), None);
        assert_eq!(seats.get((0, column_max)), None);
        assert_eq!(seats.get((row_max, column_max)), None);
        */
    }

    #[test]
    pub fn test_day11_part1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT)), 37);
    }

    #[test]
    pub fn test_day11_part2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT)), 26);
    }
}
