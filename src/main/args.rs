use std::num::ParseIntError;
use structopt::StructOpt;

fn parse_hash(src: &str) -> Result<u32, ParseIntError> {
    if src.starts_with("0x") {
        u32::from_str_radix(&src[2..], 16)
    } else {
        u32::from_str_radix(src, 10)
    }
}

#[derive(Debug, StructOpt)]
pub(crate) struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub(crate) enum Command {
    Calc(Calc),
    Reverse(Reverse),
}

#[derive(Debug, StructOpt)]
pub(crate) struct Calc {
    pub word: String,
}

#[derive(Debug, StructOpt)]
pub(crate) struct Reverse {
    #[structopt(parse(try_from_str = parse_hash))]
    pub hex_value: u32,

    #[structopt(default_value = "6")]
    #[structopt(short = "m")]
    pub max_length: usize,

    #[structopt(default_value = "abcdefghijklmnopqrstuvwxyz")]
    #[structopt(short = "a")]
    pub alphabet: String,

    #[structopt(short = "c")]
    #[structopt(long = "capital")]
    pub capitalize: bool,
}
