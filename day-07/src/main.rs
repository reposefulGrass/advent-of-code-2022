
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
}

fn part_a(input: &str) {
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

    let mut total_size = 0;
    for dir in filesystem.find_all_directories_under_size(100_000) {
        total_size += dir.size();
    }
    println!("The total size of all of the directories fitting the conditions is {}.", total_size);
}
