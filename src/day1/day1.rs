#![allow(unused)]

use std::str::FromStr;

pub fn part_1() -> () {
    let content = include_str!("day1.txt");
    println!("{}", part_1_inner(content));
}

fn part_1_inner(data: &str) -> i32 {
    let (mut list_a, mut list_b) = get_lists(data);
    list_a.sort();
    list_b.sort();

    let mut sum = 0;
    for i in 0..list_a.len() {
        sum += (list_b[i] - list_a[i]).abs();
    }

    sum
}

pub fn part_2() -> () {
    let content = include_str!("day1.txt");
    println!("{}", part_2_inner(content));
}

fn part_2_inner(data: &str) -> i32 {
    let (list_a, list_b) = get_lists(data);

    let mut sum = 0;
    for &x in &list_a {
        let mut count = 0;
        for &y in &list_b {
            if x == y {
                count += 1;
            }
        }

        sum += x * count;
    }

    sum
}

fn get_lists(data: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    for line in data.lines() {
        let parts: Vec<&str> = line.split(' ').filter(|e| !e.is_empty()).collect();
        // println!("{}, {}", parts[0], parts[1]);
        list_a.push(i32::from_str(parts[0]).unwrap());
        list_b.push(i32::from_str(parts[1]).unwrap());
    }

    (list_a, list_b)
}


#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_inner(CONTENT), 31);
    }
}

