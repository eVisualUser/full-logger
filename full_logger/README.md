# Full Logger
Easy to use logger for Rust

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
let path = vec!["error", "debug"];
let message = "Test";

log(file, path, message);
```

4. Enjoy

```csv
2023-02-21 19:00:17.771215200 +01:00;error;debug;Test;
```
