
pub enum Command<'a> {
    ChangeDirectory(&'a str),
    ListDirectory,
}

impl<'a> Command<'a> {
    pub fn parse_line(line: &'a str) -> Result<Command, &'static str> {
        if line.contains("cd") {
            let path_beginning = line.rfind(" ").ok_or("Improper Formatting")? as usize;
            let path = &line[path_beginning..].trim();
            return Ok(Command::ChangeDirectory(path));
        } else if line.contains("ls") {
            return Ok(Command::ListDirectory);
        } 

        Err("Command Not Recognized")
    }
}