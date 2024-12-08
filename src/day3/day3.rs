#![allow(unused)]
use regex::Regex;

pub fn part_1() {
    let content = include_str!("day3.txt");
    println!("{}", part_1_inner(content));
}

fn part_1_inner(data: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"mul\((?<d1>\d+),(?<d2>\d+)\)").unwrap(); // A regex for YYYY-MM-DD date format
    for capture in re.captures_iter(data) {
        let d1: i32 = capture.name("d1").unwrap().as_str().parse().unwrap();
        let d2: i32 = capture.name("d2").unwrap().as_str().parse().unwrap();

        sum += d1 * d2;
    }

    sum
}

fn find_match_positions(pattern: &str, content: &str) -> Vec<usize> {
    let re = Regex::new(pattern).unwrap();
    re.find_iter(content).map(|m| m.start()).collect()
}

pub fn part_2() {
    let content = include_str!("day3.txt");
    println!("{}", part_2_inner(content));
}


fn part_2_inner(data: &str) -> i32 {
    let mut sum = 0;

    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<d1>\d+),(?<d2>\d+)\)").unwrap(); // A regex for YYYY-MM-DD date format
    let muls: Vec<usize> = re.captures(data).unwrap().iter().filter_map(|c| c).map(|c|c.start()).collect();
    

    let mut d1: i32 = 0;
    let mut do_it = true;
    for capture in re.captures_iter(data) {
        // println!("capture");
        if let Some(doo) = capture.name("do") {
            // println!("do");
            do_it = true;
        }

        if let Some(dont) = capture.name("dont") {
            // println!("dont");
            do_it = false;
        }

        if let Some(mul) = capture.name("d1") {
            d1 = mul.as_str().parse().unwrap();
            // println!("d1: {}", d1);
        }

        if let Some(mul) = capture.name("d2") {
            let d2: i32 = mul.as_str().parse().unwrap();
            // println!("d2: {}", d2);
            if do_it {
                sum += d1 * d2;
            }
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    static CONTENT2: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_inner(CONTENT2), 48);
    }
}

