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

use hashbrown;
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
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

    return checksum;
}

#[aoc(day2, part2)]
pub fn part2<'a>(input: &'a str) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for index in 0..lines.first().unwrap().len() - 1 {
        for line in &lines {
            let line = (&line[..index], &line[index + 1..]);
            match map.entry(line) {
                Entry::Vacant(vacant) => {
                    vacant.insert(());
                }
                _ => {
                    let mut ans = line.0.to_string();
                    ans.push_str(line.1);
                    return ans;
                }
            };
        }
        map.clear();
    }
    return String::from("Fail");
}

#[aoc(day2, part2, hashbrown)]
pub fn part2_hb<'a>(input: &'a str) -> String {
    let lines = input.lines().collect::<Vec<_>>();

    let mut map = hashbrown::HashMap::new();

    for index in 0..lines.first().unwrap().len() - 1 {
        for line in &lines {
            let line = (&line[..index], &line[index + 1..]);
            match map.entry(line) {
                hashbrown::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(());
                }
                _ => {
                    let mut ans = line.0.to_string();
                    ans.push_str(line.1);
                    return ans;
                }
            };
        }
        map.clear();
    }
    return String::from("Fail");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const INPUT_PART2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_PART1), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_PART2), "fgij");
    }

    #[test]
    fn part2_hb_example() {
        assert_eq!(part2_hb(INPUT_PART2), "fgij");
    }
}
