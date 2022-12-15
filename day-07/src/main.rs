
mod cwd;
mod command;
mod filesystem;
mod parsing_state;

use parsing_state::ParsingState::*;
use cwd::CurrentWorkingDirectory;
use command::Command::{self, *};
use filesystem::{FileSystem, Directory, File};


fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    const MAX_SIZE: u32 = 100_000;

    let filesystem = parse_input(input);

    let mut total_size = 0;
    for dir in filesystem.get_all_directories() {
        if dir.size() < MAX_SIZE {
            total_size += dir.size();
        }
    }

    println!("[Part A] The total size of all of the directories fitting the conditions is {}.", total_size);
}

fn part_b(input: &str) {
    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const UNUSED_DISK_SPACE: u32 = 30_000_000;
    const TARGET_DISK_SPACE: u32 = TOTAL_DISK_SPACE - UNUSED_DISK_SPACE;

    let filesystem = parse_input(input);
    let filesystem_size = filesystem.size();

    let mut sorted_directories = filesystem.get_all_directories();
    sorted_directories.sort_by_key(|d| d.size());

    let mut selected_directory = None;
    for dir in sorted_directories {
        if filesystem_size - dir.size() < TARGET_DISK_SPACE {
            selected_directory = Some(dir);
            break;
        }
    }

    println!("[Part B] The directory we need to remove has {} bytes.", selected_directory.unwrap().size());
}

fn parse_input(input: &str) -> FileSystem {
    let mut filesystem = FileSystem::new();
    let mut cwd = CurrentWorkingDirectory::new();
    let mut state = ParsingCommand;

    for line in input.lines() {
        if line.starts_with("$") {
            state = ParsingCommand;
        }

        match state {
            ParsingCommand => {
                let command = Command::parse_line(line).expect("Proper Formatting");

                match command {
                    ChangeDirectory(path) => {
                        cwd.change_dir(path);
                    },
                    ListDirectory => {
                        state = ParsingOutput;
                    },
                }
            },
            ParsingOutput => {
                if line.starts_with("dir") {
                    let dir = Directory::parse(line).unwrap();
                    filesystem.add_directory(&cwd, dir);
                } else {
                    let file = File::parse(line);
                    filesystem.add_file(&cwd, file);
                }
            },
        }
    }

    filesystem
}
