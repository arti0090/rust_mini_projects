use crate::file_manager::list_files_in_directory;

mod file_manager;

fn main() {
    let _ = list_files_in_directory("C:/Users/Administrator/Documents");
}
