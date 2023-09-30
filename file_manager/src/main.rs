use crate::file_manager::FileManager;

mod file_manager;

fn main() {
    let directory = "C:/Users/Administrator/Documents";

    let _ = FileManager::list_files_in_directory(directory);

    // direction could be an Enum in future
    let _ = FileManager::sort_files_in_directory(directory, "ASC");
    let _ = FileManager::sort_files_in_directory(directory, "DESC");
}
