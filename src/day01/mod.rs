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
use std::io::{self, BufRead};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn solve() -> Result<()> {
    // Get line from standard input
    let stdin = io::stdin();
    let input: Vec<isize> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let sum: isize = input.iter().sum();

    println!("[Day 01][Part 1] ANS is: {}", sum.to_string());

    let mut seen = HashSet::new();
    let mut sum = 0;

    for freq in input.iter().cycle() {
        sum += freq;
        let repeated = seen.insert(sum);
        if !repeated {
            break;
        }
    }

    println!("[Day 01][Part 2] ANS is: {}", sum.to_string());
    Ok(())
}
