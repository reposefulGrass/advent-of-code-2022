
fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let mut total_priority = 0;

    for rucksack in input.lines() {
        let (compartment_a, compartment_b) = rucksack.split_at(rucksack.len() / 2);
        let shared_item = get_shared_item_from_pair(compartment_a, compartment_b).unwrap();
        total_priority += priority(shared_item) as u32;
    }

    println!("[Part A] The total priority is {}.", total_priority);
}

fn part_b(input: &str) {
    let mut total_priority = 0;
    let elves: Vec<&str> = input.lines().collect();
    
    for elf_i in elves.chunks(3) {
        let shared_item = get_shared_item_from_trio(elf_i[0], elf_i[1], elf_i[2]).unwrap();
        total_priority += priority(shared_item) as u32;
    }

    println!("[Part B] The total priority is {}.", total_priority);
}

fn priority(c: char) -> u8 {
    if c.is_ascii_uppercase() {
        (c as u8) - ('A' as u8) + 27
    } else if c.is_ascii_lowercase() {
        (c as u8) - ('a' as u8) + 1
    } else {
        0
    }
}

fn get_shared_item_from_pair(a: &str, b: &str) -> Option<char> {
    for character in a.chars() {
        if b.contains(character) {
            return Some(character);
        }
    }

    None
}

fn get_shared_item_from_trio(a: &str, b: &str, c: &str) -> Option<char> {
    for character in a.chars() {
        if b.contains(character) && c.contains(character) {
            return Some(character);
        }
    }

    None
}

#[test]
fn test_split_in_half() {
    assert_eq!(split_in_half("abcd"), ("ab", "cd"));
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1_u8);
    assert_eq!(priority('z'), 26_u8);
    assert_eq!(priority('A'), 27_u8);
    assert_eq!(priority('Z'), 52_u8);
}

#[test]
fn test_get_shared_item_pair() {
    assert_eq!(get_shared_item_from_pair("vJrwpWtwJgWr", "hcsFMMfFFhFp"), Some('p'));
}

#[test]
fn test_get_shared_item_tri() {
    assert_eq!(
        get_shared_item_from_trio(
            "vJrwpWtwJgWrhcsFMMfFFhFp", 
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ), 
        Some('r')
    );
}