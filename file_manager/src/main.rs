use crate::file_manager::FileManager;

mod file_manager;

fn main() {
    let directory = "C:/Users/Administrator/Documents";

    let _ = FileManager::list_files_in_directory(directory);
}
