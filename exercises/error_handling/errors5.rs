// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), CreationError> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
    Parse(ParseIntError),
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),

            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("debug hook");
        let description = match self {
            CreationError::Negative => String::from("number is negative"),
            CreationError::Zero => String::from("number is zero"),
            CreationError::Parse(e) =>
            /*e.to_string() +*/
            {
                String::from("   mycustomerr")
            }
        };
        f.write_str(description.as_ref())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        println!("{:?}", self);
        ""
    }
}

impl From<ParseIntError> for CreationError {
    fn from(err: ParseIntError) -> CreationError {
        CreationError::Parse(err)
    }
}
