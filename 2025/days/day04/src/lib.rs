use core::num;
use std::ops::{Index, IndexMut};

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;


const DIRS: &[(i32, i32)] = &[(1,0), (0,-1), (-1,0), (0,1)];

struct Node {
    value: char,
    stage_for_removal: bool,
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            value,
            stage_for_removal: false,
        }
    }
}

struct Map<T>(Vec<Vec<T>>);

impl<T> Map<T> {
    fn new() -> Self {
        Self {
            0: Vec::new()
        }
    }

    fn add_row(self: &mut Self, row: Vec<T>) {
        self.0.push(row);
    }

    fn width(self: &Self) -> usize {
        if self.0.len() > 0 {
            self.0[0].len()
        } else {
            0
        }
    }

    fn height(self: &Self) -> usize {
        self.0.len()
    }
}


impl<T> Index<usize> for Map<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}



fn parse(input: &str) -> Map<Node> {
    let mut map = Map::new();

    for line in input.lines() {
        let map_row = line.chars().map(|c| Node::new(c)).collect();
        map.add_row(map_row);
    }

    map
}

pub fn part_1(input: &str) -> Result<usize> {
    let map = parse(input);
    let mut accesible_roll_cnt: usize = 0;


    for y in 0..map.height() {
        for x in 0..map.width() {

            if map[y][x].value == '@'
            && 4 > count_adjacent_rolls(&map, (x as i32, y as i32)) {
                // println!("Free at: {},{}", x, y);
                accesible_roll_cnt += 1;
            }
        }
    }

    Ok(accesible_roll_cnt)
}

pub fn part_2(input: &str) -> Result<usize> {
    let mut map = parse(input);
    let mut removed_cnt: usize = 0;

    /* Repeatedly identifies accessible rolls across the map and then removes
        them, repeating until no more rolls are accessible. */
    loop {
        if 0 == find_accessible_rolls(&mut map) {
            break;
        }

        removed_cnt += remove_accessible_rolls(&mut map);
    }

    Ok(removed_cnt)
}


fn remove_accessible_rolls(map: &mut Map<Node>) -> usize {
    let mut num_removed = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {

            if map[y][x].stage_for_removal == true {
                map[y][x].stage_for_removal = false;
                map[y][x].value = '.';
                num_removed += 1;
            }
        }
    }

    num_removed
}

fn find_accessible_rolls(map: &mut Map<Node>) -> usize {
    let mut accesible_roll_cnt: usize = 0;

    for y in 0..map.height() {
        for x in 0..map.width() {

            if map[y][x].value == '@'
            && 4 > count_adjacent_rolls(&map, (x as i32, y as i32)) {
                map[y][x].stage_for_removal = true;
                accesible_roll_cnt += 1;
            }
        }
    }

    accesible_roll_cnt
}

/// Returns the number of adjacent rolls to roll at the given coordinate.
fn count_adjacent_rolls(map: &Map<Node>, point: (i32, i32)) -> u32 {
    /*                            x,           y */
    let up_left    = (point.0 - 1, point.1 - 1);
    let up         = (point.0,     point.1 - 1);
    let up_right   = (point.0 + 1, point.1 - 1);
    let right      = (point.0 + 1, point.1    );
    let down_right = (point.0 + 1, point.1 + 1);
    let down       = (point.0,     point.1 + 1);
    let down_left  = (point.0 - 1, point.1 + 1);
    let left       = (point.0 - 1, point.1    );

    let mut adjacent_cnt: u32 = 0;

    /* up-left */
    if up_left.0 >= 0 && up_left.1 >= 0
    && map[up_left.1 as usize][up_left.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* up */
    if up.0 >= 0 && up.1 >= 0
    && map[up.1 as usize][up.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* up-right */
    if up_right.0 < map.width() as i32
    && up_right.1 >= 0
    && map[up_right.1 as usize][up_right.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* right */
    if right.0 < map.width() as i32
    && right.1 >= 0
    && map[right.1 as usize][right.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* down-right */
    if down_right.0 < map.width() as i32
    && down_right.1 < map.height() as i32
    && map[down_right.1 as usize][down_right.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* down */
    if down.0 >= 0
    && down.1 < map.height() as i32
    && map[down.1 as usize][down.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* down-left */
    if down_left.0 >= 0
    && down_left.1 < map.height() as i32
    && map[down_left.1 as usize][down_left.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    /* left */
    if left.0 >= 0
    && left.1 >= 0
    && map[left.1 as usize][left.0 as usize].value == '@' {
        adjacent_cnt += 1;
    }

    adjacent_cnt
}





#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT).unwrap(), 43);
    }
}
