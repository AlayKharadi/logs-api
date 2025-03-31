use std::{
    collections::HashMap,
    fs::{self, File},
    sync::{Arc, Mutex},
};

use arrow::array::{Int64Array, RecordBatch, StringArray};
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use parquet::{arrow::ArrowWriter, file::properties::WriterProperties};
use serde_json::json;
use uuid::Uuid;

use crate::{models::log_entry::LogEntry, utils::partition};

#[axum::debug_handler]
pub async fn insert_logs(Json(logs): Json<Vec<LogEntry>>) -> impl IntoResponse {
    if logs.len() < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "Empty logs."
            })),
        ));
    }

    let buffer: HashMap<String, Vec<LogEntry>> = HashMap::new();
    let buffer_lock = Arc::new(Mutex::new(buffer));

    let Ok(mut buffer) = buffer_lock.lock() else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "message": "Unable to open file."
            })),
        ));
    };

    for log in logs {
        // Convert Unix Timestamp into UTC timestamp
        let Some(dt) = DateTime::<Utc>::from_timestamp(log.timestamp, 0) else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": "Invalid timestamp"
                })),
            ));
        };

        let partition = partition(&dt);

        buffer.entry(partition).or_default().push(log);
    }

    for (partition, mut logs) in buffer.drain() {
        logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        fs::create_dir_all(&partition).unwrap();

        let times: Vec<i64> = logs.iter().map(|log| log.timestamp).collect();
        let times = Int64Array::from(times);

        let messages: Vec<String> = logs.iter().map(|log| log.message.clone()).collect();
        let messages = StringArray::from(messages);

        let schema = Arc::new(arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("time", arrow::datatypes::DataType::Int64, false),
            arrow::datatypes::Field::new("log", arrow::datatypes::DataType::Utf8, false),
        ]));

        let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(times), Arc::new(messages)])
            .unwrap();

        let file_path = format!("{}/logs-{}.parquet", partition, Uuid::new_v4());
        let file = File::create(file_path).unwrap();
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props)).unwrap();

        writer.write(&batch).unwrap();
        writer.finish().unwrap();
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "done"
        })),
    ))
}
