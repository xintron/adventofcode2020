use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{char, digit1, space0, space1},
    combinator::{all_consuming, map_parser, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

type BagColor = String;
type BagCount = u8;

type Bag = (BagColor, BagCount);

type BagMap = HashMap<BagColor, HashMap<BagColor, BagCount>>;

pub fn take_bag_color(s: &str) -> IResult<&str, &str> {
    take_until(" bag")(s)
}

pub fn take_bags(s: &str) -> IResult<&str, HashMap<BagColor, BagCount>> {
    let (s, bags) = separated_list1(
        alt((char(','), char('.'))),
        map_parser(is_not(",."), take_bag),
    )(s)?;
    Ok((s, HashMap::from_iter(bags)))
}

pub fn take_bag(s: &str) -> IResult<&str, Bag> {
    let (s, _) = space0(s)?;
    let (s, num_bags) = map_parser(is_not(" "), digit1)(s)?;
    let (s, _) = space1(s)?;
    let (s, bag_color) = take_bag_color(s)?;
    let (s, _) = all_consuming(tuple((space1, tag("bag"), opt(char('s')))))(s)?;
    Ok((s, (bag_color.to_string(), u8::from_str(num_bags).unwrap())))
}

pub fn parse_line(i: &str) -> IResult<&str, (BagColor, HashMap<BagColor, BagCount>)> {
    let (i, bag_color) = take_bag_color(i)?;
    let (i, _) = tuple((
        space1,
        tag("bag"),
        opt(char('s')),
        space1,
        tag("contain"),
        space1,
    ))(i)?;
    let (i, bags) = opt(take_bags)(i)?;
    match bags {
        Some(b) => Ok((i, (bag_color.to_string(), b))),
        None => Ok((i, (bag_color.to_string(), HashMap::default()))),
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> BagMap {
    input
        .lines()
        .map(|s| parse_line(s).unwrap().1)
        .collect::<BagMap>()
}

#[aoc(day7, part1)]
pub fn part1(bags: &BagMap) -> u32 {
    let mut count = 0;
    for k in bags.keys() {
        let mut found = false;
        contains_bag(&mut found, &"shiny gold".to_string(), &k, &bags);
        if found {
            count += 1;
        }
    }
    count
}

pub fn contains_bag(state: &mut bool, search: &BagColor, current: &BagColor, bags: &BagMap) {
    if *state {
        return;
    }
    if bags.get(current).unwrap().contains_key(search) {
        return *state = true;
    }
    for (bag, _) in bags.get(current).unwrap().iter() {
        contains_bag(state, search, bag, bags);
    }
}

#[aoc(day7, part2)]
pub fn part2(bags: &BagMap) -> u32 {
    bag_sum(&"shiny gold".to_string(), bags)
}

pub fn bag_sum(current: &BagColor, bags: &BagMap) -> u32 {
    let bag = bags.get(current).unwrap();
    let mut sum = 0u32;
    for (k, v) in bag.iter() {
        sum += (*v as u32) + (*v as u32) * bag_sum(k, bags);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        assert_eq!(
            take_bag_color(input.lines().nth(0).unwrap()).unwrap().1,
            "light red"
        );
        assert_eq!(
            take_bag("1 bright white bag"),
            Ok(("", ("bright white".to_string(), 1)))
        );
        assert_eq!(
            take_bags("3 bright white bags, 4 muted yellow bags."),
            Ok((
                ".",
                HashMap::from_iter(vec![
                    ("bright white".to_string(), 3),
                    ("muted yellow".to_string(), 4)
                ])
            ))
        );

        // Full line
        assert_eq!(
            parse_line(input.lines().nth(0).unwrap()),
            Ok((
                ".",
                (
                    "light red".to_string(),
                    HashMap::from_iter(vec![
                        ("bright white".to_string(), 1),
                        ("muted yellow".to_string(), 2)
                    ])
                )
            ))
        );

        assert_eq!(
            parse_line(input.lines().nth(7).unwrap()),
            Ok((
                "no other bags.",
                ("faded blue".to_string(), HashMap::default())
            ))
        );

        assert_eq!(part1(&input_generator(input)), 4);
    }

    #[test]
    pub fn nested_counting_part2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part2(&input_generator(input)), 126);

        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part2(&input_generator(input)), 32);
    }
}
