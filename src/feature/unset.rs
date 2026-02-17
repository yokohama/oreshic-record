use std::fs;

use anyhow::{Result, Context};

use crate::{
    context::Context as OrsContext,
    feature::record::executor,
};

pub fn run(ctx: &OrsContext) -> Result<()> {

    let track_name = fs::read_to_string(&ctx.track_name_file_path)
        .with_context(|| format!(
            "failed to read track file: {}",
            ctx.track_name_file_path.display()
        ))?;

    fs::remove_file(&ctx.track_name_file_path)
        .with_context(|| format!(
            "failed to remove track file: {}",
            ctx.track_name_file_path.display()
        ))?;

    executor::print_track_banner("FINISHED", track_name);

    Ok(())
}

