#[derive(Debug)]
enum ConversionError {
    NotInRange,
}

/// Convert a character from 0 to f into its numerical representation.
fn char_to_num(input: char) -> Result<u8, ConversionError> {
    match input {
        '0'...'9' => Ok((input as u8) - 48),
        'a'...'f' => Ok((input as u8) - 87),
        _ => Err(ConversionError::NotInRange),
    }
}

// find the smallest base the could possibly represent the value
fn smallest_base(string: &str) -> Result<u8, ConversionError> {
    match string.to_lowercase().chars().max() {
        Some('0') => Ok(1),
        Some(c) => char_to_num(c).map(|n| n + 1),
        None => Ok(1),
    }
}

// convert a given string to the numer in the given base but also handle base 1
fn from_str_radix(input: &str, radix: u32) -> u32 {
    match radix {
        1 => 0,
        n => u32::from_str_radix(input, n).unwrap(),
    }
}


fn main() {
    let inputs: Vec<String> = std::env::args().skip(1).collect();
    for input in inputs {
        println!("> For {}: ", input);
        match smallest_base(&input) {
            Ok(smallest_base) => {
                for base in smallest_base..17 {
                    println!("  base {} => {}", base, from_str_radix(&input, base as u32));
                }
            }
            Err(ConversionError::NotInRange) => {
                println!("  Input {} not in range", input);
            }
        }
    }
}
