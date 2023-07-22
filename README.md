# Full Logger
Easy to use logger for Rust

## Features

- [X] Support few file formats.
- [X] Console log.
- [X] Message box using [fltk](https://crates.io/crates/fltk).
- [X] Support logs of Result and Option.

## Formats

### INI

Full support

#### Example

```ini
[error]
Y2023_M2_D21_H18_M59_S27_ML1677002367767 = Test
Y2023_M2_D21_H18_M59_S27_ML1677002367775 = RESULT_OK_"Test"
```

### TOML

Full support

#### Example

```toml
[error]
Y2023_M2_D21_H18_M57_S50_ML1677002270145 = "Test"
Y2023_M2_D21_H18_M57_S50_ML1677002270150 = "RESULT_OK_\"Test\""
```

### CSV

Full support

#### Example

```csv
2023-02-21 19:00:17.771215200 +01:00;error;Test;
2023-02-21 19:00:17.777826800 +01:00;error;RESULT_OK_"Test";
```

## Getting-Started

1. Manage Log Files
```rust
let working_dir = String::from("log");
let max_file_size = FileSize::Mo(100);
let file_manager = FileManager::new(String::from("log"), max_file_size);

// Get file path to a file under max_file_size (create one if necessary)
let file = file_manager.get_file_path();

// Allow console printing
set_allow_console_log(true);

// Setup simple logs
set_or_create_global_log_file("log", FileSize::Mo(100));

// Add a message box when the log is an error.
set_message_box_trigger("error");
```

2. Set the file format

```rust
// Do not need other code
set_file_format(FileFormat::CSV);
```

3. Time to Log
```rust
let path = vec!["error", "debug"];
let message = "Test";

let mut error = Result::<i32, i32>::Ok(12);

log(file, path, message);
error = log_result(file, path, error);

// Or you can use the simple way (must have defined a global file before)
simple_log(log, path, message);
error = simple_log_result(log, path, error);
```

4. Enjoy

Here it's the result of a log calling ```log(file, path, message);```

```csv
2023-02-21 19:00:17.771215200 +01:00;error;debug;Test;
```
