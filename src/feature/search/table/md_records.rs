use crate::feature::search::{
    executor::common::Record,
    table::print_table,
};

pub fn print_md_table(records: &[Record]) {
    let headers = ["Index", "Type", "Command", "Count"];

    let rows: Vec<Vec<String>> = records
        .iter()
        .map(|r| {
            vec![
                r.index.to_string(),
                format!("{} / {}", r.record_type, r.unit_type),
                r.title.clone().unwrap_or_default(),
                r.count.map(|c| c.to_string()).unwrap_or_default(),
            ]
        })
        .collect();

    print_table(&headers, &rows, &[1, 1, 6, 1]);
}
