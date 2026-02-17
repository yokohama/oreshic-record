use anyhow::Result;

use crate::{
    feature::{
        record::executor::{
            batch::Batch,
            pty::Pty,
            RecordExecutor,
        },
        search::executor::common::Record,
    },
};

pub fn exe(section: &Record) -> Result<()> {
    let cmd_str = section.command
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no command in section"))?;

    let parts: Vec<&str> = cmd_str.split_whitespace().collect();

    let (command, args) = parts
        .split_first()
        .ok_or_else(|| anyhow::anyhow!("empty command"))?;
    let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();

    let mode = section.mode
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!(
            "mode is not specified in code block. Use: ```bash batch or ```bash pty"
        ))?;
    
    let executor: Box<dyn RecordExecutor> = match mode {
        "batch" => Box::new(Batch::new(command, args_vec)),
        "pty" => Box::new(Pty::new(command, args_vec)),
        other => {
            return Err(anyhow::anyhow!(
                "invalid mode '{}'. Supported: batch | pty",
                other
            ));
        }
    };
    
    executor.run()?;
    
    Ok(())
}
