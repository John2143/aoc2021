#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

pub fn read_data() -> impl Iterator<Item = usize> {
    include_str!("../data/p6.txt")
        .trim_end()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
}

pub fn _sim_ez(num_days: usize) -> usize {
    let mut pool = read_data().collect_vec();

    for _day in 0..num_days {
        let mut new_fish = 0;
        for fish_age in &mut pool {
            if *fish_age == 0 {
                *fish_age = 6;
                new_fish += 1;
            } else {
                *fish_age -= 1;
            }
        }

        pool.extend(std::iter::repeat(8).take(new_fish));
    }

    println!("{} days = {}", num_days, pool.len());

    pool.len()
}

pub fn sim(num_days: usize) -> usize {
    let mut buckets = vec![0; 9];
    for fish in read_data() {
        buckets[fish] += 1;
    }

    for _day in 0..num_days {
        let spawning = buckets[0];

        for x in 0..(buckets.len() - 1) {
            buckets[x] = buckets[x + 1];
        }
        buckets[6] += spawning;
        buckets[8] = spawning;
    }

    buckets.iter().sum()
}

pub fn a() {
    let s = sim(80);
    assert_eq!(s, 380243);
}
pub fn b() {
    dbg!(sim(256));
}
