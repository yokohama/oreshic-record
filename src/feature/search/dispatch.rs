use anyhow::Result;

use crate::{
    context::Context, 
    cli::{SearchArgs, SearchCommands},
    feature::search::executor::{
        command::Command,
        query::Query,
        writeup::Writeup,
        SearchExecutor,
    },
};

pub fn run(args: SearchArgs, ctx: &Context) -> Result<()> {
    match args.command {
        SearchCommands::Command { number, run, del, open, cat } => {
            run_with_executor(
                Command::new(number, run, del, open, cat, false),
                &ctx,
            )
        },
        SearchCommands::Track { number, run, del, open, cat } => {
            run_with_executor(
                Command::new(number, run, del, open, cat, true),
                &ctx,
            )
        },
        SearchCommands::Query { word, number, run, del, open, cat } => {
            run_with_executor(
                Query::new(word, number, run, del, open, cat),
                &ctx,
            )
        },
        SearchCommands::Writeup { number, open, cat } => {
            run_with_executor(
                Writeup::new(number, open, cat),
                &ctx,
            )
        },
    }
}

fn run_with_executor<T: SearchExecutor>(
    executor: T,
    ctx: &Context,
) -> Result<()> {
    executor.run(&ctx)
}
