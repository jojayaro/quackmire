use crate::custom_table::Table;
use duckdb::arrow::{record_batch::RecordBatch, util::{display::{ArrayFormatter, FormatOptions}, pretty::pretty_format_batches}};

pub fn create_table(batch: RecordBatch) -> Table {

    let options = FormatOptions::default();

    let headers: Vec<String> = batch.schema().fields.iter().map(|f| f.name().clone()).collect();

    let rows: Vec<Vec<String>> = (0..batch.num_rows())
        .map(|row| {
            batch.columns().iter().map(|c| {
                let formatter = ArrayFormatter::try_new(c.as_ref(), &options).unwrap();
                formatter.value(row).to_string()
            }).collect()
        })
        .collect();

    Table::new(headers, rows)

}