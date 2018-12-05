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

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<u8> {
    let chars: Vec<u8> = input.trim().as_bytes().to_owned();
    return chars;
}

fn is_caps(a: u8, b: u8) -> bool {
    if a > b {
        if a - b == 32 {
            return true;
        } else {
            return false;
        }
    } else {
        if b - a == 32 {
            return true;
        } else {
            return false;
        }
    }
}

fn is_let(a: u8, b: u8) -> bool {
    let mut a = a;
    if a >= 97 {
        a -= 32;
    }
    if a - 65 == b {
        return true;
    } else {
        return false;
    }
}

#[aoc(day5, part1)]
fn part1(input: &Vec<u8>) -> usize {
    let mut next = 1;
    let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
    polymer.push(input[0]);
    while next < input.len() {
        'inner: while is_caps(*polymer.last().unwrap(), input[next]) && next < input.len() - 1 {
            next += 1;
            polymer.pop();
            if polymer.is_empty() {
                break 'inner;
            }
        }
        polymer.push(input[next]);
        next += 1;
    }
    return polymer.len();
}

#[aoc(day5, part2)]
fn part2(input: &Vec<u8>) -> usize {
    let mut pol_size: [usize; 26] = [0; 26];
    for (i, x) in pol_size.iter_mut().enumerate() {
        let mut next = 1;
        let mut polymer: Vec<u8> = Vec::with_capacity(input.len());
        polymer.push(input[0]);
        'outer: while next < input.len() {
            'inner: while is_caps(*polymer.last().unwrap(), input[next]) && next < input.len() - 1 {
                next += 1;
                polymer.pop();
                if polymer.is_empty() {
                    break 'inner;
                }
            }
            polymer.push(input[next]);
            if is_let(*polymer.last().unwrap(), i as u8) {
                polymer.pop();
                if polymer.is_empty() {
                    next += 1;
                    polymer.push(input[next]);
                }
            }
            next += 1;
        }
        *x = polymer.len();
    }
    return *pol_size.iter().min().unwrap();
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
