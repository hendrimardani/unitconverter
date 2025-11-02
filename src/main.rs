use anyhow::Result;
use clap::Parser;
use unitconv::{Cli, run};

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
