mod cli;
mod feature;
mod context;
mod ors_type;

use anyhow::Result;

use clap::Parser;
use cli::{Cli, SubCommands};
use context::Context;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let ctx = Context::new()?; 

    match cli.command {
        SubCommands::Record(sub_args) => feature::record::dispatch::run(sub_args, &ctx),
        SubCommands::Set(sub_args) => feature::set::run(sub_args, &ctx),
        SubCommands::Unset => feature::unset::run(&ctx),
        SubCommands::Search(sub_args) => feature::search::dispatch::run(sub_args, &ctx),
    }
}
