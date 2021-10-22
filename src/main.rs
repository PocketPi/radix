//#![no_std]

use core::fmt;
use core::num::ParseIntError;
use std::error::Error;

use structopt::StructOpt;

#[derive(Debug)]
enum RadixError {
    InvalidInput(ParseIntError),
    NotImplementedOperator,
}

impl fmt::Display for RadixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RadixError::InvalidInput(i) => fmt::Display::fmt(&i, f),
            RadixError::NotImplementedOperator => f.write_str("Operator not implemented"),
        }
    }
}

impl Error for RadixError {}

fn parse_hex(src: &str) -> Result<isize, RadixError> {
    let mut radix = 10;
    let stripped = match src.strip_prefix("0x") {
        Some(v) => {
            radix = 16;
            v
        }
        None => &src,
    };

    let value: isize = isize::from_str_radix(stripped, radix)
        .or_else(|e| {
            if radix == 10 {
                println!("Failed parsing in radix 10. Trying radix 16!");
                radix = 16;
                isize::from_str_radix(stripped, radix)
            } else {
                Err(e)
            }
        })
        .map_err(RadixError::InvalidInput)?;

    Ok(value)
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "radix",
    about = "Tool that will convert input number of any radix to dec, hex and bin."
)]
struct Opt {
    #[structopt(parse(try_from_str = parse_hex))]
    input1: isize,

    operator: Option<String>,

    #[structopt(parse(try_from_str = parse_hex))]
    input2: Option<isize>,
}

fn main() -> Result<(), RadixError> {
    let opt = Opt::from_args();

    let mut result = opt.input1;

    if let Some(o) = opt.operator {
        if let Some(v) = opt.input2 {
            result = match o.as_str() {
                "+" => result.wrapping_add(v),
                "-" => result.wrapping_sub(v),
                "*" => result.wrapping_mul(v),
                "/" => result.wrapping_div(v),
                "%" => result % v,
                _ => return Err(RadixError::NotImplementedOperator),
            };
        };
    };

    println!("dec: {:}", result);
    println!("hex: 0x{:x}", result);
    println!("bin: 0b{:b}", result);

    Ok(())
}
