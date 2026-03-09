use std::env;
use std::path::PathBuf;

use anyhow::Result;

pub fn exe(path: &PathBuf) -> Result<()> {
    exe_at_line(path, None)
}

pub fn exe_at_line(path: &PathBuf, line: Option<usize>) -> Result<()> {
    let editor = env::var("EDITOR")
        .map_err(|_| anyhow::anyhow!("$EDITOR is not set"))?;

    let mut cmd = std::process::Command::new(editor);

    // vim/nvim/nano等は +行番号 で該当行にジャンプ
    if let Some(n) = line {
        cmd.arg(format!("+{}", n + 1)); // start_lineは0-basedなので+1
    }

    cmd.arg(path)
        .status()?
        .success()
        .then_some(())
        .ok_or_else(|| anyhow::anyhow!("editor exited with error"))?;

    Ok(())
}
