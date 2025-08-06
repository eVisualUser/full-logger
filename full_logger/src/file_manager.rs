use std::collections::HashMap;

use chrono::{Datelike, Timelike};

pub enum FileSize {
    O(u64),
    Ko(u64),
    Mo(u64),
}

/// A struct to manage log files, including their creation and management based on size.
pub struct FileManager {
    pub dir: String,
    pub files: HashMap<String, u64>,
    pub max_size: u64,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub file_extension: String,
}

impl FileManager {
    pub fn new(dir: String, max_size: FileSize) -> Self {
        let mut files = HashMap::<String, u64>::new();

        let max_size = match max_size {
            FileSize::O(size) => size,
            FileSize::Ko(size) => size * 1024,
            FileSize::Mo(size) => size * 1048576,
        };

        if !std::path::Path::new(&dir).exists() {
            std::fs::create_dir(&dir).unwrap();
        }

        for entry in std::fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                files.insert(
                    path.to_str().unwrap().to_string(),
                    std::fs::metadata(&path).unwrap().len(),
                );
            }
        }

        Self {
            dir,
            files,
            max_size,
            prefix: None,
            suffix: None,
            file_extension: String::from("log"),
        }
    }

    /// Returns the path of a file that is less than the maximum size.
    pub fn get_file_path(&self) -> String {
        for file in &self.files {
            if file.1 < &self.max_size {
                return file.0.clone();
            }
        }

        let now = chrono::Local::now();
        let file_name = format!(
            "{}/{}Y{}_M{}_D{}-H{}_M{}_S{}_ML{}{}.{}",
            self.dir,
            match &self.prefix {
                Some(prefix) => prefix.clone(),
                None => String::new(),
            },
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_millis(),
            match &self.suffix {
                Some(suffix) => suffix.clone(),
                None => String::new(),
            },
            self.file_extension
        );
        std::fs::File::options()
            .create(true)
            .write(true)
            .open(file_name.clone())
            .unwrap();

        file_name
    }

    /// Set the file prefix for the log files.
    pub fn set_file_prefix(&mut self, prefix: String) {
        self.prefix = Some(prefix);
    }

    /// Set the file prefix for the log files.
    pub fn set_file_prefix_str(&mut self, prefix: &str) {
        self.set_file_prefix(prefix.to_owned());
    }

    /// Set the file suffix for the log files.
    pub fn set_file_suffix(&mut self, suffix: String) {
        self.suffix = Some(suffix);
    }

    /// Set the file suffix for the log files.
    pub fn set_file_suffix_str(&mut self, suffix: &str) {
        self.set_file_suffix(suffix.to_owned());
    }

    /// Set the file extention for the log files.
    pub fn set_file_extension(&mut self, ext: String) {
        self.file_extension = ext;
    }

    /// Set the file extention for the log files.
    pub fn set_file_extension_str(&mut self, ext: &str) {
        self.set_file_extension(ext.to_owned());
    }
}
