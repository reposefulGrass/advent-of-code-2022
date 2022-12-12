
#[derive(Debug)]
pub struct CurrentWorkingDirectory<'a>(Vec<&'a str>);

impl<'a> CurrentWorkingDirectory<'a> {
    pub fn new() -> Self {
        CurrentWorkingDirectory(vec![])
    }

    pub fn change_dir(&mut self, path: &'a str) {
        if path == ".." {
            self.0.pop();
        } else if path == "/" {
            self.0.clear();
            self.0.push("/");
        } else {
            self.0.push(path);
        }
    }

    pub fn inner(&self) -> &Vec<&'a str> {
        &self.0
    }
}
