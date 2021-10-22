//#![no_std]

use core::fmt;
use core::num::ParseIntError;
use std::error::Error;

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "radix",
    about = "Tool that will convert input number of any radix to dec, hex and bin."
)]
struct Opt {
    input1: String,

    operator: Option<String>,

    input2: Option<String>,

    #[structopt(short, long, default_value = "32")]
    width: usize,
}

#[derive(Debug)]
enum RadixError {
    InvalidWidth,
    InvalidInput(ParseIntError),
    NotImplementedOperator,
}

impl fmt::Display for RadixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RadixError::InvalidWidth => f.write_str("Invalid width"),
            RadixError::InvalidInput(i) => fmt::Display::fmt(&i, f),
            RadixError::NotImplementedOperator => f.write_str("Operator not implemented"),
        }
    }
}

impl Error for RadixError {}

fn str_to_int(input: String) -> Result<i64, RadixError> {
    let mut radix = 10;
    let stripped = match input.strip_prefix("0x") {
        Some(v) => {
            radix = 16;
            v
        }
        None => &input,
    };

    let value: i64 = i64::from_str_radix(stripped, radix)
        .or_else(|e| {
            if radix == 10 {
                println!("Failed parsing in radix 10. Trying radix 16!");
                radix = 16;
                i64::from_str_radix(stripped, radix)
            } else {
                Err(e)
            }
        })
        .map_err(RadixError::InvalidInput)?;

    Ok(value)
}

fn main() -> Result<(), RadixError> {
    let opt = Opt::from_args();

    if (opt.width % 2) != 0 {
        return Err(RadixError::InvalidWidth);
    }

    let value1 = str_to_int(opt.input1)?;

    let value2 = match opt.input2 {
        Some(v) => Some(str_to_int(v)?),
        None => None,
    };

    let mut result = value1;

    if let Some(o) = opt.operator {
        if let Some(v) = value2 {
            result = match o.as_str() {
                "+" => value1.wrapping_add(v),
                "-" => value1.wrapping_sub(v),
                "*" => value1.wrapping_mul(v),
                "/" => value1.wrapping_div(v),
                "%" => value1 % v,
                _ => return Err(RadixError::NotImplementedOperator),
            };
        };
    };

    println!("dec: {:}", result);
    println!("hex: 0x{:x}", result);
    println!("bin: 0b{:b}", result);

    Ok(())
}
