use std::{fs, io};

pub struct FileManager;

impl FileManager {
    pub fn list_files_in_directory(directory: &str) -> io::Result<()> {
        let entries = fs::read_dir(directory)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        println!("{}", file_name_str);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn sort_files_in_directory(directory: &str, direction: &str) -> io::Result<()> {
        let entries = fs::read_dir(directory)?;

        let mut files_with_lengths: Vec<(String, u64)> = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // check for files and their metadata, push this data to Vector (array) of name and len
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if let Ok(metadata) = fs::metadata(&path) {
                            files_with_lengths.push((file_name_str.to_string(), metadata.len()));
                        }
                    }
                }
            }

            match direction {
                // sort vector by the length in ascending or desc order
                "DESC" => files_with_lengths.sort_by_key(|&(_, length)| std::cmp::Reverse(length)),
                _ => files_with_lengths.sort_by_key(|&(_, length)| length),
            }

            for (file_name, length) in &files_with_lengths {
                println!("{}: {}", file_name, length);
            }
        }

        Ok(())
    }
}