use anyhow::Result;

use std::fs;
use std::path::Path;

pub fn exe(
    path: &Path,
    start: usize,
    end: usize,
) -> Result<()> {
    if start == 0 || end < start {
        return Err(anyhow::anyhow!("invalid line range"));
    }

    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    if end > lines.len() {
        return Err(anyhow::anyhow!("range exceeds file length"));
    }

    let mut new_content = String::new();

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1; // 1-based

        if line_no < start || line_no > end {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    fs::write(path, new_content)?;

    Ok(())
}

pub fn writeup(path: &Path) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(path)?;
            println!("File deleted: {:?}", path);
        } else {
            println!("Not a file: {:?}", path);
        }
    } else {
        println!("File not found: {:?}", path);
    }

    Ok(())
}
