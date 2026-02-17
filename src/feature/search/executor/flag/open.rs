use std::env;
use std::path::PathBuf;

use anyhow::Result;

pub fn exe(path: &PathBuf) -> Result<()> {
    let editor = env::var("EDITOR")
        .map_err(|_| anyhow::anyhow!("$EDITOR is not set"))?;

    std::process::Command::new(editor)
        .arg(path)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("editor exited with error"))?;

    Ok(())
}
