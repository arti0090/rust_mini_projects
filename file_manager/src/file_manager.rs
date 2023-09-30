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
}