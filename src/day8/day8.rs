#![allow(unused)]

use std::collections::{HashMap, HashSet};
use crate::utils::{permutate, CharMap};
use crate::utils::multimap::MultiMap;

pub fn part_1() {
    let data = include_str!("day8.txt");
    let r = part_1_inner(data);
    println!("{}", r);
}

fn part_1_inner(data: &str) -> i32 {
    let map = CharMap::from_str(data);
    let freqs = build_antennas(&map);
    let mut antinodes = HashSet::new();

    for (freq, antennas) in freqs.get_values() {
        if antennas.len() <= 1 {
            continue;
        }

        for ((x1, y1), (x2, y2)) in permutate(antennas) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let (p1x, p1y) = (x2 + dx, y2 + dy);
            let (p2x, p2y) = (x1 - dx, y1 - dy);
            if map.is_inside(p1x, p1y) {
                antinodes.insert((p1x, p1y));
            }
            if map.is_inside(p2x, p2y) {
                antinodes.insert((p2x, p2y));
            }

        }
    }

    antinodes.len() as i32
}

fn build_antennas(map: &CharMap) -> MultiMap<char, (i32, i32)>{
    let mut dict = MultiMap::new();
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get_char(x, y) {
                '.' => {},
                '\0' => panic!("invalid map value!"),
                c => dict.insert(c, (x, y))
            }
        }
    }

    dict
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 14);
    }

}

