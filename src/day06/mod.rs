// Advent of Code
// Copyright (C) 2018  Isaac Curtis

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use hashbrown::HashMap;
use std::fmt::{self, Display};
use std::i32;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Point> {
    let mut input_vec: Vec<Point> = Vec::new();
    for line in input.trim().lines() {
        let all_the_things = line
            .split(|c| c == ',' || c == ' ')
            .filter_map(|c| c.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let x = all_the_things[0];
        let y = all_the_things[1];
        input_vec.push(Point { x: x, y: y })
    }
    input_vec.sort_by_key(|k| k.x);
    input_vec
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:3},{:3}]", self.x, self.y)
    }
}

pub fn manhatten(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<Point>) -> usize {
    let max_x = input.iter().max_by_key(|k| k.x).unwrap().x;
    let max_y = input.iter().max_by_key(|k| k.y).unwrap().y;
    let min_x = input.iter().min_by_key(|k| k.x).unwrap().x;
    let min_y = input.iter().min_by_key(|k| k.y).unwrap().y;
    let mut count = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut mindist = 1 << 20;
            let mut idx = 0;
            let mut eq = false;
            for k in 0..input.len() {
                let dist = manhatten(Point { x, y }, input[k]);
                if dist < mindist {
                    mindist = dist;
                    idx = k;
                    eq = false;
                } else if dist == mindist {
                    eq = true;
                }
            }
            if eq == false {
                *count.entry(idx).or_insert(0) += 1;
            }
        }
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !(x == min_x || x == max_x || y == min_y || y == max_y) {
                continue;
            }
            let mut mindist = 1 << 20;
            let mut idx = 0;
            let mut eq = false;
            for k in 0..input.len() {
                let dist = manhatten(Point { x, y }, input[k]);
                if dist < mindist {
                    mindist = dist;
                    idx = k;
                    eq = false;
                } else if dist == mindist {
                    eq = true;
                }
            }
            if eq == false {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    *count.entry(idx).or_insert(0) = 0;
                }
            }
        }
    }

    return *count.values().max().unwrap();
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Point>) -> usize {
    const MAX_DIST: i32 = 10_000;
    let max_x = input.iter().max_by_key(|k| k.x).unwrap().x;
    let max_y = input.iter().max_by_key(|k| k.y).unwrap().y;
    let min_x = input.iter().min_by_key(|k| k.x).unwrap().x;
    let min_y = input.iter().min_by_key(|k| k.y).unwrap().y;
    let mut count = 0;
    for x in min_x..=max_x {
        'nextpoint: for y in min_y..=max_y {
            let mut dist_sum = 0;
            for k in 0..input.len() {
                dist_sum += manhatten(Point { x, y }, input[k]);
                if dist_sum >= MAX_DIST {
                    continue 'nextpoint;
                }
            }
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;
        assert_eq!(part1(&parse_input(input)), 17);
    }

    #[test]
    fn test_part2() {
        let input = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;
        assert_eq!(part2(&parse_input(input)), 16);
    }

}
