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

use hashbrown::{HashMap, HashSet};
use packed_simd::i32x16;
use rayon::prelude::*;
use std::error::Error;
use std::fmt::{self, Display};
use std::i32;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Point> {
    let mut input_vec = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Point>>();
    input_vec.sort_by_key(|k| k.x);
    input_vec
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn l1dist(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn neighbors(self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }
}

impl std::str::FromStr for Point {
    type Err = Box<Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let point = input
            .split(|c| c == ',' || c == ' ')
            .filter_map(|c| c.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Ok(Point {
            x: point[0],
            y: point[1],
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:3},{:3}]", self.x, self.y)
    }
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
            let mut mindist = 1 << 8;
            let mut idx = 0;
            let mut eq = false;
            for k in 0..input.len() {
                let dist = Point { x, y }.l1dist(input[k]);
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
                let dist = Point { x, y }.l1dist(input[k]);
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

fn simd_dist_l1(x1: i32x16, y1: i32x16, x2: i32x16, y2: i32x16) -> i32x16 {
    // (self.x - other.x).abs() + (self.y - other.y).abs()
    abs_s(x1 - x2) + abs_s(y1 - y2)
}

fn abs_s(vec: i32x16) -> i32x16 {
    let mask = vec >> (std::mem::size_of::<i32>() * 8 - 1) as u32;
    (vec + mask) ^ mask
}

#[aoc(day6, part2)]
fn part2(input: &Vec<Point>) -> usize {
    const MAX_DIST: i32 = 10_000;
    let mut open_set = Vec::new();
    // start with mean of points
    let center_x = input.iter().map(|v| v.x).sum::<i32>() / input.len() as i32;
    let center_y = input.iter().map(|v| v.y).sum::<i32>() / input.len() as i32;
    open_set.push(Point {
        x: center_x,
        y: center_y,
    });
    let mut visited = HashSet::new();
    let mut count = 0;

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        let mut too_far = false;
        let mut dist_sum = 0;
        'nextpoint: for k in input.iter() {
            dist_sum += current.l1dist(*k);
            if dist_sum >= MAX_DIST {
                too_far = true;
                break 'nextpoint;
            }
        }
        if !too_far {
            'children: for child in current.neighbors() {
                if visited.contains(&child) {
                    continue 'children;
                } else {
                    open_set.push(child);
                }
            }
            count += 1;
        }
        visited.insert(current);
    }

    return count;
}

#[aoc(day6, part2, orig)]
fn part2_orig(input: &Vec<Point>) -> usize {
    const MAX_DIST: i32x16 = i32x16::splat(10_000);
    const ONES: i32x16 = i32x16::splat(1);
    const ZEROS: i32x16 = i32x16::splat(0);
    let range_32: i32x16 = i32x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    let max_x = input.iter().max_by_key(|k| k.x).unwrap().x;
    let max_y = input.iter().max_by_key(|k| k.y).unwrap().y;
    let min_x = input.iter().min_by_key(|k| k.x).unwrap().x;
    let min_y = input.iter().min_by_key(|k| k.y).unwrap().y;
    (min_x..max_x + 1)
        .into_par_iter()
        .map(|x| {
            let mut count = 0;
            let x1_simd = i32x16::splat(x);
            for y in (min_y..=max_y).step_by(i32x16::lanes()) {
                let mut dist = i32x16::splat(0);
                let y1_simd = i32x16::splat(y) + range_32;
                for k in input {
                    let x2_simd = i32x16::splat(k.x);
                    let y2_simd = i32x16::splat(k.y);
                    dist += simd_dist_l1(x1_simd, y1_simd, x2_simd, y2_simd);
                }
                count += dist.lt(MAX_DIST).select(ONES, ZEROS).wrapping_sum() as u32;
            }
            count
        })
        .sum::<u32>() as usize
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
