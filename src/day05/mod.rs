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

use rayon::prelude::*;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<u8> {
    let chars: Vec<u8> = input.trim().as_bytes().to_owned();
    return chars;
}

fn collapse(input: impl IntoIterator<Item = u8>, polymer: &mut Vec<u8>) {
    let mut unit = b'\0';
    for next in input {
        if next ^ 0x20 == unit {
            polymer.pop();
            unit = polymer.last().cloned().unwrap_or_default();
        } else {
            unit = next;
            let _ = polymer.push(next);
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &Vec<u8>) -> usize {
    let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
    collapse(input.clone(), &mut polymer);
    return polymer.len();
}

#[aoc(day5, part2)]
fn part2(input: &Vec<u8>) -> usize {
    let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
    (b'A'..b'Z' + 1)
        .map(|letter| {
            polymer.clear();
            let filtered = input.iter().cloned().filter(|b| b & !32 != letter);
            collapse(filtered, &mut polymer);
            polymer.len()
        })
        .min()
        .expect("Couldn't get min")
}

#[aoc(day5, part2, rayon)]
fn part2_rayon(input: &Vec<u8>) -> usize {
    // I was using for loops before I stole this style from CryZe
    (b'A'..b'Z' + 1)
        .into_par_iter()
        .map(|letter| {
            let mut polymer: Vec<u8> = Vec::new();
            let filtered = input.iter().cloned().filter(|b| b & !32 != letter);
            collapse(filtered, &mut polymer);
            polymer.len()
        })
        .min()
        .expect("Couldn't get min")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"dabAcCaCBAcCcaDA
"#;
        assert_eq!(part1(&parse_input(input)), 10);
    }

    #[test]
    fn test_part2() {
        let input = r#"dabAcCaCBAcCcaDA
"#;
        assert_eq!(part2(&parse_input(input)), 4);
    }
}
