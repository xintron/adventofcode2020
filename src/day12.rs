type Instructions = Vec<Instruction>;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    action: Action,
    value: usize,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            action: s
                .chars()
                .nth(0)
                .map(|c| c.to_string().parse().unwrap())
                .unwrap(),
            value: s[1..].parse().unwrap(),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl std::str::FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::North),
            "S" => Ok(Self::South),
            "E" => Ok(Self::East),
            "W" => Ok(Self::West),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "F" => Ok(Self::Forward),
            _ => unreachable!("Invalid action"),
        }
    }
}

static DIRECTION_ROTATION: [Action; 4] = [Action::East, Action::South, Action::West, Action::North];
struct Ship {
    direction: Action,
    position: (isize, isize),
    waypoint: (isize, isize),
    move_type: MoveType,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            direction: Action::East,
            position: (0, 0),
            waypoint: (10, -1),
            move_type: MoveType::Direct,
        }
    }
}

impl Ship {
    fn move_waypoint(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::East => self.waypoint.0 += instruction.value as isize,
            Action::West => self.waypoint.0 -= instruction.value as isize,
            Action::North => self.waypoint.1 -= instruction.value as isize,
            Action::South => self.waypoint.1 += instruction.value as isize,
            _ => (),
        }
    }

    fn move_ship(&mut self, direction: Action, steps: isize) {
        match self.move_type {
            MoveType::Direct => match direction {
                Action::East => self.position.0 += steps,
                Action::West => self.position.0 -= steps,
                Action::North => self.position.1 -= steps,
                Action::South => self.position.1 += steps,
                _ => (),
            },
            MoveType::Waypoint => {
                self.position.0 += steps * self.waypoint.0;
                self.position.1 += steps * self.waypoint.1;
            }
        }
    }

    fn rotate(&mut self, instruction: &Instruction) {
        match self.move_type {
            MoveType::Direct => {
                let index = DIRECTION_ROTATION
                    .iter()
                    .position(|x| *x == self.direction)
                    .unwrap();
                let steps = instruction.value as isize / 90;
                let new_index = match instruction.action {
                    Action::Right => index as isize + steps,
                    Action::Left => index as isize - steps,
                    _ => unreachable!("Only right|left actions possible!"),
                };
                let new_dir = DIRECTION_ROTATION[(((new_index % 4) + 4) % 4) as usize];
                self.direction = new_dir;
            }
            MoveType::Waypoint => {
                let steps = instruction.value as isize / 90;

                let rotations = match (instruction.action, steps) {
                    (Action::Right, 1) | (Action::Left, 3) => 1,
                    (_, 2) => 2,
                    (Action::Right, 3) | (Action::Left, 1) => 3,
                    _ => 0,
                };

                let mut waypoint = self.waypoint.clone();
                for _ in 0..rotations {
                    waypoint = (-waypoint.1, waypoint.0);
                }

                self.waypoint = waypoint;
            }
        }
    }
}

enum MoveType {
    Direct,
    Waypoint,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Instructions {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn part1(instructions: &Instructions) -> usize {
    let mut ship = Ship::default();
    for ins in instructions {
        match ins.action {
            Action::Forward => ship.move_ship(ship.direction, ins.value as isize),
            Action::East | Action::West | Action::North | Action::South => {
                ship.move_ship(ins.action, ins.value as isize)
            }
            Action::Left | Action::Right => ship.rotate(&ins),
        }
    }
    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

#[aoc(day12, part2)]
pub fn part2(instructions: &Instructions) -> usize {
    let mut ship = Ship {
        move_type: MoveType::Waypoint,
        ..Ship::default()
    };
    for ins in instructions {
        match ins.action {
            Action::Forward => ship.move_ship(ins.action, ins.value as isize),
            Action::East | Action::West | Action::North | Action::South => ship.move_waypoint(ins),
            Action::Left | Action::Right => ship.rotate(&ins),
        }
    }
    (ship.position.0.abs() + ship.position.1.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_day12_input_generator() {
        let ins = input_generator(SAMPLE_INPUT);
        assert_eq!(
            ins[0],
            Instruction {
                action: Action::Forward,
                value: 10
            }
        );
        assert_eq!(
            ins[1],
            Instruction {
                action: Action::North,
                value: 3
            }
        );
        assert_eq!(
            ins[2],
            Instruction {
                action: Action::Forward,
                value: 7
            }
        );
        assert_eq!(
            ins[3],
            Instruction {
                action: Action::Right,
                value: 90
            }
        );
        assert_eq!(
            ins[4],
            Instruction {
                action: Action::Forward,
                value: 11
            }
        );
    }

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT)), 17 + 8)
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT)), 214 + 72)
    }

    #[test]
    fn test_day12_ship_rotate() {
        // Rotate right (clockwise)
        let mut ship = Ship::default();
        ship.rotate(&Instruction {
            action: Action::Right,
            value: 90,
        });
        assert_eq!(ship.direction, Action::South);

        let mut ship = Ship::default();
        ship.rotate(&Instruction {
            action: Action::Right,
            value: 270,
        });
        assert_eq!(ship.direction, Action::North);

        // Rotate left (counter-clockwise)
        let mut ship = Ship::default();
        ship.rotate(&Instruction {
            action: Action::Left,
            value: 90,
        });
        assert_eq!(ship.direction, Action::North);

        let mut ship = Ship::default();
        ship.rotate(&Instruction {
            action: Action::Left,
            value: 270,
        });
        assert_eq!(ship.direction, Action::South);
    }

    #[test]
    fn test_day12_waypoint_rotate() {
        // Rotate right (clockwise)
        let mut ship = Ship {
            move_type: MoveType::Waypoint,
            ..Ship::default()
        };
        assert_eq!(ship.waypoint, (10, -1));

        ship.rotate(&Instruction {
            action: Action::Right,
            value: 90,
        });
        assert_eq!(ship.waypoint, (1, 10));

        let mut ship = Ship {
            move_type: MoveType::Waypoint,
            ..Ship::default()
        };
        ship.rotate(&Instruction {
            action: Action::Right,
            value: 270,
        });
        assert_eq!(ship.waypoint, (-1, -10));

        // Rotate left (counter-clockwise)
        let mut ship = Ship {
            move_type: MoveType::Waypoint,
            ..Ship::default()
        };
        ship.rotate(&Instruction {
            action: Action::Left,
            value: 90,
        });
        assert_eq!(ship.waypoint, (-1, -10));

        let mut ship = Ship {
            move_type: MoveType::Waypoint,
            ..Ship::default()
        };
        ship.rotate(&Instruction {
            action: Action::Left,
            value: 270,
        });
        assert_eq!(ship.waypoint, (1, 10));
    }
}
