use std::num::ParseIntError;

use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "radix",
    about = "Tool that will convert input number of any radix to dec, hex and bin."
)]
struct Opt {
    input: String,

    #[structopt(short, long, default_value="32")]
    width: i64,
}

fn main() -> Result<(), ParseIntError> {
    let mut radix: u32 = 10;
    let opt = Opt::from_args();

    let input = opt.input;
    let stripped = match input.strip_prefix("0x") {
        Some(v) => {
            radix = 16;
            v
        }
        None => &input,
    };

    let v = i64::from_str_radix(stripped, radix).or_else(|e| {
        if radix == 10 {
            println!("Failed parsing in radix 10. Trying radix 16!");
            i64::from_str_radix(stripped, 16)
        } else {
            Err(e)
        }
    })?;

    println!("dec: {:}", v);
    println!("hex: 0x{:x}", v);
    println!("bin: {:#018b}", v);

    Ok(())
}
