use crate::databento::{parse_action, parse_side};
use crate::model::TickData;
use anyhow::{Result, anyhow};
use csv::Reader;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct DatabentoMboCsvRow {
    pub ts_recv: u64,
    pub ts_event: u64,
    #[allow(dead_code)]
    pub rtype: u8,
    pub publisher_id: u16,
    pub instrument_id: u32,
    pub action: String,
    pub side: String,
    pub price: i64,
    pub size: u32,
    pub channel_id: u8,
    pub order_id: u64,
    pub flags: u8,
    pub ts_in_delta: i32,
    pub sequence: u32,
}

pub trait FromDatabentoRow {
    fn from_row(row: &DatabentoMboCsvRow) -> Result<Self>
    where
        Self: Sized;
}

pub fn load_from_databento_csv<T: FromDatabentoRow>(filepath: &str) -> Result<Vec<T>> {
    let content = fs::read_to_string(filepath)?;
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut messages = Vec::new();

    for (line_num, result) in reader.deserialize::<DatabentoMboCsvRow>().enumerate() {
        let row = result.map_err(|e| anyhow!("Failed to parse CSV line {}: {:?}", line_num, e))?;
        let message = T::from_row(&row).map_err(|e| {
            anyhow!(
                "Failed to convert line {} to target type: {:?}",
                line_num,
                e
            )
        })?;
        messages.push(message);
    }

    Ok(messages)
}

impl FromDatabentoRow for TickData {
    fn from_row(row: &DatabentoMboCsvRow) -> Result<Self> {
        Ok(TickData {
            timestamp: row.ts_event,
            price: row.price,
            sequence: row.sequence,
            size: row.size,
            instrument_id: row.instrument_id,
            action: parse_action(&row.action)?,
            side: parse_side(&row.side)?,
            _padding: [0; 2],
        })
    }
}
