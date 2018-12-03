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

use std::collections::HashSet;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[isize]) -> isize {
    return input.iter().sum();
}

#[aoc(day1, part2)]
fn part2(input: &[isize]) -> isize {
    let mut seen = HashSet::new();
    let mut sum = 0;

    for freq in input.iter().cycle() {
        sum += freq;
        let repeated = seen.insert(sum);
        if !repeated {
            break;
        }
    }

    return sum;
}
