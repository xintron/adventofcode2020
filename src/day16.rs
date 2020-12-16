// (start, end), Rule-name
// start and end are inclusive
type RuleRange = ((usize, usize), String);
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
        for range in ranges {
            rules.push(((range[0], range[1]), rule_name.to_owned()));
        }
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
                .any(|((start, end), _)| n >= start && n <= end)
            {
                return Some(n);
            }
            None
        })
        .sum::<usize>()
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
                ((1, 3), "class".to_owned()),
                ((5, 7), "class".to_owned()),
                ((6, 11), "row".to_owned()),
                ((33, 44), "row".to_owned()),
                ((13, 40), "seat".to_owned()),
                ((45, 50), "seat".to_owned()),
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
}
