use full_logger::{file_manager::{FileManager, FileSize}, logger::*};

fn main() {
    let mut file_manager = FileManager::new(String::from("log"), FileSize::Mo(100));
    file_manager.set_file_prefix_str("DEMO_");
    file_manager.set_file_suffix_str("_LOG");
    file_manager.set_file_extension_str("log");

    let file = file_manager.get_file_path();

    set_file_format(FileFormat::CSV);
    set_allow_console_log(true);
    set_or_create_global_log_file("log", FileSize::Mo(100));

    simple_log(vec!["error"], "Test");
    log(&file, vec!["error"], "Test");

    let result: Result<&str, &str> = Ok("Test");
    simple_log_result(vec!["error"], result).unwrap();
    log_result(&file, vec!["error"], result).unwrap();
}
