use std::fmt;

#[derive(Debug)]
struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot divide by zero")
    }
}

fn divide(a: i32, b: i32) -> Result<i32, DivideByZeroError> {
    if b == 0 {
        Err(DivideByZeroError)
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(20, 0);
    
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

