use duckdb::arrow::{array::ArrayRef, datatypes::DataType, record_batch::RecordBatch};

use crate::custom_table::Table;

fn array_value_to_string(array: &ArrayRef, index: usize) -> String {
    if array.is_null(index) {
        "".to_string()
    } else {
        match array.data_type() {
            DataType::Boolean => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::BooleanArray>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Int8 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Int8Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Int16 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Int16Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Int32 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Int32Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Int64 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Int64Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::UInt8 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::UInt8Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::UInt16 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::UInt16Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::UInt32 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::UInt32Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::UInt64 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::UInt64Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Float32 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Float32Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Float64 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Float64Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Date32 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::Date32Array>()
                .unwrap()
                .value(index)
                .to_string(),
            DataType::Utf8 => array
                .as_any()
                .downcast_ref::<duckdb::arrow::array::StringArray>()
                .unwrap()
                .value(index)
                .to_string(),
            _ => format!("Unsupported type: {:?}", array.data_type()),
        }
    }
}

pub fn create_table(batch: &RecordBatch) -> Table<'static> {
    let headers: Vec<String> = batch
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().to_string())
        .collect();

    let rows: Vec<Vec<String>> = (0..batch.num_rows())
        .map(|i| {
            (0..batch.num_columns())
                .map(|j| array_value_to_string(batch.column(j), i))
                .collect()
        })
        .collect();

    let table = Table::new(headers, rows);

    table
}
