#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

#[derive(Debug)]
struct VentLine {
    from: (usize, usize),
    to: (usize, usize),
}

impl VentLine {
    fn is_not_diag(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }
    fn apply_to_board(&self, board: &mut [Vec<usize>]) {
        if self.from.0 == self.to.0 {
            let mut start = self.from.1;
            let mut end = self.to.1;
            if end < start {
                std::mem::swap(&mut start, &mut end);
            }
            for y in start..=end {
                board[self.from.0][y] += 1;
            }
        } else if self.from.1 == self.to.1 {
            let mut start = self.from.0;
            let mut end = self.to.0;
            if end < start {
                std::mem::swap(&mut start, &mut end);
            }

            for x in start..=end {
                board[x][self.from.1] += 1;
            }
        } else if self.from.1 < self.to.1 {
            let dist = self.to.0 - self.from.0;
            for offset in 0..=dist {
                board[self.from.0 + offset][self.from.1 + offset] += 1;
            }
        } else {
            let dist = self.to.0 - self.from.0;
            for offset in 0..=dist {
                board[self.from.0 + offset][self.from.1 - offset] += 1;
            }
        }
    }
}

fn parse_coord(input: &str) -> IResult<&str, usize> {
    let (input, x) = digit1(input)?;
    let x = x.parse().unwrap();
    Ok((input, x))
}

fn parse_line(input: &str) -> IResult<&str, VentLine> {
    let (input, x1) = parse_coord(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y1) = parse_coord(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, x2) = parse_coord(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y2) = parse_coord(input)?;

    let line = if x1 <= x2 {
        VentLine {
            from: (x1, y1),
            to: (x2, y2),
        }
    } else {
        VentLine {
            from: (x2, y2),
            to: (x1, y1),
        }
    };

    Ok((input, line))
}

fn read_data() -> impl Iterator<Item = VentLine> {
    include_str!("../data/p5.txt")
        .lines()
        .map(|input| parse_line(input).unwrap().1)
}

pub fn a() {
    let mut board = vec![vec![0usize; 1000]; 1000];
    for line in read_data().filter(VentLine::is_not_diag) {
        line.apply_to_board(&mut board);
    }

    let mut danger = 0;
    for row in board {
        for col in row {
            if col >= 2 {
                danger += 1;
            }
        }
    }

    dbg!(danger);
}

pub fn b() {
    let mut board = vec![vec![0usize; 1000]; 1000];
    for line in read_data() {
        line.apply_to_board(&mut board);
    }

    let mut danger = 0;
    for row in board {
        for col in row {
            if col >= 2 {
                danger += 1;
            }
        }
    }

    dbg!(danger);
}
