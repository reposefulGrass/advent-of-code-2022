
use crate::cwd::CurrentWorkingDirectory;

#[derive(Debug)]
pub struct FileSystem {
    root: Directory,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
    total_size: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Directory {
                name: String::from("/"),
                directories: vec![],
                files: vec![],
                total_size: 0,
            }
        }
    }

    pub fn add_directory(&mut self, cwd: &CurrentWorkingDirectory, dir: Directory) {
        let found_dir = self.find_directory(cwd).unwrap();
        found_dir.directories.push(dir);
    }

    pub fn add_file(&mut self, cwd: &CurrentWorkingDirectory, file: File) {
        let found_dir = self.find_directory(cwd).unwrap();
        found_dir.total_size += file.size;
        found_dir.files.push(file);
        self.root.recalculate_sizes();
    }

    pub fn find_directory(&mut self, cwd: &CurrentWorkingDirectory) -> Option<&mut Directory> {
        self.root.find_directory(cwd.inner())
    }

    pub fn get_all_directories(&self) -> Vec<&Directory> {
        self.root.get_all_directories()
    }

    pub fn size(&self) -> u32 {
        self.root.size()
    }
}

impl Directory {
    pub fn parse(line: &str) -> Result<Directory, &'static str> {
        let delim = line.rfind(" ").ok_or("Invalid Directory Format")?;
        let name = &line[delim..].trim();

        Ok(Directory {
            name: name.to_string(),
            directories: vec![],
            files: vec![],
            total_size: 0,
        })
    }

    fn find_directory(&mut self, cwd: &[&str]) -> Option<&mut Directory> {
        if cwd.is_empty() {
            return None;
        } else if cwd.len() == 1 && cwd[0] == self.name {
            return Some(self);
        } else if cwd[0] == self.name {
            for dir in self.directories.iter_mut() {
                if dir.name == cwd[1] {
                    return dir.find_directory(&cwd[1..]);
                }
            }
        }

        None
    }

    fn get_all_directories(&self) -> Vec<&Directory> {
        let mut dirs = Vec::new();

        for dir in &self.directories {
            dirs.append(&mut dir.get_all_directories());
        }

        dirs.push(self);

        dirs
    }

    fn recalculate_sizes(&mut self) {
        self.total_size = 0;

        for file in &self.files {
            self.total_size += file.size;
        }

        for dir in &mut self.directories {
            dir.recalculate_sizes();
            self.total_size += dir.total_size;
        }
    }

    pub fn size(&self) -> u32 {
        self.total_size
    }
}

impl File {
    pub fn parse(line: &str) -> File {
        let mut s = line.split(" ");
        let size: u32 = s.next().unwrap().parse().unwrap();
        let name = s.next().unwrap();

        File {
            name: name.to_string(),
            size: size,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_directory() {
        let mut filesystem = FileSystem::new();
        let cwd = CurrentWorkingDirectory::new();

        let root_directory = filesystem.root.find_directory(cwd.inner()).unwrap();
        assert_eq!(root_directory.name, "/");
    }

    #[test]
    fn test_add_directory() {
        let mut filesystem = FileSystem::new();
        let mut cwd = CurrentWorkingDirectory::new();

        let new_dir = Directory::parse("dir a").unwrap();
        filesystem.add_directory(&cwd, new_dir);

        println!("{:?}", filesystem.root);

        cwd.change_dir("a");
        let found_dir = filesystem.find_directory(&cwd).unwrap();
        assert_eq!(found_dir.name, "a");
    }
}
