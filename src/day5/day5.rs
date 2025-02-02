#![allow(unused)]

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day5/day5_grammar.pest"]
pub struct Input;

#[derive(Debug)]
struct InputData {
    pub rules: Vec<(i32, i32)>,
    pub orders: Vec<Vec<i32>>
}

fn parse_input(content: &str) -> InputData {
    let input = Input::parse(Rule::input, &content)
        .expect("couldn't parse content!")
        .next().unwrap();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut orders: Vec<Vec<i32>> = Vec::new();
    for r in input.into_inner() {
        match r.as_rule() {
            Rule::rules => { rules = parse_rules(r) }
            Rule::orders => { orders = parse_orders(r); }
            _ => { }
        }
    }

    InputData {rules, orders}
}

fn parse_orders(rule: Pair<Rule>) -> Vec<Vec<i32>> {
    rule.into_inner()
        .filter(|r| r.as_rule() == Rule::order)
        .map(|r| parse_order(r))
        .collect()
}

fn parse_order(rule: Pair<Rule>) -> Vec<i32> {
    rule.into_inner()
        .filter(|r| r.as_rule() == Rule::num)
        .map(|r| r.as_str().parse().unwrap())
        .collect()
}

fn parse_rules(rule: Pair<Rule>) -> Vec<(i32, i32)> {
    rule.into_inner()
        .filter(|r| r.as_rule() == Rule::rule)
        .map(|r| parse_rule(r))
        .collect()
}

fn parse_rule(rule: Pair<Rule>) -> (i32, i32) {
    let nums: Vec<i32> = rule.into_inner()
        .filter(|r| r.as_rule() == Rule::num)
        .map(|r| r.as_str().parse().unwrap())
        .collect();

    assert_eq!(nums.len(), 2);
    (nums[0], nums[1])
}

fn part_1_inner(data: &str) -> i32 {
    let input = parse_input(data);
    // println!("{:?}", input);

    let mut total = 0;
    for seq in &input.orders {
        if is_valid_dumb(seq, &input) {
            total += get_middle_number(seq);
        }
    }
    total
}

pub fn part_1() {
    let content = include_str!("day5.txt");
    println!("{}", part_1_inner(content));
}


fn get_middle_number(seq: &Vec<i32>) -> i32 {
    *seq.get(seq.len() / 2usize).unwrap()
}

fn is_valid_dumb(seq: &Vec<i32>, input: &InputData) -> bool {
    // very simple and dumb impl -> NO TRANSITIVITY
    let allowed_exceptions = Vec::with_capacity(seq.len() - 1);
    for (i, v) in seq.iter().copied().enumerate() {
        for (l, r) in input.rules.iter().copied() {
            if r == v && !allowed_exceptions.contains(&l) {
                if seq[i+1 ..].contains(&l) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 143);
    }
}

