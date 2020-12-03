#[derive(Debug)]
pub struct Password {
    min: usize,
    max: usize,
    policy: char,
    password: String,
}

impl From<&str> for Password {
    fn from(data: &str) -> Self {
        let parts = data.trim().split(' ').collect::<Vec<&str>>();
        let mut range = parts[0].split('-').map(|x| x.parse().unwrap());
        Password {
            min: range.next().unwrap(),
            max: range.next().unwrap(),
            policy: parts[1].chars().next().unwrap(),
            password: parts[2].to_string(),
        }
    }
}

impl Password {
    fn is_within_range(&self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.policy).count();
        count >= self.min && count <= self.max
    }

    fn only_one(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let first = chars[self.min - 1] == self.policy;
        let second = chars[self.max - 1] == self.policy;
        (first && !second) || (!first && second)
    }

    fn only_one_2(&self) -> bool {
        let first = self.password.chars().nth(self.min - 1).unwrap() == self.policy;
        let second = self.password.chars().nth(self.max - 1).unwrap() == self.policy;
        (first && !second) || (!first && second)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input.lines().map(Password::from).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Password]) -> usize {
    input.iter().filter(|p| p.is_within_range()).count()
}

#[aoc(day2, part2, collected)]
pub fn part2(input: &[Password]) -> usize {
    input.iter().filter(|p| p.only_one()).count()
}

#[aoc(day2, part2, iter)]
pub fn part2_iter(input: &[Password]) -> usize {
    input.iter().filter(|p| p.only_one_2()).count()
}
