use std::{
    fs,
    env,
    path::{Path, PathBuf},
    iter::Peekable,
    process::Command,
    io::Write,
};

use tempfile::Builder;
use anyhow::Result;
use minijinja::{Environment, context};

pub const TEMPLATE_PATH: &str = include_str!("../../../templates/record.md");

use crate::ors_type::{UnitType, RecordType};

#[derive(Debug, Clone)]
pub struct Record {
    pub index: usize,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub path: PathBuf,
    pub unit_type: UnitType,
    pub record_type: RecordType,
    pub count: Option<usize>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub command: Option<String>,
    pub mode: Option<String>,
    pub result: Option<String>,
    pub tags: Vec<String>,
}

pub fn count_section(path: &Path) -> usize {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    content
        .lines()
        .filter(|line| line.trim_start().starts_with("# "))
        .count()
}

pub fn collect_writeup(record: &Record) -> Result<Record> {
    let mut writeup_record = record.clone();

    let stem = record.path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let content = fs::read_to_string(&record.path)?;
    let mut lines = content.lines().enumerate().peekable();

    let mut tags = Vec::new();

    while let Some((_, next_line)) = lines.peek() {
        let _ = next_line.trim();

        let (_, actual_line) = lines.next().unwrap();
        let trimmed = actual_line.trim();

        if trimmed == "## Tag" {
            if let Some(tag_line) = read_plain_block(&mut lines) {
                if tag_line != "{none}" && !tag_line.is_empty() {
                    tags = tag_line
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
            }
        }
    }

    writeup_record.title = Some(stem.to_string());
    writeup_record.command = Some("/* file */".to_string());
    writeup_record.tags = tags;

    Ok(writeup_record)
}

pub fn collect_sections(record: &Record) -> Result<Vec<Record>> {
    let content = fs::read_to_string(record.path.clone())?;
    let mut sections = Vec::new();

    let total_lines = content.lines().count();
    
    let mut lines = content.lines().enumerate().peekable();
    let mut index = 0;

    while let Some((line_no, line)) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") {
            let start_line = line_no; // 0-based
         
            let title = trimmed.trim_start_matches("# ").trim().to_string();

            let mut message = None;
            let mut command = None;
            let mut mode = None;
            let mut result_block = None;
            let mut tags = Vec::new();

            let mut end_line = start_line;
            let mut found_next_section = false;

            while let Some((next_no, next_line)) = lines.peek() {
                let t = next_line.trim();

                // 次のセクション開始
                if t.starts_with("# ") {
                    end_line = *next_no;
                    found_next_section = true;
                    break;
                }

                // 消費
                let (_, actual_line) = lines.next().unwrap();
                let trimmed = actual_line.trim();

                if trimmed == "## Message" {
                    message = read_plain_block(&mut lines);
                }
                else if trimmed == "## Command" {
                    (mode, command) = read_code_block(&mut lines);
                }
                else if trimmed == "## Result" {
                    (_, result_block) = read_code_block(&mut lines);
                }
                else if trimmed == "## Tag" {
                    if let Some(tag_line) = read_plain_block(&mut lines) {
                        if tag_line != "{none}" && !tag_line.is_empty() {
                            tags = tag_line
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        }
                    }
                }
            }

            if !found_next_section {
                end_line = total_lines;
            }

            sections.push(Record {
                index,
                start_line: Some(start_line),
                end_line: Some(end_line),
                path: record.path.clone(),
                unit_type: UnitType::Section,
                record_type: record.record_type,
                count: None,
                title: Some(title),
                message,
                command,
                mode,
                result: result_block,
                tags,
            });

            index += 1;
        }
    }

    Ok(sections)
}

pub fn print_section(section: &Record) -> Result<()> {
    let mut env_jinja = Environment::new();
    env_jinja.add_template("section", TEMPLATE_PATH)?;

    let tmpl = env_jinja.get_template("section")?;

    let content = tmpl.render(context! {
       title => section.title.clone().unwrap_or("{none}".to_string()),
       message => section.message.clone().unwrap_or("{none}".to_string()),
       command => section.command,
       mode => section.mode,
       result => section.result.clone().unwrap_or("{none}".to_string()),
       tag => section.tags.join(","),
    })?;

    if let Ok(viewer) = env::var("MD_VIEWER") {
        let mut tmp = Builder::new().suffix(".md").tempfile()?;
        tmp.write_all(content.as_bytes())?;

        let parts: Vec<&str> = viewer.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        let status = Command::new(cmd)
            .args(args)
            .arg(tmp.path())
            .status()?;

        if !status.success() {
            anyhow::bail!("viewer exited with error");
        }

        return Ok(());
    }

    println!("{}", content);

    Ok(())
}

pub fn print_section_raw(section: &Record) -> Result<()> {
    let mut env_jinja = Environment::new();
    env_jinja.add_template("section", TEMPLATE_PATH)?;

    let tmpl = env_jinja.get_template("section")?;

    let content = tmpl.render(context! {
       title => section.title.clone().unwrap_or("{none}".to_string()),
       message => section.message.clone().unwrap_or("{none}".to_string()),
       command => section.command,
       mode => section.mode,
       result => section.result.clone().unwrap_or("{none}".to_string()),
       tag => section.tags.join(","),
    })?;

    println!("{}", content);

    Ok(())
}

pub fn print_md(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let converted = convert_image_paths(&content);

    if let Ok(viewer) = env::var("MD_VIEWER") {
        let mut temp = Builder::new().suffix(".md").tempfile()?;
        temp.write_all(converted.as_bytes())?;

        let parts: Vec<&str> = viewer.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        let status = Command::new(cmd)
            .args(args)
            .arg(temp.path())
            .status()?;

        if !status.success() {
            anyhow::bail!("viewer exited with error");
        }

        return Ok(());
    }

    println!("{}", converted);

    Ok(())
}

pub fn print_md_raw(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;
    println!("{}", content);
    Ok(())
}

fn convert_image_paths(content: &str) -> String {
    let base_dir = match env::var("ORS_RECORDS_DIR") {
        Ok(dir) => dir,
        Err(_) => return content.to_string(),
    };

    let abs_images_dir = Path::new(&base_dir).join("images");
    let abs_images_str = abs_images_dir.to_string_lossy();

    content.replace("](./images/", &format!("]({}/", abs_images_str))
}

fn read_plain_block<'a, I>(
    lines: &mut Peekable<I>
) -> Option<String>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut buf = String::new();

    while let Some((_, line)) = lines.peek() {
        let trimmed = line.trim();

        if trimmed.starts_with("## ") || trimmed.starts_with("# ") {
            break;
        }

        buf.push_str(trimmed);
        buf.push('\n');
        lines.next();
    }

    let v = buf.trim();

    if v.is_empty() || v == "{none}" {
        None
    } else {
        Some(v.to_string())
    }
}

fn read_code_block<'a, I>(
    lines: &mut Peekable<I>,
) -> (Option<String>, Option<String>)
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut buf = String::new();
    let mut mode: Option<String> = None;

    // ===== 開始フェンス =====
    while let Some((_, line)) = lines.next() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            let header = trimmed.trim_start_matches("```").trim();
            let parts: Vec<&str> = header.split_whitespace().collect();

            if parts.len() >= 2 {
                mode = Some(parts[1].to_string());
            }
            break;
        }
    }

    // ===== 本文 =====
    while let Some((_, line)) = lines.next() {
        if line.trim().starts_with("```") {
            break;
        }

        buf.push_str(line);
        buf.push('\n');
    }

    let body = buf.trim();

    let body_opt = if body.is_empty() {
        None
    } else {
        Some(body.to_string())
    };

    (mode, body_opt)
}

pub fn collect_records(base_dir: &Path) -> Result<Vec<Record>> {
    let mut records = Vec::new();
    let mut index = 0;

    fn walk(
        dir: &Path,
        index: &mut usize,
        records: &mut Vec<Record>,
    ) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk(&path, index, records)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {

                let record_type = infer_record_type(&path);

                let md_file_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();

                let mut record = Record {
                    index: *index,
                    start_line: None,
                    end_line: None,
                    path: path.clone(),
                    unit_type: UnitType::MdFile,
                    record_type,
                    count: None,
                    title: Some(md_file_name.to_string()),
                    message: None,
                    command: None,
                    mode: None,
                    result: None,
                    tags: vec![],
                };

                record.count = Some(count_section(&path));

                records.push(record);

                *index += 1;
            }
        }

        Ok(())
    }

    walk(base_dir, &mut index, &mut records)?;

    // fs::read_dir の順序は保証されないためソート
    records.sort_by(|a, b| a.path.cmp(&b.path));

    // ソート後にインデックスを振り直す
    for (i, record) in records.iter_mut().enumerate() {
        record.index = i;
    }

    Ok(records)
}

fn infer_record_type(path: &Path) -> RecordType {
    if path.components().any(|c| c.as_os_str() == "commands") {
        RecordType::Command
    } else if path.components().any(|c| c.as_os_str() == "tracks") {
        RecordType::Track
    } else {
        RecordType::Writeup
    }
}
