use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    Liner(Liner),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct Liner {}

fn main() {
    dbg!(Cargo::parse());
}
