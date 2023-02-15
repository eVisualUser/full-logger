# Full Logger
Easy to use logger for Rust

## Formats

### INI

Full support

#### Example
```ini
[error]
ERROR_Y2023_M2_D15_H22_M44_S6_ML1676497446493 = Test
```

### TOML

Full support

#### Example
```toml
[error]

[error.file]

[error.file.debug]
ERROR_Y2023_M2_D15_H22_M45_S3_ML1676497503389 = "Test"
```

### CSV

Full support

#### Example

```csv
2023-02-15 22:46:05.745414400 +01:00;ERROR;Test;
```

## Short-Tutorial

1. Manage Log Files
```rust
let working_dir = String::from("log");
let max_file_size = 1000000; // 1000000o
let file_manager = FileManager::new(String::from("log"), max_file_size);

// Get file path to a file under max_file_size (create one if necessary)
let file = file_manager.get_file_path();
```

2. Set the file format

```rust
// Do not need other code
set_file_format(FileFormat::CSV);
```

3. Time to Log
```rust
// CSV don't support sub-classes
let path = None;
let log_type = "ERROR";
let message = "Test";

log(file, None, log_type, message);
```

4. Enjoy

```csv
2023-02-15 22:55:33.779135600 +01:00;ERROR;Test;
```
