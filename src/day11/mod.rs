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

use std::fmt::{self, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x: usize,
    y: usize,
    s: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} Y: {} size: {}", self.x, self.y, self.s)
    }
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = (rack_id * y + serial) * rack_id;
    power = power / 100 % 10 - 5;
    return power;
}

#[aoc(day11, part1)]
fn part1(input: &str) -> Point {
    let serial: i32 = input.trim().parse().unwrap();
    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;
    let mut rack: Vec<i32> = vec![0; (WIDTH + 1) * (HEIGHT + 1)];

    for x in 1..=WIDTH {
        for y in 1..=HEIGHT {
            rack[x + y * WIDTH] = power_level(x as i32, y as i32, serial);
        }
    }

    let mut max: i32 = 0;
    let mut best = Point { x: 0, y: 0, s: 3 };
    for x in 0..WIDTH - 2 {
        for y in 0..HEIGHT - 2 {
            let mut power: i32 = 0;
            for xi in x..x + 3 {
                for yi in y..y + 3 {
                    power += rack[xi + yi * WIDTH];
                }
            }

            if power > max {
                best.x = x;
                best.y = y;
                max = power;
            }
        }
    }

    return best;
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Point {
    let serial: i32 = input.trim().parse().unwrap();
    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;
    let mut rack: Vec<i32> = vec![0; (WIDTH + 1) * (HEIGHT + 1)];

    for x in 1..=WIDTH {
        for y in 1..=HEIGHT {
            rack[x + y * WIDTH] = power_level(x as i32, y as i32, serial);
        }
    }

    let mut max: i32 = 0;
    let mut best = Point { x: 0, y: 0, s: 1 };
    for s in 1..300 {
        for x in 0..WIDTH - s + 1 {
            for y in 0..HEIGHT - s + 1 {
                let mut sum: i32 = 0;
                for xi in x..x + s {
                    for yi in y..y + s {
                        sum += rack[xi + yi * WIDTH];
                    }
                }

                if sum > max {
                    best.x = x;
                    best.y = y;
                    best.s = s;
                    max = sum;
                }
            }
        }
    }

    return best;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = r#"18"#;
        assert_eq!(part1(&input), Point { x: 33, y: 45 });
    }
    #[test]
    fn test_part1_1() {
        let input = r#"42"#;
        assert_eq!(part1(&input), Point { x: 21, y: 61 });
    }

    #[test]
    fn test_power_0() {
        assert_eq!(power_level(122, 79, 57), -5);
    }
    #[test]
    fn test_power_1() {
        assert_eq!(power_level(217, 196, 39), 0);
    }
    #[test]
    fn test_power_2() {
        assert_eq!(power_level(101, 153, 71), 4);
    }
    #[test]
    fn test_power_3() {
        assert_eq!(power_level(3, 5, 8), 4);
    }
    #[test]
    fn test_power_4() {
        assert_eq!(power_level(23 as i32, 63 as i32, 42), 4);
    }

}
