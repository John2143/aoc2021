#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

pub fn read_data() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p3.txt").lines()
}

fn add_number(s: &str, v: &mut [usize]) {
    for (bit, count) in s.chars().zip(v) {
        if bit == '1' {
            *count += 1;
        }
    }
}

fn calc_bins<'a, I: Iterator<Item = &'a str>>(mut data: I) -> (Vec<usize>, usize) {
    let line1 = data.next().unwrap();
    let mut buckets = vec![0; line1.len()];
    add_number(line1, &mut buckets);

    let mut total_count = 1;
    for bits in data {
        add_number(bits, &mut buckets);
        total_count += 1;
    }

    (buckets, total_count)
}

pub fn a() {
    let data = read_data();
    let (bins, total_count) = calc_bins(data);
    let mut gamma = 0;
    let mut epsilon = 0;
    for &bit_count in &bins {
        gamma <<= 1;
        epsilon <<= 1;

        if bit_count > total_count / 2 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    dbg!(gamma * epsilon);
}

enum ReadingType {
    Oxygen,
    CO2,
}

fn get_reading(mut possible_readings: Vec<&str>, read_type: ReadingType) -> usize {
    let mut bit_pos = 0;
    while possible_readings.len() > 1 {
        let (bins, total_count) = calc_bins(possible_readings.iter().copied());

        let most_common_bit = if bins[bit_pos] >= (total_count - bins[bit_pos]) {
            '1'
        } else {
            '0'
        };

        if let ReadingType::Oxygen = read_type {
            possible_readings.retain(|bits| bits.chars().nth(bit_pos) == Some(most_common_bit));
        } else {
            possible_readings.retain(|bits| bits.chars().nth(bit_pos) != Some(most_common_bit));
        }

        bit_pos += 1;
    }

    usize::from_str_radix(possible_readings[0], 2).unwrap()
}

pub fn b() {
    let oxy_gens = read_data().collect_vec(); //most common
    let co2_scrubber = oxy_gens.clone();

    let oxy = get_reading(oxy_gens, ReadingType::Oxygen);
    let co2 = get_reading(co2_scrubber, ReadingType::CO2);

    dbg!(oxy * co2);
    //001100111001
    //110100101111
    //2784375
}
