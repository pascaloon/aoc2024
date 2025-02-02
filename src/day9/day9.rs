#![allow(unused)]

use std::str::FromStr;
use crate::utils::MyVecUtils;

pub fn part_1() {
    let data = include_str!("day9.txt");
    let r = part_1_inner(data);
    println!("{}", r);
}

fn part_1_inner(data: &str) -> u64 {
    let mut disk = parse_disk(data);

    for i in (0..disk.len()).rev() {
        let d = disk[i];
        if d == -1 {
            continue;
        }

        let p = disk.iter().copied()
            .position(|d| d == -1)
            .expect("very unlikely no space");
        if p > i {
            break;
        }

        disk[p] = d;
        disk[i] = -1;

    }

    let checksum = compute_checksum(&disk);
    checksum
}

fn compute_checksum(disk: &Vec<i32>) -> u64 {
    disk.iter().copied()
        .take_while(|d| *d != -1)
        .enumerate()
        .map(|(i, id)| i as u64 * id as u64)
        .sum()
}

fn parse_disk(data: &str) -> Vec<i32> {
    let mut disk: Vec<i32> = Vec::new();
    let mut is_space = false;
    let mut file_id = 0;
    for c in data.chars() {
        let q = c.to_digit(10).expect("digit expected in input string.");
        disk.reserve(q as usize);
        if is_space {
            disk.add_repeated(q, -1);
        }
        else {
            disk.add_repeated(q, file_id);
            file_id += 1;
        }
        is_space = !is_space;
    }

    disk
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 1928);
    }

}

