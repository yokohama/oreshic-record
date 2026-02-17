use crate::context::Context;

pub mod common;

pub mod command;
pub mod query;
pub mod writeup;

pub mod flag;

pub trait SearchExecutor {
    fn run(&self, ctx: &Context) -> anyhow::Result<()>;
}
