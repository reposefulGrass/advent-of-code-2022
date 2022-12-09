
use lazy_static::lazy_static;
use std::ops::RangeInclusive;
use regex::Regex;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let mut total = 0;

    for line in input.lines() {
        let (range_a, range_b) = line_to_ranges(line);
        if contains_one_another(range_a, range_b) {
            total += 1;
        }
    }

    println!("[part A] There are {total} total assignment pairs that contain the other.");
}

fn part_b(input: &str) {
    let mut total = 0;

    for line in input.lines() {
        let (range_a, range_b) = line_to_ranges(line);
        if overlaps(range_a.clone(), range_b.clone()) {
            total += 1;
        }
    }

    println!("[Part B] There are {total} total overlaps.");
}

fn line_to_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let caps = PATTERN.captures(line).unwrap();
    let a: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let b: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let c: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
    let d: u32 = caps.get(4).unwrap().as_str().parse().unwrap();

    (a..=b, c..=d)
}

fn contains_one_another(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    return is_within(a.clone(), b.clone()) || is_within(b.clone(), a.clone());
}

fn is_within(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    for item in a {
        if !b.contains(&item) {
            return false;
        }
    }

    true
}

fn overlaps(a: RangeInclusive<u32>, b: RangeInclusive<u32>) -> bool {
    let a: Vec<u32> = a.clone().into_iter().collect();
    let (&a_first, &a_last) = (a.first().unwrap(), a.last().unwrap());
    let b: Vec<u32> = b.clone().into_iter().collect();
    let (&b_first, &b_last) = (b.first().unwrap(), b.last().unwrap());

    (a_last >= b_first && a_last <= b_last) || (b_last >= a_first && b_last <= a_last)
}
