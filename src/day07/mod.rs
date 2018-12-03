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

use itertools::Itertools;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn solve() -> Result<()> {
    // Get line from standard input
    let stdin = io::stdin();
    // let lines = stdin.lock().lines();

    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut doubles = 0;
    let mut triples = 0;

    for line in input.lines() {
        let mut chars: Vec<char> = line.chars().collect();

        chars.sort_by(|a, b| b.cmp(a));
        // let repeated: String = chars.iter().collect();
        let counts = chars
            .into_iter()
            .map(|c| (c, 1))
            .coalesce(|(c, n), (d, m)| {
                if c == d {
                    Ok((c, n + m))
                } else {
                    Err(((c, n), (d, m)))
                }
            });

        let mut first_double = true;
        let mut first_triple = true;
        for (_, n) in counts {
            if n == 2 && first_double {
                doubles += 1;
                first_double = false;
            } else if n == 3 && first_triple {
                triples += 1;
                first_triple = false;
            }
        }
    }
    let checksum = doubles * triples;

    println!("[Day 07][Part 1] ANS is: {}", checksum.to_string());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let box_ids: Vec<&str> = input.lines().collect();
    for i in 0..box_ids.len() {
        for j in 1..box_ids.len() {
            if let Some(common) = common_letters(&box_ids[i], &box_ids[j]) {
                writeln!(io::stdout(), "[Day 07][Part 2] ANS is: {}", common)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn common_letters(box_id1: &str, box_id2: &str) -> Option<String> {
    if box_id1.len() != box_id2.len() {
        return None;
    }

    let mut found_one_wrong = false;
    for (c1, c2) in box_id1.chars().zip(box_id2.chars()) {
        if c1 != c2 {
            if found_one_wrong {
                return None;
            }
            found_one_wrong = true;
        }
    }
    Some(
        box_id1
            .chars()
            .zip(box_id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}
