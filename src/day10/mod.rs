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

use std::fmt::{self, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} Y: {}", self.x_pos, self.y_pos)
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    points: Vec<Point>,
    time: usize,
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        Grid { points, time: 0 }
    }

    fn step(&mut self) {
        for p in &mut self.points {
            p.x_pos += p.x_vel;
            p.y_pos += p.y_vel;
        }
        self.time += 1;
    }

    fn step_backward(&mut self) {
        for p in &mut self.points {
            p.x_pos -= p.x_vel;
            p.y_pos -= p.y_vel;
        }
        self.time -= 1;
    }

    fn get_time(&mut self) -> usize {
        return self.time;
    }

    fn extent(&mut self) -> i32 {
        let xmax = self.points.iter().max_by_key(|x| x.x_pos).unwrap();
        let xmin = self.points.iter().min_by_key(|x| x.x_pos).unwrap();
        return (xmax.x_pos - xmin.x_pos) + (xmax.y_pos - xmin.y_pos);
    }

    fn print_grid(&mut self, width: usize, height: usize) {
        let xmin = self.points.iter().min_by_key(|x| x.x_pos).unwrap().x_pos;
        let ymin = self.points.iter().min_by_key(|x| x.y_pos).unwrap().y_pos;
        let mut buffer: Vec<char> = vec![' '; width * height];
        for p in &mut self.points {
            let x = (p.x_pos - xmin) as usize;
            let y = (p.y_pos - ymin) as usize;
            buffer[x + width * y] = '#';
        }
        for y in 0..height {
            for x in 0..width {
                print! {"{}", buffer[y*width + x]}
            }
            println!("");
        }
    }
}

mod parsers {
    use super::Point;
    use nom::*;
    use std::str::FromStr;

    named!(number(&str) -> i32,
    map_res!(recognize!(pair!(opt!(tag!("-")), digit)), i32::from_str));

    named!(point(&str) -> Point,
    do_parse!(
        tag!("position=<") >>
        opt!(tag!(" ")) >>
        x_pos: number >>
        tag!(", ") >>
        opt!(tag!(" ")) >>
        y_pos: number >>
        tag!("> velocity=<") >>
        opt!(tag!(" ")) >>
        x_vel: number >>
        tag!(", ") >>
        opt!(tag!(" ")) >>
        y_vel: number >>
        tag!(">") >>
        (Point { x_pos, y_pos, x_vel, y_vel })
        )
    );

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl std::str::FromStr for Point {
        type Err = ParseError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match point(input) {
                Ok(("", instruction)) => Ok(instruction),
                _ => Err(ParseError),
            }
        }
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Point> {
    let instructions = input
        .trim()
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>, _>>()
        .unwrap();
    return instructions;
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Point>) -> usize {
    let mut grid = Grid::new(input.clone());
    const WIDTH: usize = 80;
    const HEIGHT: usize = 10;
    let mut extent = 1 << 24;
    loop {
        grid.step();
        let current = grid.extent();
        if current > extent {
            break;
        }
        extent = current;
    }
    grid.step_backward();
    grid.print_grid(WIDTH, HEIGHT);

    return grid.get_time();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;
        assert_eq!(part1(&parse_input(input)), "CABDFE");
    }

    // PART 2 BROKEN. NUM WORKERS WRONG
    // #[test]
    // fn test_part2() {
    // let input = r#"
    // Step C must be finished before step A can begin.
    // Step C must be finished before step F can begin.
    // Step A must be finished before step B can begin.
    // Step A must be finished before step D can begin.
    // Step B must be finished before step E can begin.
    // Step D must be finished before step E can begin.
    // Step F must be finished before step E can begin.
    // "#;
    // assert_eq!(part2(&parse_input(input)), 15);
    // }

}
