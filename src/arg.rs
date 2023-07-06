use clap::{Parser};
#[path = "./permission.rs"]
mod permission;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Permission with rwx format
    #[arg(short, long)]
    dac: Option<String>,

    #[arg(short, long)]
    octal: Option<u8>,
}


pub fn parse_args() {
    let args = Args::parse();

     let from_str: Option<String> = args.dac;

     match from_str {
        Some(value) => {
            permission::dac_to_octal(value);
            return
        },
        None => return
     }
}
