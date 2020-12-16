use itertools::Itertools;

// (start1, end1), (start2, end2), Rule-name
// start and end are inclusive
type RuleRange = ((usize, usize), (usize, usize), String);
#[derive(Clone)]
struct Instruction {
    rules: Vec<RuleRange>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Instruction {
    let parts: Vec<&str> = input.splitn(3, "\n\n").collect();

    let mut rules: Vec<RuleRange> = Vec::new();
    for line in parts[0].lines() {
        let mut data = line.split(':');
        let rule_name = data.next().unwrap();

        let ranges: Vec<Vec<usize>> = data
            .next()
            .unwrap()
            .split(" or ")
            .map(|r| r.trim().split('-').map(|x| x.parse().unwrap()).collect())
            .collect();
        rules.push((
            (ranges[0][0], ranges[0][1]),
            (ranges[1][0], ranges[1][1]),
            rule_name.to_owned(),
        ));
    }

    let ticket_parser = |line: &str| {
        line.split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };

    let my_ticket = parts[1].lines().skip(1).map(ticket_parser).next().unwrap();
    let nearby_tickets = parts[2].lines().skip(1).map(ticket_parser).collect();

    Instruction {
        rules: rules.to_owned(),
        my_ticket: my_ticket,
        nearby_tickets: nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part1(instruction: &Instruction) -> usize {
    instruction
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter_map(|n| {
            if !instruction
                .rules
                .iter()
                .any(|((start1, end1), (start2, end2), _)| {
                    within_range(n, start1, end1, start2, end2)
                })
            {
                return Some(n);
            }
            None
        })
        .sum::<usize>()
}

#[aoc(day16, part2)]
fn part2(instruction: &Instruction) -> usize {
    let filtered: Vec<Vec<usize>> = instruction
        .nearby_tickets
        .clone()
        .into_iter()
        .filter(|ticket| {
            ticket.iter().all(|n| {
                instruction
                    .rules
                    .iter()
                    .any(|rule| within_range(n, &rule.0 .0, &rule.0 .1, &rule.1 .0, &rule.1 .1))
            })
        })
        .collect();
    let ins = Instruction {
        nearby_tickets: filtered,
        ..(&instruction.clone()).to_owned()
    };
    let ordered = map_columns(&ins);

    let f: Vec<usize> = ordered
        .iter()
        .enumerate()
        .filter(|(_, v)| v.starts_with("departure"))
        .map(|(i, _)| ins.my_ticket[i])
        .collect();
    f.iter().product()
}

// Calculates and maps each column from nearby_tickets to a specific rule-name.
// The returned vector is ordered
fn map_columns(instruction: &Instruction) -> Vec<String> {
    let mut columns = Vec::new();

    for rule in instruction.rules.iter() {
        for j in 0..instruction.my_ticket.len() {
            let matches = instruction
                .nearby_tickets
                .iter()
                .all(|x| within_range(&x[j], &rule.0 .0, &rule.0 .1, &rule.1 .0, &rule.1 .1));
            if matches {
                columns.push((j, rule.2.to_owned()));
            }
        }
    }

    let mut order: Vec<String> = Vec::with_capacity(instruction.rules.len());
    order.resize(instruction.rules.len(), String::new());
    loop {
        if columns.len() == 0 {
            break;
        }
        let map = columns.clone().into_iter().into_group_map();

        for (k, v) in map {
            if v.len() == 1 {
                let name = v[0].clone();
                order[k] = v[0].to_owned();
                columns = columns
                    .clone()
                    .into_iter()
                    .filter(|(_, v)| *v != name)
                    .collect();
            }
        }
    }

    order
}

fn within_range(value: &usize, start1: &usize, end1: &usize, start2: &usize, end2: &usize) -> bool {
    (value >= start1 && value <= end1) || (value >= start2 && value <= end2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn input_parser() {
        let ins = input_generator(SAMPLE_INPUT);
        assert_eq!(
            ins.rules,
            vec![
                ((1, 3), (5, 7), "class".to_owned()),
                ((6, 11), (33, 44), "row".to_owned()),
                ((13, 40), (45, 50), "seat".to_owned()),
            ]
        );
        assert_eq!(ins.my_ticket, vec![7, 1, 14]);
        assert_eq!(
            ins.nearby_tickets,
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        );
    }

    #[test]
    fn test_part1() {
        let ins = input_generator(SAMPLE_INPUT);
        assert_eq!(part1(&ins), 71usize);
    }

    #[test]
    fn test_part2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let ins = input_generator(input);
        let ordered = map_columns(&ins);
        assert_eq!(
            &ordered,
            &vec!["row".to_owned(), "class".to_owned(), "seat".to_owned()]
        );
    }
}
