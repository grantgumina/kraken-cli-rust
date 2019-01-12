use std::path::Path;
use std::fs::File;
use std::io::prelude::{Read, Write};

#[derive(Debug)]
pub struct StrReplace {
    data: String,
}

impl StrReplace {
    /// Creates StrReplace from the contents of a file at the given path
    pub fn from_file(path: &str) -> StrReplace {
        let filepath = Path::new(path);
        let mut file = File::open(filepath).unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data).expect("Failed to read file.");

        StrReplace { data: data }
    }

    /// Creates StrReplace from a given &str
    pub fn from_str(str: &str) -> StrReplace {
        StrReplace { data: str.to_string() }
    }

    /// Replace the occurence of one string with another
    /// returns self for chainability.

    pub fn replace(&mut self, search: &str, replacement: &str) -> &mut Self {
        self.data = self.data.replace(search, replacement);
        self
    }

    /// Writes the possibly mutated data to a file at the given destination
    pub fn to_file(&self, dst: &str) {
        let mut file = File::create(dst).unwrap();
        file.write_all(self.data.as_bytes()).expect("Failed to write file.");
    }

    /// Makes a &str out of StrReplace for further use
    pub fn to_str(&self) -> &str {
        &self.data
    }

    /// Replace the occurence of one string with another
    /// without creating a StrReplace
    pub fn replace_here(search: &str, replacement: &str, input: &str) -> String {
        input.replace(search, replacement)
    }
}