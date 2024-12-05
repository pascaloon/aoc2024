use std::str::FromStr;

pub fn part_1() {
    let content = include_str!("day2.txt");
    println!("{}", part_1_inner(content));
}

fn part_1_inner(data: &str) -> i32 {
    let mut sum = 0;

    for line in data.lines() {
        let v: Vec<i32> = line
            .split(' ')
            .map(|c| i32::from_str(c).unwrap())
            .collect();

        if validate(&v) {
            sum += 1;
        }
    }

    sum
}

pub fn part_2() {
    let content = include_str!("day2.txt");
    println!("{}", part_2_inner(content));
}

fn part_2_inner(data: &str) -> i32 {
    let mut sum = 0;

    for line in data.lines() {
        let v: Vec<i32> = line
            .split(' ')
            .map(|c| i32::from_str(c).unwrap())
            .collect();

        if validate(&v) {
            sum += 1;
        }
        else {
            for idx in 0..v.len() {
                let mut v = v.clone();
                v.remove(idx);
                if validate(&v) {
                    sum += 1;
                    break;
                }
            }
        }

    }

    sum
}


fn validate(input: &Vec<i32>) -> bool {
    // 7 6 4 2 1
    // 7 6 4 2
    // 6 4 2 1
    let zip: Vec<(i32, i32)> =
        input[..input.len()].iter().copied()
            .zip(input[1..].iter().copied())
            .collect();
    // for (a, b) in &zip {
    //     print!("({}, {}), ", a, b)
    // }

    // println!();

    let all_incr = zip.iter().all(|(a, b)| a < b);
    let all_decr = zip.iter().all(|(a, b)| a > b);

    let all_rate = zip.iter().all(|(a, b)| {
        let d = (b - a).abs();
        d >= 1 && d <= 3
    });

    (all_incr || all_decr) && all_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_inner(CONTENT), 4);
    }
}

