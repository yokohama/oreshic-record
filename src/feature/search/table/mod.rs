pub mod md_records;
pub mod section_records;

use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;
use colored::*;

pub fn print_table(headers: &[&str], rows: &[Vec<String>], ratios: &[usize]) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(120);

    let col_count = headers.len();
    let sep_width = 3 * (col_count - 1);

    let total_ratio: usize = ratios.iter().sum();
    let usable_width = term_width - sep_width;

    let mut col_widths = Vec::new();
    for r in ratios {
        col_widths.push(usable_width * r / total_ratio);
    }

    println!("");
    print_header(headers, &col_widths);
    println!("{}", "-".repeat(term_width).bright_black());

    for row in rows {
        print_row_colored(row, &col_widths);
    }
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());

        if current_width + ch_width > width {
            lines.push(current.clone());
            current.clear();
            current_width = 0;
        }

        current.push(ch);
        current_width += ch_width;
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn pad_display_width(s: &str, width: usize) -> String {
    let display_width = UnicodeWidthStr::width(s);

    if display_width >= width {
        return s.to_string();
    }

    let padding = width - display_width;
    format!("{}{}", s, " ".repeat(padding))
}

fn print_row_colored(cols: &[String], widths: &[usize]) {
    let wrapped: Vec<Vec<String>> = cols
        .iter()
        .zip(widths)
        .map(|(c, w)| wrap(c, *w))
        .collect();

    let max_lines = wrapped.iter().map(|v| v.len()).max().unwrap_or(1);

    for i in 0..max_lines {
        for (col_i, col_lines) in wrapped.iter().enumerate() {
            let raw = col_lines.get(i).map(|s| s.as_str()).unwrap_or("");

            let padded = pad_display_width(raw, widths[col_i]);

            let colored = match col_i {
                0 => padded.green().to_string(),
                1 => padded.bright_black().to_string(),
                _ => padded.white().to_string(),
            };

            print!("{}", colored);

            if col_i != wrapped.len() - 1 {
                print!("{}", " | ".bright_black());
            }
        }
        println!();
    }
}

fn print_header(cols: &[&str], widths: &[usize]) {
    let wrapped: Vec<Vec<String>> = cols
        .iter()
        .zip(widths)
        .map(|(c, w)| wrap(c, *w))
        .collect();

    let max_lines = wrapped.iter().map(|v| v.len()).max().unwrap_or(1);

    for i in 0..max_lines {
        for (col_i, col_lines) in wrapped.iter().enumerate() {
            let raw = col_lines.get(i).map(|s| s.as_str()).unwrap_or("");

            let padded = pad_display_width(raw, widths[col_i]);

            let colored = padded
                .blue()
                .bold()
                .to_string();

            print!("{}", colored);

            if col_i != wrapped.len() - 1 {
                print!("{}", " | ".bright_black());
            }
        }
        println!();
    }
}

