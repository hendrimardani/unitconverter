mod converter;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use converter::HistoryList;

#[derive(Parser)]
#[command(
    name = "unitconv",
    version = "1.0.0",
    about = "Aplikasi converter CLI sederhana"
)]

pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]

pub enum Commands {
    Convert(UnitArgs),
    List,
    History,
}

#[derive(Args)]
pub struct UnitArgs {
    #[arg(short, long)]
    from: String,
    #[arg(short, long)]
    to: String,
    #[arg(short, long)]
    value: f32,
}

pub fn run(cli: Cli) -> Result<()> {
    let mut histories = HistoryList::load().unwrap_or_default();

    match cli.commands {
        Commands::Convert(args) => {
            let result = histories.calculate(args.from, args.to, args.value)?;
            histories.add(result);
            histories.save().context("Gagal menyimpan data history")?;
        }
        Commands::List => {
            histories.list();
        }
        Commands::History => {
            histories.print();
        }
    }
    Ok(())
}
