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

fn collapse(input: &mut Vec<u8>) -> usize {
    let mut new_len = 0;
    for i in 0..input.len() {
        if new_len > 0 && input[new_len - 1] ^ 0x20 == input[i] {
            new_len -= 1;
        } else {
            input[new_len] = input[i];
            new_len += 1;
        }
    }
    return new_len;
}

#[aoc(day5, part1)]
fn part1(input: &Vec<u8>) -> usize {
    let mut input = input.clone();
    let len = collapse(&mut input);
    return len;
}

// #[aoc(day5, part1, rayon)]
// fn part1_rayon(input: &Vec<u8>) -> usize {
// let split_poly: Vec<u8> = input
// .par_chunks(32)
// .map(|n| {
// let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
// collapse(n.iter().cloned(), &mut polymer);
// polymer
// })
// .reduce(
// || Vec::new(),
// |mut a: Vec<u8>, b: Vec<u8>| {
// a.extend(b);
// a
// },
// );
// let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
// collapse(split_poly, &mut polymer);
// return polymer.len();
// }

#[aoc(day5, part2)]
fn part2(input: &Vec<u8>) -> usize {
    let mut input = input.clone();
    let reduced_len = collapse(&mut input);
    // I was using for loops before I stole this style from CryZe
    (b'a'..b'z' + 1)
        .into_par_iter()
        .map(|letter| {
            let mut new_len = 0;
            let mut input_clone = [0; 1 << 16];
            for i in 0..reduced_len {
                if input[i] | 0x20 == letter {
                    continue;
                }

                if new_len > 0 && input_clone[new_len - 1] ^ 0x20 == input[i] {
                    new_len -= 1;
                } else {
                    input_clone[new_len] = input[i];
                    new_len += 1;
                }
            }
            new_len
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
