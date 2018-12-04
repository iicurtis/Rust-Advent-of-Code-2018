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

use hashbrown::HashSet;
use std::str;

#[derive(Debug)]
pub struct LandClaim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<LandClaim> {
    let mut input_vec: Vec<LandClaim> = Vec::new();
    for line in input.lines() {
        let all_the_things = line
            .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<u16>().ok())
            .collect::<Vec<_>>();
        let id = all_the_things[0];
        let xpos = all_the_things[1];
        let ypos = all_the_things[2];
        let xsize = all_the_things[3];
        let ysize = all_the_things[4];
        input_vec.push(LandClaim {
            id: id,
            x: xpos,
            y: ypos,
            width: xsize,
            height: ysize,
        })
    }
    return input_vec;
}

#[aoc(day3, part1)]
fn part1(input: &Vec<LandClaim>) -> usize {
    let mut claims: Vec<usize> = vec![0; 1 << 20];
    for claim in input.iter() {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                claims[x as usize + y as usize * 1024] += 1;
            }
        }
    }
    let intersected_claims = claims.iter().filter(|v| **v > 1).count();
    return intersected_claims;
}

#[aoc(day3, part2)]
fn part2(input: &Vec<LandClaim>) -> usize {
    let mut claims: Vec<u16> = vec![0; 1 << 20];
    let mut all_ids = HashSet::new();
    for claim in input.iter() {
        let mut overlap = false;
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                let idx = x as usize + y as usize * 1024;
                if claims[idx] == 0 {
                    claims[idx] = claim.id;
                } else {
                    overlap = true;
                    all_ids.remove(&claims[idx]);
                }
            }
        }
        if !overlap {
            all_ids.insert(claim.id);
        }
    }

    match all_ids.iter().next() {
        Some(id) => *id as usize,
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_part1() {
    // let input = r#"
    // #1 @ 1,3: 4x4
    // #2 @ 3,1: 4x4
    // #3 @ 5,5: 2x2
    // "#;
    // assert_eq!(part1(&parse_input(input)), 4);
    // }

    // #[test]
    // fn test_part2() {
    // let input = r#"
    // #1 @ 1,3: 4x4
    // #2 @ 3,1: 4x4
    // #3 @ 5,5: 2x2
    // "#;
    // assert_eq!(part2(&parse_input(input)), 3);
    // }

}
