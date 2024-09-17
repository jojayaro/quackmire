use crate::custom_table::Table;
use duckdb::arrow::{record_batch::RecordBatch, util::pretty::pretty_format_batches};

pub fn create_table(batch: &RecordBatch) -> Table {
    let formatted = pretty_format_batches(&[batch.clone()]).unwrap().to_string();
    let lines: Vec<&str> = formatted.lines().collect();

    if lines.len() < 5 {
        return Table::new(vec![], vec![]);
    }

    let headers: Vec<String> = lines[1]
        .split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let rows: Vec<Vec<String>> = lines
        .iter()
        .skip(3)
        .filter(|line| !line.starts_with('+') && !line.trim().is_empty()) // Skip border lines
        .map(|line| {
            line.split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();

    Table::new(headers, rows)
}
