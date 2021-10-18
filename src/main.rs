

//#![no_std]

use std::error::Error;
use core::fmt;
use core::num::ParseIntError;

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "radix",
    about = "Tool that will convert input number of any radix to dec, hex and bin."
)]
struct Opt {
    input: String,

    #[structopt(short, long, default_value = "32")]
    width: usize,
}

#[derive(Debug)]
enum RadixError {
    InvalidWidth,
    InvalidInput(ParseIntError),
}

impl fmt::Display for RadixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RadixError::InvalidWidth => f.write_str("Invalid width"),
            RadixError::InvalidInput(i) => fmt::Display::fmt(&i, f),
        }
    }
}

impl Error for RadixError {}

fn main() -> Result<(), RadixError> {
    let mut radix: u32 = 10;
    let opt = Opt::from_args();

    if (opt.width % 2) != 0 {
        return Err(RadixError::InvalidWidth);
    }

    let input = opt.input;
    let stripped = match input.strip_prefix("0x") {
        Some(v) => {
            radix = 16;
            v
        }
        None => &input,
    };

    let v = i64::from_str_radix(stripped, radix)
        .or_else(|e| {
            if radix == 10 {
                println!("Failed parsing in radix 10. Trying radix 16!");
                i64::from_str_radix(stripped, 16)
            } else {
                Err(e)
            }
        })
        .map_err(|e| RadixError::InvalidInput(e))?;

    println!("dec: {:}", v);
    println!("hex: 0x{:0>width$x}", v, width = opt.width / 8);
    println!("bin: 0b{:0>width$b}", v, width = opt.width / 2);

    Ok(())
}
