#![allow(static_mut_refs)]

pub mod file_manager;
pub mod logger;
pub mod thread;

pub mod libraries {
    pub use chrono;
    #[cfg(feature="message_box")]
    pub use fltk;
    #[cfg(feature="ini_support")]
    pub use pretty_ini;
}

#[cfg(test)]
pub mod test {
    use crate::{
        file_manager::{FileManager, FileSize},
        logger::*, thread::{flush_log_thread, start_log_thread},
    };

    #[test]
    pub fn global_test() {
        let mut file_manager = FileManager::new(String::from("log"), FileSize::Mo(10));
        file_manager.set_file_prefix_str("DEMO_");
        file_manager.set_file_suffix_str("_LOG");
        file_manager.set_file_extension_str("log");

        let file = file_manager.get_file_path();
        set_or_create_global_log_file(file.as_str());

        set_file_format(FileFormat::CSV);
        set_allow_console_log(true);
        set_log_server("http://localhost:8000".to_string());
        
        start_log_thread(10, 1);

        set_message_box_trigger(Some(String::from("error")));

        simple_log(vec!["error"], "Test");
        log(&file, vec!["error"], "Test");

        let result: Result<&str, &str> = Ok("Test");
        simple_log_result(vec!["error"], result).unwrap();
        log_result(&file, vec!["error"], result).unwrap();

        simple_log_option(vec!["error"], Some(10));
        log_option(&file, vec!["error"], Some(10));

        flush_log_thread(1);
    }
}
