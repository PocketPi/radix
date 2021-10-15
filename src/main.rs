use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(name = "radix", about = "Tool that will convert input number of any radix to dec, hex and bin.")]
struct Opt {
    input: Option<String>,
}

fn main() {
    let mut radix:u32 = 10;
    let input;
    let opt = Opt::from_args();
    match opt.input {
        Some(v) => input = v,
        None => panic!("Not a valid input"),
    };
    let stripped;
    match input.strip_prefix("0x") {
        Some(v) => {
            stripped = v;
            radix = 16;
        }
        None => stripped = &input,
    }

    match i64::from_str_radix(stripped, radix) {
        Ok(v) => {
            println!("dec: {:}", v);
            println!("hex: {:#x}", v);
            println!("bin: {:#b}", v);
        },
        Err(e) => panic!("{}", e),
    }


}
