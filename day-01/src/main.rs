
fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let elf_groups: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut most_calories = 0;

    for group in elf_groups {
        let mut calories: u32 = 0;
        for calories_str in group.lines() {
            calories += calories_str.parse::<u32>().unwrap();
        }   
        if calories > most_calories {
            most_calories = calories;
        }
    }

    println!("The most calories carried is {}.", most_calories);
}

fn part_b(input: &str) {
    let elf_groups: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut elf_calories: Vec<u32> = vec![];

    for group in elf_groups {
        let mut calories: u32 = 0;
        for calories_str in group.lines() {
            calories += calories_str.parse::<u32>().unwrap();
        }
        elf_calories.push(calories);
    }

    elf_calories.sort();
    elf_calories.reverse();

    let most_three_calories: u32 = elf_calories[0..3].iter().sum();
    println!("The total calories of the top 3 elfs is {}.", most_three_calories);
}
