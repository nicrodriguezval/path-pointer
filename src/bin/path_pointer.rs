use anyhow::Result;
use clap::Parser;

use path_pointer::opts::Opts;

fn main() -> Result<()> {
    let opts = Opts::parse();
    print!("{:?}", opts);

    Ok(())
}
