#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

pub fn read_data() -> impl Iterator<Item = i32> {
    include_str!("../data/p1.txt")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
}

pub fn a() {
    let readings = read_data();
    let mut num_increasing = 0;

    for (last_depth, depth) in readings.tuple_windows() {
        if depth > last_depth {
            num_increasing += 1;
        }
    }

    println!("Num increasing: {}", num_increasing);
}
pub fn b() {
    let readings = read_data();
    let mut num_increasing = 0;

    for (a, b, c, depth) in readings.tuple_windows() {
        if (b + c + depth) > (a + b + c) {
            num_increasing += 1;
        }
    }

    println!("Num increasing windowed: {}", num_increasing);
}
