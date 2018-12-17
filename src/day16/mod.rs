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

#[derive(Debug, Clone, Copy)]
struct Instruction([u8; 4]);

#[derive(Debug, Clone, Copy)]
struct Register([u8; 4]);

#[derive(Debug, Clone, Copy)]
struct Sample {
    before: Register,
    instr: Instruction,
    after: Register,
}

#[derive(Debug, Clone)]
struct Input {
    instructions: Vec<Instruction>,
    samples: Vec<Sample>,
}

use nom::*;
use std::str::FromStr;

named!(number(&str) -> u8, map_res!(recognize!(digit), u8::from_str));

named!(before(&str) -> Register, do_parse!( tag!("Before: [") >>
    reg0: number >> tag!(", ") >> reg1: number >> tag!(", ") >> reg2: number >>
    tag!(", ") >> reg3: number >> tag!("]\n") >>
    (Register([reg0, reg1, reg2, reg3]))));

named!(after(&str) -> Register,
do_parse!(
    tag!("After:  [") >>
    reg0: number >>
    tag!(", ") >>
    reg1: number >>
    tag!(", ") >>
    reg2: number >>
    tag!(", ") >>
    reg3: number >>
    tag!("]\n") >>
    (Register([reg0, reg1, reg2, reg3]))));

named!(instruction(&str) -> Instruction,
do_parse!(
    op: number >>
    tag!(" ") >>
    A: number >>
    tag!(" ") >>
    B: number >>
    tag!(" ") >>
    C: number >>
    opt!(tag!("\n")) >>
    (Instruction([op, A, B, C]))));

named!(sample(&str) -> Sample,
do_parse!(
    before: before >>
    instr: instruction >>
    after: after >>
    tag!("\n") >>
    (Sample {before, instr, after})
    ));

named!(instr(&str) -> Input,
do_parse!(
    // samples: many1!( sample ) >>
    // tag!("\n\n") >>
    instructions: many0!( instruction ) >>
    ( Input{ samples, instructions } )
    ));

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<Sample> {
    println!("{:?}", instr(input));
    return Vec::new();
}

#[aoc(day16, part1)]
fn part1(input: &Vec<Sample>) -> usize {
    for i in input.iter() {
        println!("{:?}", i);
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "9";
        assert_eq!(part1(input), "5158916779");
    }

    #[test]
    fn test_part1_1() {
        let input = "5";
        assert_eq!(part1(input), "0124515891");
    }

    #[test]
    fn test_part1_2() {
        let input = "18";
        assert_eq!(part1(input), "9251071085");
    }

    #[test]
    fn test_part1_3() {
        let input = "2018";
        assert_eq!(part1(input), "5941429882");
    }

    #[test]
    fn test_part2() {
        let input = "51589";
        assert_eq!(part2(input), 9);
    }

    #[test]
    fn test_part2_1() {
        let input = "01245";
        assert_eq!(part2(input), 5);
    }

    #[test]
    fn test_part2_2() {
        let input = "92510";
        assert_eq!(part2(input), 18);
    }

    #[test]
    fn test_part2_3() {
        let input = "59414";
        assert_eq!(part2(input), 2018);
    }

}
