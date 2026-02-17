use crate::feature::search::{
    executor::common::Record,
    table::print_table,
};

pub fn print_section_table(records: &[Record]) {
    let headers = ["Index", "Type", "Title", "Command"];

    let rows: Vec<Vec<String>> = records
        .iter()
        .map(|r| {
            vec![
                r.index.to_string(),
                format!("{} / {}", r.record_type, r.unit_type),
                r.title.clone().unwrap_or_default(),
                r.command.clone().unwrap_or_default(),
            ]
        })
        .collect();

    print_table(&headers, &rows, &[1, 1, 4, 4]);
}
