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
use std::collections::hash_map::Entry;
use std::collections::HashMap;
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

    println!("[Day 02][Part 1] ANS is: {}", checksum.to_string());
    Ok(())
}

pub fn part2<'a>(input: &'a str) -> Result<()> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for index in 0..lines.first().unwrap().len() - 1 {
        for line in &lines {
            let line = (&line[..index], &line[index + 1..]);
            match map.entry(line) {
                Entry::Vacant(vacant) => {
                    vacant.insert(());
                }
                _ => writeln!(
                    io::stdout(),
                    "[Day 02][Part 2] ANS is: {}{}",
                    line.0,
                    line.1
                )?,
            };
        }
        map.clear();
    }

    Ok(())
}
