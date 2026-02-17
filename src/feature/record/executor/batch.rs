use std::process::{Command, Stdio};
use std::io::{Read, Write};
use std::thread;
use anyhow::Result;

use std::os::unix::process::CommandExt;

use super::{RecordExecutor, ExecResult};

#[derive(Debug)]
pub struct Batch {
    pub command: String,
    pub args: Vec<String>,
}

impl Batch {
    pub fn new(command: &str, args: Vec<String>) -> Self {
        Self {
            command: command.to_string(),
            args,
        }
    }
}

impl RecordExecutor for Batch {
    fn mode(&self) -> &'static str {
        "batch"
    }

    fn run(&self) -> Result<ExecResult> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args).stdout(Stdio::piped());

        unsafe {
            cmd.pre_exec(|| {
                nix::unistd::setsid()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(())
            });
        }

        let mut child = cmd.spawn()?;

        let mut stdout = child.stdout.take().unwrap();

        let out_handle = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            let mut collected = Vec::new();
            let mut out = std::io::stdout();

            while let Ok(n) = stdout.read(&mut buf) {
                if n == 0 { break; }
                out.write_all(&buf[..n]).ok();
                collected.extend_from_slice(&buf[..n]);
            }
            collected
        });

        let _status = child.wait()?;

        Ok(ExecResult {
            stdout: out_handle.join().unwrap(),
        })
    }
}
