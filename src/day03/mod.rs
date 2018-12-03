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

use hashbrown::{HashMap, HashSet};
use matrix::format::conventional::Conventional;
use std::str;

pub struct LandClaim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<LandClaim> {
    let mut input_vec: Vec<LandClaim> = Vec::new();
    for line in input.lines() {
        let all_the_things = line
            .split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<usize>().ok())
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
    let mut claims = HashMap::new();
    for claim in input.iter() {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                *claims.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    let intersected_claims = claims.values().filter(|v| **v > 1).count();
    return intersected_claims;
}

#[aoc(day3, part2)]
fn part2(input: &Vec<LandClaim>) -> usize {
    let mut claims = HashMap::<_, usize>::new();
    let mut all_ids = HashSet::new();
    for claim in input.iter() {
        let mut overlap = false;
        all_ids.insert(claim.id);
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if let Some(old) = claims.insert((x, y), claim.id) {
                    overlap = true;
                    all_ids.remove(&old);
                }
            }
        }
        if !overlap {
            all_ids.insert(claim.id);
        }
    }

    match all_ids.iter().next() {
        Some(id) => *id,
        None => 0,
    }
}
