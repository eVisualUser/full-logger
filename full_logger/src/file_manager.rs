use std::collections::HashMap;

use chrono::{Datelike, Timelike};

pub struct FileManager {
    pub dir: String,
    pub files: HashMap<String, u64>,
    pub max_size: u64,
}

impl FileManager {
    pub fn new(dir: String, max_size: u64) -> Self {
        let mut files = HashMap::<String, u64>::new();

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
        }
    }

    pub fn get_file_path(&self) -> String {
        for file in &self.files {
            if file.1 < &self.max_size {
                return file.0.clone();
            }
        }

        let now = chrono::Local::now();
        let file_name = format!(
            "{}/Y{}_M{}_D{}-{}-{}-{}_ML{}.log",
            self.dir,
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_millis()
        );
        std::fs::File::options()
            .create(true)
            .write(true)
            .open(file_name.clone())
            .unwrap();

        file_name
    }
}
