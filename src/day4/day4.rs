use std::iter::Iterator;

#[warn(dead_code)]

static WORD: &[u8] = b"XMAS";

pub fn part_1() {
    let content = include_str!("day4.txt");
    println!("{}", part_1_inner(content));
}

fn part_1_inner(data: &str) -> i32 {
    let map = CharMap::fom_str(data);

    let first_letter = *WORD.iter().next().unwrap() as char;

    let mut total_count = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let c = map.get_char(x, y);
            if c != first_letter {
                continue
            }

            // X found, probe for word
            let pos = (x, y);
            if test_line(&map,pos, (1, 0)) {
                println!("(1, 0)");
                total_count += 1;
            }
            if test_line(&map,pos, (1, 1)) {
                println!("(1, 1)");
                total_count += 1;
            }
            if test_line(&map,pos, (0, 1)) {
                println!("(0, 1)");
                total_count += 1;
            }
            if test_line(&map,pos, (-1, 1)) {
                println!("(-1, 1)");
                total_count += 1;
            }
            if test_line(&map,pos, (-1, 0)) {
                println!("(-1, 0)");
                total_count += 1;
            }
            if test_line(&map,pos, (-1, -1)) {
                println!("(-1, -1)");
                total_count += 1;
            }
            if test_line(&map,pos, (0, -1)) {
                println!("(0, -1)");
                total_count += 1;
            }
            if test_line(&map,pos, (1, -1)) {
                println!("(1, -1)");
                total_count += 1;
            }
        }
    }
    total_count
}

fn test_line(map: &CharMap, pos:(i32, i32), delta: (i32, i32)) -> bool {
    let (px, py) = pos;
    let (delta_x, delta_y) = delta;
    for ci in 0..WORD.len() {
        let i = ci as i32;
        let nx = px + (delta_x * i);
        let ny = py + (delta_y * i);

        if map.get_char(nx, ny) != WORD[ci] as char {
            return false;
        }
    }

    true
}

struct CharMap<'a> {
    data: &'a[u8],
    pub width: i32,
    pub height: i32
}

impl CharMap<'_> {
    pub fn fom_str(data: &str) -> CharMap {
        let width = data.lines().next().unwrap().len() as i32;
        let height = data.lines().count() as i32;
        let data = data.as_bytes();
        CharMap { data, width, height }
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        match (x, y) {
            (_, _) if x < 0 => '\0',
            (_, _) if x >= self.width => '\0',
            (_, _) if y < 0 => '\0',
            (_, _) if y >= self.height => '\0',
            (_, _) => {
                let i = (y * (self.width + 1)) + x; // +1 for the \n chars
                *self.data.get(i as usize).unwrap() as char
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 18);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(crate::day3::day3::part_2_inner(CONTENT2), 48);
    // }
}

