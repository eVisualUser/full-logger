use crate::file_manager;
use crate::file_manager::FileSize;
use chrono::{Datelike, Timelike};
use std::io::{ErrorKind, Write};
use curl::easy::Easy;

/// Specify the log file for simple_log/simple_log_result
static mut GLOBAL_LOG_FILE: String = String::new();

/// If true all logs will be printed to the console
static mut ALLOW_CONSOLE_LOG: bool = false;

static mut MESSAGE_BOX_TRIGGER: Option<String> = None;

/// Allow to use logging GUI app: https://github.com/eVisualUser/log-server
static mut LOG_SERVER_URL: String = String::new();

// Define the location keyword to use the message box
pub fn set_message_box_trigger(trigger: Option<String>) {
    unsafe {
        MESSAGE_BOX_TRIGGER = trigger;
    }
}

/// Activate/Deactivate the console printing of the logs
pub fn set_allow_console_log(allowed: bool) {
    unsafe {
        ALLOW_CONSOLE_LOG = allowed;
    }
}

/// Allow to use logging GUI app: https://github.com/eVisualUser/log-server
pub fn set_log_server(url: String) {
    unsafe {
        LOG_SERVER_URL = url;
    }
}

/// Define the file used by simple_log/simple_log_result
pub fn set_or_create_global_log_file(dir: &str, max_size: FileSize) {
    let manager = file_manager::FileManager::new(String::from(dir), max_size);
    unsafe {
        GLOBAL_LOG_FILE = manager.get_file_path();
    }
}

#[derive(Clone)]
pub enum FileFormat {
    CSV,
    INI,
    TOML,
}

static mut FILE_FORMAT: FileFormat = FileFormat::CSV;

pub fn set_file_format(ff: FileFormat) {
    unsafe { FILE_FORMAT = ff }
}

pub fn get_file_format() -> FileFormat {
    unsafe { FILE_FORMAT.clone() }
}

/// Log the input to the global file and can print to console if allowed
pub fn simple_log(location: Vec<&str>, content: &str) -> std::io::Result<()> {
    log(unsafe { &GLOBAL_LOG_FILE }, location, content)
}

/// Log the input to the file and can print to console if allowed
pub fn log(file: &str, location: Vec<&str>, content: &str) -> std::io::Result<()> {
    let now = chrono::Local::now();
    let log_name = format!(
        "Y{}_M{}_D{}_H{}_M{}_S{}_ML{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_millis()
    );
    let _ = now;

    match unsafe { &MESSAGE_BOX_TRIGGER } {
        Some(trigger) => {
            if location.contains(&trigger.as_str()) {
                fltk::dialog::message_title(&log_name);
                fltk::dialog::alert(100, 100, content);
            }
        }
        None => (),
    }

    if file.is_empty() {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            format!("Unable to create a log for {}", file),
        ));
    }

    if unsafe { ALLOW_CONSOLE_LOG } {
        println!("[{}] {}", log_name, content);
    }

    if unsafe { !LOG_SERVER_URL.is_empty() } {
        let request = format!(
            "{}/log/{}/{}/{}",
            unsafe { LOG_SERVER_URL.clone() },
            "full-logger",
            location.first().unwrap_or(&"debug"),
            content
        );

        let mut easy = Easy::new();
        easy.url(request.as_str()).unwrap();

        match easy.perform() {
            Ok(_) => {
                // Do nothing
            }
            Err(e) => {
                eprintln!("Failed to send log to server: {}", e);
                unsafe { LOG_SERVER_URL.clear(); } // Clear the URL to prevent further attempts
            }
        }
    }

    match get_file_format() {
        FileFormat::CSV => {
            let mut file = std::fs::File::options().append(true).open(file).unwrap();
            let mut line = format!("{};", chrono::Local::now().to_string());
            for loc in location {
                line.push_str(&format!("{};", loc));
            }
            line.push_str(&format!("{};", content));
            writeln!(&mut file, "{}", line)?;
        }
        FileFormat::INI => {
            use pretty_ini::{ini, ini_file, variable::Variable};

            let mut ini_file = ini_file::IniFile::default();
            ini_file.set_path(&file);

            let mut ini = ini::Ini::default();
            ini.load(&mut ini_file).unwrap();

            match ini.get_table_ref_mut(location.first().unwrap()) {
                Ok(table) => {
                    let mut log = Variable::default();
                    log.key = log_name;
                    log.value = String::from(content);

                    table.add_variable(log);
                }
                Err(_) => {
                    ini.create_table(location.first().unwrap());
                    match ini.get_table_ref_mut(location.first().unwrap()) {
                        Ok(table) => {
                            let mut log = Variable::default();
                            log.key = log_name;

                            log.value = String::from(content);

                            table.add_variable(log);
                        }
                        Err(_) => (),
                    }
                }
            }

            ini_file.save(&mut ini, None).unwrap();
        }
        FileFormat::TOML => {
            use toml_edit::{value, Document};

            let file_content = std::fs::read_to_string(file.clone()).unwrap();
            let mut doc = file_content.parse::<Document>().expect("invalid xml doc");

            let mut table = doc.as_table_mut();
            for path in location {
                let mut exist = table.contains_table(&path);

                if !exist {
                    table.insert(&path, toml_edit::Item::Table(toml_edit::Table::new()));
                    exist = true;
                }

                if exist {
                    table = table[&path].as_table_mut().unwrap();
                }
            }

            table.insert(&log_name, value(content));

            std::fs::write(file, doc.to_string())?;
        }
    }
    Ok(())
}

/// Log the input to the global file and can print to console if allowed
pub fn simple_log_result<O: std::fmt::Debug, E: std::fmt::Debug>(
    location: Vec<&str>,
    content: Result<O, E>,
) -> Result<O, E> {
    log_result(unsafe { &GLOBAL_LOG_FILE }, location, content)
}

/// Log the input to the file and can print to console if allowed
pub fn log_result<O: std::fmt::Debug, E: std::fmt::Debug>(
    file: &str,
    location: Vec<&str>,
    content: Result<O, E>,
) -> Result<O, E> {
    let log_content;

    match &content {
        Ok(log) => {
            log_content = format!("RESULT_OK_{:?}", log);
        }
        Err(log) => {
            log_content = format!("RESULT_ERR_{:?}", log);
        }
    }

    log(file, location, &log_content).unwrap();

    content
}

/// Log the input to the global file and can print to console if allowed
pub fn simple_log_option<O: std::fmt::Debug>(location: Vec<&str>, content: Option<O>) -> Option<O> {
    log_option(unsafe { &GLOBAL_LOG_FILE }, location, content)
}

/// Log the input to the file and can print to console if allowed
pub fn log_option<O: std::fmt::Debug>(
    file: &str,
    location: Vec<&str>,
    content: Option<O>,
) -> Option<O> {
    let log_content;

    match &content {
        Some(log) => {
            log_content = format!("Some({:?})", log);
        }
        None => {
            log_content = "None".to_owned();
        }
    }

    log(file, location, &log_content).unwrap();

    content
}
