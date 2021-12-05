#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

struct BingoData {
    called_numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

#[derive(Clone, Debug)]
struct BingoBoard {
    numbers: [usize; 25],
}

#[derive(Clone)]
pub struct BingoMask {
    mask: [bool; 25],
    has_won: bool,
}

impl BingoMask {
    fn new() -> Self {
        Self {
            mask: Default::default(),
            has_won: false,
        }
    }
    fn is_solved(&self) -> bool {
        for row in self.mask.chunks_exact(5) {
            if row.iter().all(|&f| f) {
                return true;
            }
        }

        for col in 0..5 {
            if [
                self.mask[col],
                self.mask[col + 5],
                self.mask[col + 10],
                self.mask[col + 15],
                self.mask[col + 20],
            ]
            .iter()
            .all(|&f| f)
            {
                return true;
            }
        }

        false
    }
}

impl BingoBoard {
    fn check_called(&self, num: usize) -> Option<usize> {
        self.numbers.iter().position(|&s| s == num)
    }

    fn calc_value(&self, mask: &BingoMask) -> usize {
        self.numbers
            .iter()
            .zip(mask.mask)
            .filter_map(|(&num, is_masked)| {
                if is_masked {
                    return None;
                }

                Some(num)
            })
            .sum()
    }
}

fn read_data() -> BingoData {
    let mut board_data = include_str!("../data/p4.txt").lines();

    let called_numbers = board_data.next().unwrap();
    let called_numbers = called_numbers
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let mut boards = vec![];

    let mut current_board = None;
    let mut board_index = 0;
    for line in board_data {
        match current_board {
            None => {
                current_board = Some(BingoBoard {
                    numbers: Default::default(),
                });
                board_index = 0;
            }
            Some(ref mut board) => {
                for (offset, num) in line
                    .split(" ")
                    .filter(|d| d.len() > 0)
                    .map(|x| x.parse().unwrap())
                    .enumerate()
                {
                    board.numbers[board_index * 5 + offset] = num
                }

                board_index += 1;
                if board_index == 5 {
                    boards.push(current_board.take().unwrap());
                }
            }
        }
    }

    BingoData {
        called_numbers,
        boards,
    }
}

pub fn a() {
    let bingo_data = read_data();
    let mut masks = vec![BingoMask::new(); bingo_data.boards.len()];
    for called_number in bingo_data.called_numbers {
        for (board, mut board_mask) in bingo_data.boards.iter().zip(&mut masks) {
            if let Some(pos) = board.check_called(called_number) {
                board_mask.mask[pos] = true;
                if board_mask.is_solved() {
                    dbg!(called_number * board.calc_value(&board_mask));
                    return;
                }
            }
        }
    }
}
pub fn b() {
    let bingo_data = read_data();
    let mut masks = vec![BingoMask::new(); bingo_data.boards.len()];
    let mut last_winning_board_score = 0;

    for called_number in bingo_data.called_numbers {
        for (board, mut board_mask) in bingo_data.boards.iter().zip(&mut masks) {
            if board_mask.has_won {
                continue;
            }

            if let Some(pos) = board.check_called(called_number) {
                board_mask.mask[pos] = true;
                if board_mask.is_solved() {
                    board_mask.has_won = true;
                    last_winning_board_score = called_number * board.calc_value(&board_mask);
                }
            }
        }
    }

    dbg!(last_winning_board_score);
}
