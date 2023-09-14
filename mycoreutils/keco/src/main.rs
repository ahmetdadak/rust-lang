mod args;
use args::KecoArgs;
use clap::Parser;
use std::io;
use std::io::Write;

fn main() {
    let args = KecoArgs::parse();
    let color = args.color;
    let text = args.text;
    let stdout = io::stdout();
    if color.len() != 6 {
        eprintln!("Please provide a hex code with valid length.");
        std::process::exit(1);
    }
    let r = match u8::from_str_radix(&color[0..2], 16) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Please provide a valid hex code: {}.", err);
            std::process::exit(1);
        }
    };
    let g = match u8::from_str_radix(&color[2..4], 16) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Please provide a valid hex code: {}.", err);
            std::process::exit(1);
        }
    };
    let b = match u8::from_str_radix(&color[4..6], 16) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Please provide a valid hex code: {}.", err);
            std::process::exit(1);
        }
    };
    let mut handle = stdout.lock();
    if let Err(err) = writeln!(
        handle,
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        r,
        g,
        b,
        text.join(" ")
    ) {
        eprintln!("Write error: {}", err);
        std::process::exit(1);
    }
}
