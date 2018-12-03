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

#[aoc(day2, part1, birkinfeld)]
// https://github.com/birkenfeld/advent18/blob/master/src/bin/day02.rs
fn part1_birk(input: &str) -> u32 {
    let ids = input.lines().collect::<Vec<_>>();
    // Using fold here lets us keep track of the doubles/triples state
    // in the iterator without mutable outer variables.
    let (doubles, triples) = ids.iter().fold((0, 0), |(dbls, tpls), id| {
        let mut freqs = HashMap::<_, u32>::default();
        // Determine frequency of every character in the ID using a hashmap.
        id.chars().for_each(|c| *freqs.entry(c).or_default() += 1);
        // If we find any of the needed frequency, casting the bool to u32
        // gives "+ 0" or "+ 1".
        (
            dbls + freqs.values().any(|&n| n == 2) as u32,
            tpls + freqs.values().any(|&n| n == 3) as u32,
        )
    });
    return doubles * triples;
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

#[aoc(day2, part2, cryzefast)]
// https://github.com/zesterer/advent-of-code-2018/blob/master/examples/puzzle-2-2.rs
fn exec_fast(l: &[u8]) -> String {
    // I challenge you, whoever the hell you are, to write an implementation faster than this!

    // Uh huh.

    // Didn't think so.
    let lines = l
        .chunks(27)
        .map(|l| str::from_utf8(l).ok().unwrap())
        .collect::<Vec<_>>();

    use packed_simd::u8x32;

    let mut rail = [unsafe { std::mem::uninitialized::<u8x32>() }; 250];
    for (i, c) in l.chunks(27).enumerate() {
        let mut ptr =
            unsafe { std::slice::from_raw_parts_mut(&mut rail[i] as *mut _ as *mut u8, 32) };
        ptr[0..26].copy_from_slice(&c[0..26]);
        ptr[26..32].copy_from_slice(&[0, 0, 0, 0, 0, 0])
    }

    for i in 0..250 {
        for j in i + 1..250 {
            if (rail[i] - rail[j]).min(u8x32::splat(1)).wrapping_sum() == 1 {
                return lines[i].to_string();
            }
        }
    }

    return lines[0].to_string();
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
    fn part1_birk_example() {
        assert_eq!(part1_birk(INPUT_PART1), 12);
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
