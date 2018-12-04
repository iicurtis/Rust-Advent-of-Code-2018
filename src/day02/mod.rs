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
use std::str;

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

#[aoc(day2, part1, burntsushi)]
// https://github.com/BurntSushi/advent-of-code/blob/master/2018/aoc02/src/main.rs
fn part1_burnt(input: &str) -> u32 {
    let mut frequencies = [0u8; 256];
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        for f in frequencies.iter_mut() {
            *f = 0;
        }
        for b in line.as_bytes().iter().map(|&b| b as usize) {
            frequencies[b] = frequencies[b].saturating_add(1);
        }
        if frequencies.iter().any(|&f| f == 2) {
            twos += 1;
        }
        if frequencies.iter().any(|&f| f == 3) {
            threes += 1;
        }
    }
    return twos * threes;
}

#[aoc(day2, part2, simd_discord)]
pub fn part2_simd<'a>(input: &'a str) -> String {
    use packed_simd::u8x32;
    let lines = input.lines();

    #[repr(align(32))]
    #[derive(Copy, Clone)]
    struct Line([u8; 32]);

    let mut storage = [u8x32::splat(0); 250];
    let mut buf = Line([0; 32]);
    for (storage, line) in storage.iter_mut().zip(lines) {
        let line = line.trim_end();
        buf.0[..line.len()].copy_from_slice(line.as_bytes());
        *storage = u8x32::from_slice_aligned(&buf.0);
    }

    for (i, &a) in storage.iter().enumerate() {
        for &b in &storage[i + 1..] {
            if a.eq(b)
                .select(u8x32::splat(1), u8x32::splat(0))
                .wrapping_sum()
                == 31
            {
                let mut buf = String::with_capacity(25);
                let a: [u8; 32] = a.into();
                let b: [u8; 32] = b.into();
                for (&a, &b) in a.iter().zip(&b) {
                    if a == b {
                        buf.push(a as char);
                    }
                }
                return buf;
            }
        }
    }
    return String::from("fail");
}

#[aoc(day2, part2)]
pub fn part2<'a>(input: &'a str) -> String {
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
}
