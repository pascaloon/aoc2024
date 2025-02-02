use std::collections::HashSet;
use crate::utils::{add_tuple, CharMap};

pub fn part_1() {
    let data = include_str!("day6.txt");
    let r = part_1_inner(data);
    println!("{}", r);
}

fn part_1_inner(data: &str) -> i32 {
    let map = CharMap::from_str(data);
    let mut player = find_player(&map).expect("No player in map!!!");

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(player.pos);

    loop {
        let (x, y) = add_tuple(player.pos, player.dir);
        let next = map.get_char(x, y);
        if next == '\0' {
            break;
        }
        if next == '#' {
            player.dir = rotate_right(player.dir).expect("Invalid Direction!");
            continue;
        }
        player.pos = (x, y);
        set.insert(player.pos);
    }

    set.len() as i32
}

fn find_player(map: &CharMap) -> Option<Player> {
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get_char(x, y) {
                '^' => return Some(Player {pos: (x, y), dir: ( 0, -1)}),
                '>' => return Some(Player {pos: (x, y), dir: ( 1,  0)}),
                'v' => return Some(Player {pos: (x, y), dir: ( 0,  1)}),
                '<' => return Some(Player {pos: (x, y), dir: (-1,  0)}),
                _ => {}
            };
        }
    }
    None
}

fn rotate_right(dir: (i32, i32)) -> Option<(i32, i32)> {
    match dir {
        ( 1,  0) => Some(( 0,  1)),
        ( 0,  1) => Some((-1,  0)),
        (-1,  0) => Some(( 0, -1)),
        ( 0, -1) => Some(( 1,  0)),
        _ => None
    }
}

struct Player {
    pub pos: (i32, i32),
    pub dir: (i32, i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    static CONTENT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner(CONTENT), 41);
    }

}

