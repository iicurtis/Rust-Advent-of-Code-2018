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

use nom::alpha;
use util::num;
use std::io::{self, BufRead};

pub fn solve() {
    // Get line from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines();

    let mut registers = [0, 26];

    for line in input {
        let line = line.unwrap();

        ws!(line.as_str(), do_parse!(
                reg1: alpha >>
                alt!(
                    tag_s!("inc") | tag_s!("dec")
                ) >>
                val1: num >>
                tag_s!("if") >>


        ))
        println!("{}", reg[0]);
    }

    println!(
        "[Day 08][Part 1] ANS is: {}",
        registers.iter().max().unwrap()
    );

    // println!("[Day 08][Part 2] ANS is: {}", captcha2.to_string());
}
