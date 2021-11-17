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
        None => src,
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
    about = "Tool that will convert input number of any radix to dec, hex and bin.",
    author = "Peter Rasmussen",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp,
                        structopt::clap::AppSettings::AllowNegativeNumbers]
)]
struct Opt {

    /// Required argument, input value in dec or hex format
    #[structopt(parse(try_from_str = parse_hex))]
    input1: Option<isize>,

    /// Required if two input values are given
    #[structopt(requires("input2"), possible_values(&["+", "-", "*", "/", "%"]))]
    operator: Option<String>,

    /// If <operator> and <input2> values are given simple math will be happen
    #[structopt(parse(try_from_str = parse_hex), requires("operator"))]
    input2: Option<isize>,

    /// Width determines how many bits is used to show the result value.
    #[structopt(short, long, requires("input1"), possible_values(&["2", "4", "8", "16", "32", "64"]))]
    width: Option<usize>,
}

fn main() -> Result<(), RadixError> {
    let opt = Opt::from_args();

    if let Some(mut r) = opt.input1 {
        if let Some(o) = opt.operator {
            if let Some(v) = opt.input2 {
                r = match o.as_str() {
                    "+" => r.wrapping_add(v),
                    "-" => r.wrapping_sub(v),
                    "*" => r.wrapping_mul(v),
                    "/" => r.wrapping_div(v),
                    "%" => r % v,
                    _ => return Err(RadixError::NotImplementedOperator),
                };
            };
        };

        let mut width: usize = 2;
        if r / 2_isize.pow(4) >= 1 {
            width = 8;
            if r / 2_isize.pow(width as u32) >= 1 {
                width = 16;
                if (r / 2_isize.pow(width as u32)) >= 1 {
                    width = 32;
                    if (r / 2_isize.pow(width as u32)) >= 1 {
                        width = 64;
                    }
                }
            }
        }

        if let Some(w) = opt.width {
            if width > w {
                println!("Width not wide enough to reprecent result, setting width to: {:}", width);
            } else {
                width = w;
            }
        }

        println!("dec: {:}", r);
        println!("hex: 0x{:01$x}", r, width / 4);
        println!("bin: 0b{:01$b}", r, width);
    };

    Ok(())
}
