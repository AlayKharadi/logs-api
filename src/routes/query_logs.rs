use std::{
    fs::{self, File},
    path::Path,
};

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Duration, Utc};
use parquet::{
    file::reader::{FileReader, SerializedFileReader},
    record::RowAccessor,
};
use serde::Deserialize;
use serde_json::json;
use tracing::warn;

use crate::{models::log_entry::LogEntry, utils::partition};

type LogType = i64;

#[derive(Debug, Deserialize)]
pub struct RequestParams {
    start: String,
    end: String,
    text: String,
}

#[axum::debug_handler]
pub async fn query_logs(Query(params): Query<RequestParams>) -> impl IntoResponse {
    let Ok(start) = params.start.parse::<LogType>() else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "`start` should be a valid timestamp."
            })),
        ));
    };

    let Ok(end) = params.end.parse::<LogType>() else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "`end` should be a valid timestamp."
            })),
        ));
    };

    println!("{} - {} = {}", start, end, start-end);

    if start > end {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "`start` should be less than `end` timestamp."
            })),
        ));
    }

    let Some(start_dt) = DateTime::<Utc>::from_timestamp(start, 0) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "`start` should be a valid timestamp."
            })),
        ));
    };

    let Some(end_dt) = DateTime::<Utc>::from_timestamp(end, 0) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "`end` should be a valid timestamp."
            })),
        ));
    };

    let mut logs: Vec<LogEntry> = Vec::new();

    let mut current = start_dt;

    while current <= end_dt {
        let partition = partition(&current);

        if Path::new(&partition).exists() == false {
            warn!("Couldn't find a record for {}.", partition);
            continue;
        }

        let Ok(log_entries) = fs::read_dir(&partition) else {
            warn!("Couldn't read directory for {}.", partition);
            continue;
        };

        for log_entry in log_entries {
            let Ok(entry) = log_entry else {
                continue;
            };

            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("parquet") {
                warn!("File at {:#?} is not a parquet file.", path);
                continue;
            }

            let Ok(file) = File::open(&path) else {
                continue;
            };

            let Ok(reader) = SerializedFileReader::new(file) else {
                continue;
            };

            let Ok(row_group_header) = reader.get_row_group(0) else {
                continue;
            };

            let Ok(mut row_iter) = row_group_header.get_row_iter(None) else {
                continue;
            };

            while let Some(row_item) = row_iter.next() {
                let Ok(row) = row_item else {
                    continue;
                };

                let Ok(time) = row.get_long(0) else {
                    continue;
                };

                let Ok(log) = row.get_string(1) else {
                    continue;
                };

                if (time >= start && time <= end) == false {
                    continue;
                }

                if log.contains(&params.text) == false {
                    continue;
                }

                logs.push(LogEntry {
                    timestamp: time,
                    message: log.to_string(),
                });
            }
        }

        current = current + Duration::hours(1);
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "logs": logs
        })),
    ))
}
