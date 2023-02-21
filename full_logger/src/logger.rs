use chrono::{Datelike, Timelike};
use std::io::Write;

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

#[allow(unused)]
pub fn log(file: &str, location: Vec<&str>, content: &str) {
    match get_file_format() {
        FileFormat::CSV => {
            let mut file = std::fs::File::options().append(true).open(file).unwrap();
            let mut line = format!("{};", chrono::Local::now().to_string());
            for loc in location {
                line.push_str(&format!("{};", loc));
            }
            line.push_str(&format!("{};", content));
            writeln!(&mut file, "{}", line);
        }
        FileFormat::INI => {
            use pretty_ini::{ini, ini_file, variable::Variable};

            let mut ofile = ini_file::IniFile::default();
            ofile.set_path(&file);

            let mut ini = ini::Ini::default();
            ini.load(&mut ofile).unwrap();

            match ini.get_table_ref_mut(location.first().unwrap()) {
                Ok(table) => {
                    let mut log = Variable::default();
                    let now = chrono::Local::now();
                    log.key = format!(
                        "Y{}_M{}_D{}_H{}_M{}_S{}_ML{}",
                        now.year(),
                        now.month(),
                        now.day(),
                        now.hour(),
                        now.minute(),
                        now.second(),
                        now.timestamp_millis()
                    );

                    log.value = String::from(content);

                    table.add_variable(log);
                }
                Err(_) => {
                    ini.create_table(location.first().unwrap());
                    match ini.get_table_ref_mut(location.first().unwrap()) {
                        Ok(table) => {
                            let mut log = Variable::default();
                            let now = chrono::Local::now();
                            log.key = format!(
                                "Y{}_M{}_D{}_H{}_M{}_S{}_ML{}",
                                now.year(),
                                now.month(),
                                now.day(),
                                now.hour(),
                                now.minute(),
                                now.second(),
                                now.timestamp_millis()
                            );

                            log.value = String::from(content);

                            table.add_variable(log);
                        }
                        Err(_) => (),
                    }
                }
            }

            ofile.save(&mut ini);
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

            let now = chrono::Local::now();
            table.insert(
                &format!(
                    "Y{}_M{}_D{}_H{}_M{}_S{}_ML{}",
                    now.year(),
                    now.month(),
                    now.day(),
                    now.hour(),
                    now.minute(),
                    now.second(),
                    now.timestamp_millis()
                ),
                value(content),
            );

            std::fs::write(file, doc.to_string());
        }
        _ => todo!("File Format not supported yet"),
    }
}

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

    log(file, location, &log_content);

    content
}
