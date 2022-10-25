use clap::Parser;
use tikey::client::{client, TiKeyArgs};

fn main() {
    let args = TiKeyArgs::parse();
    client(args).expect("TiKey running failed");
}
