#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

pub fn read_data() -> impl Iterator<Item = (&'static str, i32)> {
    include_str!("../data/p2.txt").lines().map(|x| {
        let (command, amt) = x.split_once(" ").unwrap();
        let amt = amt.parse().unwrap();
        (command, amt)
    })
}

pub fn a() {
    let mut depth = 0;
    let mut distance = 0;

    for (command, amt) in read_data() {
        match command {
            "forward" => distance += amt,
            "down" => depth += amt,
            "up" => depth -= amt,
            _ => panic!("invalid command"),
        }
    }

    dbg!(depth, distance);
    println!("depth x distance = {}", depth * distance);
}

pub fn b() {
    let mut depth = 0;
    let mut aim = 0;
    let mut distance = 0;

    for (command, amt) in read_data() {
        match command {
            "forward" => {
                distance += amt;
                depth += aim * amt;
            }
            "down" => aim += amt,
            "up" => aim -= amt,
            _ => panic!("invalid command"),
        }
    }

    dbg!(depth, distance);
    println!("depth x distance = {}", depth * distance);
}
