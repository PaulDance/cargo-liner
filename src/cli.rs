use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cargo {
    Liner(Liner),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Liner {}

pub fn parse_args() -> Liner {
    match Cargo::parse() {
        Cargo::Liner(args) => args,
    }
}
