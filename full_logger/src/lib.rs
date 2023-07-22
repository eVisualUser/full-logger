pub mod file_manager;
pub mod logger;

pub mod libraries {
    pub use chrono;
    pub use fltk;
    pub use pretty_ini;
    pub use toml_edit;
}

#[cfg(test)]
pub mod test {
    use crate::{
        file_manager::{FileManager, FileSize},
        logger::*,
    };

    #[test]
    pub fn global_test() {
        let mut file_manager = FileManager::new(String::from("log"), FileSize::Mo(100));
        file_manager.set_file_prefix_str("DEMO_");
        file_manager.set_file_suffix_str("_LOG");
        file_manager.set_file_extension_str("log");

        let file = file_manager.get_file_path();

        set_file_format(FileFormat::CSV);
        set_allow_console_log(true);
        set_or_create_global_log_file("log", FileSize::Mo(100));

        set_message_box_trigger(Some(String::from("error")));

        simple_log(vec!["error"], "Test").unwrap();
        log(&file, vec!["error"], "Test").unwrap();

        let result: Result<&str, &str> = Ok("Test");
        simple_log_result(vec!["error"], result).unwrap();
        log_result(&file, vec!["error"], result).unwrap();

        simple_log_option(vec!["error"], Some(10));
    }
}
