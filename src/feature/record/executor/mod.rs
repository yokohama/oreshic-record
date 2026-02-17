use std::fs;
use std::io::ErrorKind;

use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;
use colored::*;

use crate::context::Context;

pub mod batch;
pub mod pty;

pub trait RecordExecutor {
    fn mode(&self) -> &'static str;
    fn run(&self) -> anyhow::Result<ExecResult>;
}

pub struct ExecResult {
    pub stdout: Vec<u8>,
}

pub fn get_track_name(ctx: &Context) -> Option<String> {
    match fs::read_to_string(&ctx.track_name_file_path) {
        Ok(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => None,
        Err(_) => None,
    }
}

pub fn print_track_banner(action: &str, name: String) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(120);

    let file_name = format!("{}.md", name);
    let label = format!("--- [ {}: {} ] ", action, file_name);

    let label_width = UnicodeWidthStr::width(label.as_str());

    if label_width >= term_width {
        println!("{}", label);
        return;
    }

    let dash_count = term_width - label_width;
    let dashes = "-".repeat(dash_count);

    println!();
    println!("{}{}", label.yellow(), dashes.yellow());
    println!();
}
