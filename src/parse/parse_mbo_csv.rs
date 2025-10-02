use crate::model::TickData;
use crate::parse::{parse_action, parse_side};
use anyhow::{Result, anyhow};
use csv::Reader;
use databento::dbn::{FlagSet, MboMsg, RecordHeader, rtype};
use serde::Deserialize;
use std::fs;
use std::path::Path;

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

pub trait FromMboRow {
    fn from_mbo_csv(row: &DatabentoMboCsvRow) -> Result<Option<Self>>
    where
        Self: Sized;
}

pub fn load_from_databento_mbo_csv<T: FromMboRow>(filepath: impl AsRef<Path>) -> Result<Vec<T>> {
    let content = fs::read_to_string(filepath)?;
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut messages = Vec::new();

    for (line_num, result) in reader.deserialize::<DatabentoMboCsvRow>().enumerate() {
        let row = result.map_err(|e| anyhow!("Failed to parse CSV line {}: {:?}", line_num, e))?;
        if let Some(message) = T::from_mbo_csv(&row)? {
            messages.push(message);
        }
    }

    Ok(messages)
}

impl FromMboRow for MboMsg {
    fn from_mbo_csv(row: &DatabentoMboCsvRow) -> Result<Option<Self>> {
        if row.action != "A" && row.action != "C" {
            return Ok(None);
        }

        let mbo_message = Self {
            hd: RecordHeader::new::<MboMsg>(
                rtype::MBO,
                row.publisher_id,
                row.instrument_id,
                row.ts_event,
            ),
            order_id: row.order_id,
            price: row.price / 10_000_000,
            size: row.size,
            flags: FlagSet::from(row.flags),
            channel_id: row.channel_id,
            action: parse_action(&row.action)?,
            side: parse_side(&row.side)?,
            ts_recv: row.ts_recv,
            ts_in_delta: row.ts_in_delta,
            sequence: row.sequence,
        };

        Ok(Some(mbo_message))
    }
}

impl FromMboRow for TickData {
    fn from_mbo_csv(row: &DatabentoMboCsvRow) -> Result<Option<Self>> {
        if row.action != "A" && row.action != "C" {
            return Ok(None);
        }

        let tick_data = Self {
            timestamp: row.ts_event,
            price: row.price / 10_000_000,
            sequence: row.sequence,
            volume: row.size,
            instrument_id: row.instrument_id,
            action: parse_action(&row.action)?,
            side: parse_side(&row.side)?,
            order_id: row.order_id,
            _padding: [0; 26],
        };

        Ok(Some(tick_data))
    }
}
