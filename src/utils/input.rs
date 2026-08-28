use std::io::{self, Write};

pub fn next_int(prompt: &str, error_msg: &str, should_loop: bool, min: i32, max: i32) -> i32 {
    let mut buffer = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_ok() {
            if let Ok(parsed_value) = buffer.trim().parse::<i32>() {
                if parsed_value >= min && parsed_value <= max {
                    return parsed_value; // * Success
                }
            }
        }

        println!("{}", error_msg);

        if !should_loop {
            break;
        }
    }

    min - 1
}

pub fn next_str(prompt: &str, error_msg: &str, should_loop: bool) -> String {
    let mut buffer = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_ok() {
            let trimmed = buffer.trim().to_string();
            if !trimmed.is_empty() {
                return trimmed;
            }
        }

        println!("{}", error_msg);

        if !should_loop {
            break;
        }
    }

    buffer
}
