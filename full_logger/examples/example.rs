use full_logger::{file_manager::FileManager, logger::*};

fn main() {
    let mut file_manager = FileManager::new(String::from("log"), 1000000);
    file_manager.set_file_prefix_str("DEMO_");
    file_manager.set_file_suffix_str("_LOG");
    file_manager.set_file_extension_str("log");

    let file = file_manager.get_file_path();

    set_file_format(FileFormat::CSV);

    log(&file, vec!["error"], "Test");

    let result: Result<&str, &str> = Ok("Test");
    log_result(&file, vec!["error"], result).unwrap();
}
