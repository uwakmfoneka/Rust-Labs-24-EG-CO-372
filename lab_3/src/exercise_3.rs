use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),

    OutOfRange {
        value: i32,
        min: i32,
        max: i32,
    },

    EmptyInput,

    DivisibleByZero,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e) => {
                write!(f, "Parse error: {}", e)
            }

            AppError::OutOfRange { value, min, max } => {
                write!(f, "{} is not in [{}, {}]", value, min, max)
            }

            AppError::EmptyInput => {
                write!(f, "Input was empty")
            }

            AppError::DivisibleByZero => {
                write!(f, "Division by zero is not allowed")
            }
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

pub fn parse_and_validate(
    s: &str,
    min: i32,
    max: i32,
) -> Result<i32, AppError> {
    if s.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }

    let n: i32 = s.trim().parse()?;

    if n < min || n > max {
        return Err(AppError::OutOfRange {
            value: n,
            min,
            max,
        });
    }

    Ok(n)
}

pub fn safe_div(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        return Err(AppError::DivisibleByZero);
    }

    Ok(a / b)
}

pub fn main() {
    println!("========== Parse and Validate ==========");

    let test_cases = vec!["42", "101", "abc", "", "-5"];

    for case in test_cases {
        match parse_and_validate(case, 0, 100) {
            Ok(n) => println!("Valid: {}", n),
            Err(e) => println!("Error for {:?}: {}", case, e),
        }
    }

    println!("\n========== Safe Division ==========");

    let divisions = vec![(20, 5), (15, 3), (10, 0)];

    for (a, b) in divisions {
        match safe_div(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(e) => println!("Error dividing {} by {}: {}", a, b, e),
        }
    }
}