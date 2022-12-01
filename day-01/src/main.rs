
fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let mut elves = elf_calories(input);
    println!("[Part A] The most calories carried is {}.", elves[0]);
}

fn part_b(input: &str) {
    let mut elves = elf_calories(input);
    println!(
        "[Part B] The total calories of the top 3 elfs is {}.",
        elves[0..3].iter().sum()
    );
}

fn elf_calories(input: &str) -> Vec<u32> {
    input.split("\r\n\r\n")
        .map(|elf| elf.lines()
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum())
        .collect()
        .sort()
        .reverse()
}

#[test]
fn test_elf_calories() {
    let input = include_str!("../input/test.txt");
    assert_eq!(elf_calories(input), vec![6_000, 4_000, 11_000, 24_000, 10_000]);
}