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
        SearchCommands::Command { number, run, del, open } => {
            run_with_executor(
                Command::new(number, run, del, open, false),
                &ctx,
            )
        },
        SearchCommands::Track { number, run, del, open } => {
            run_with_executor(
                Command::new(number, run, del, open, true),
                &ctx,
            )
        },
        SearchCommands::Query { word, number, run, del, open } => {
            run_with_executor(
                Query::new(word, number, run, del, open),
                &ctx,
            )
        },
        SearchCommands::Writeup { number, open } => {
            run_with_executor(
                Writeup::new(number, open),
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
