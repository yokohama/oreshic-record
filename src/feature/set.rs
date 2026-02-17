use std::fs;
use anyhow::Result;

use crate::{
    cli::SetArgs,
    context::Context,
    feature::record::executor,
};

pub fn run(args: SetArgs, ctx: &Context) -> Result<()> {
    executor::print_track_banner("START", args.track_name.clone());
    fs::write(&ctx.track_name_file_path, &args.track_name)?;

    Ok(())
}
