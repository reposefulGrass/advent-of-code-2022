
fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn elf_calories(input: &str) -> Vec<u32> {
    input.split("\r\n\r\n")
        .map(|elf| elf.lines()
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum())
        .collect()
}

fn part_a(input: &str) {
    let mut calories = elf_calories(input);
    calories.sort();
    calories.reverse();

    println!("The most calories carried is {}.", calories[0]);
}

fn part_b(input: &str) {
    let mut calories = elf_calories(input);
    calories.sort();
    calories.reverse();

    let most_three_calories: u32 = calories[0..3].iter().sum();
    println!("The total calories of the top 3 elfs is {}.", most_three_calories);
}

#[test]
fn test_elf_calories() {
    let input = include_str!("../input/test.txt");
    assert_eq!(elf_calories(input), vec![6_000, 4_000, 11_000, 24_000, 10_000]);
}