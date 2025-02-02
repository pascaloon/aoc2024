#![allow(unused)]

use std::str::Split;

pub fn part_1() {
    let data = include_str!("day7.txt");
    let r = part_1_inner(data);
    println!("{}", r);
}

fn part_1_inner(data: &str) -> u64 {
    let mut sum = 0;
    for line in data.lines() {
        let mut s = line.split(": ");
        let target : u64 = s.next().unwrap().parse().unwrap();
        let right = s.next().unwrap();
        if is_possible(target, 0, right.split(" ")) {
            sum += target;
        }
    }

    sum
}

fn is_possible(target: u64, acc: u64,  mut elems: Split<&str>) -> bool {
    match elems.next() {
        None => acc == target,
        Some(next_value) => {
            let next: u64 = next_value.parse().unwrap();
            let new_split = elems.clone();
            is_possible(target, acc + next, elems)
            || is_possible(target, acc * next, new_split)
        }
    }
}

pub fn part_2() {
    let data = include_str!("day7.txt");
    let r = part_2_inner(data);
    println!("{}", r);
}

fn part_2_inner(data: &str) -> u64 {
    let mut sum = 0;
    for line in data.lines() {
        let mut s = line.split(": ");
        let target : u64 = s.next().unwrap().parse().unwrap();
        let right = s.next().unwrap();
        if is_possible2(target, 0, right.split(" ")) {
            sum += target;
        }
    }

    sum
}

fn is_possible2(target: u64, acc: u64,  mut elems: Split<&str>) -> bool {
    match elems.next() {
        None => acc == target,
        Some(next_value) => {
            let next: u64 = next_value.parse().unwrap();
            let new_split = elems.clone();
            let new_split2 = elems.clone();
            is_possible2(target, acc + next, elems)
            || is_possible2(target, acc * next, new_split)
            || is_possible2(target, concat(acc, next), new_split2)
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let mut a = a.to_string();
    a.push_str(&b.to_string());
    a.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 3749);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_inner(CONTENT), 11387);
    }

}

