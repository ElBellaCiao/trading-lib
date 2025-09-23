use crate::model::BookSnapshot;
use anyhow::{Result, anyhow};
use csv::Reader;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct DatabentoMbp10CsvRow {
    pub ts_recv: u64,
    pub ts_event: u64,
    #[allow(dead_code)]
    pub rtype: u8,
    pub publisher_id: u16,
    pub instrument_id: u32,
    pub action: String,
    pub side: String,
    pub depth: u8,
    pub price: i64,
    pub size: u32,
    pub flags: u8,
    pub ts_in_delta: i32,
    pub sequence: u32,
    pub bid_px_0: i64,
    pub bid_px_1: i64,
    pub bid_px_2: i64,
    pub bid_px_3: i64,
    pub bid_px_4: i64,
    pub bid_px_5: i64,
    pub bid_px_6: i64,
    pub bid_px_7: i64,
    pub bid_px_8: i64,
    pub bid_px_9: i64,
    pub ask_px_0: i64,
    pub ask_px_1: i64,
    pub ask_px_2: i64,
    pub ask_px_3: i64,
    pub ask_px_4: i64,
    pub ask_px_5: i64,
    pub ask_px_6: i64,
    pub ask_px_7: i64,
    pub ask_px_8: i64,
    pub ask_px_9: i64,
    pub bid_sz_0: u32,
    pub bid_sz_1: u32,
    pub bid_sz_2: u32,
    pub bid_sz_3: u32,
    pub bid_sz_4: u32,
    pub bid_sz_5: u32,
    pub bid_sz_6: u32,
    pub bid_sz_7: u32,
    pub bid_sz_8: u32,
    pub bid_sz_9: u32,
    pub ask_sz_0: u32,
    pub ask_sz_1: u32,
    pub ask_sz_2: u32,
    pub ask_sz_3: u32,
    pub ask_sz_4: u32,
    pub ask_sz_5: u32,
    pub ask_sz_6: u32,
    pub ask_sz_7: u32,
    pub ask_sz_8: u32,
    pub ask_sz_9: u32,
    pub bid_ct_0: u32,
    pub bid_ct_1: u32,
    pub bid_ct_2: u32,
    pub bid_ct_3: u32,
    pub bid_ct_4: u32,
    pub bid_ct_5: u32,
    pub bid_ct_6: u32,
    pub bid_ct_7: u32,
    pub bid_ct_8: u32,
    pub bid_ct_9: u32,
    pub ask_ct_0: u32,
    pub ask_ct_1: u32,
    pub ask_ct_2: u32,
    pub ask_ct_3: u32,
    pub ask_ct_4: u32,
    pub ask_ct_5: u32,
    pub ask_ct_6: u32,
    pub ask_ct_7: u32,
    pub ask_ct_8: u32,
    pub ask_ct_9: u32,
}

pub trait FromMbp10Row {
    fn from_mbp10_csv(row: &DatabentoMbp10CsvRow) -> Result<Self>
    where
        Self: Sized;
}

pub fn load_from_databento_mbp_csv<T: FromMbp10Row>(filepath: impl AsRef<Path>) -> Result<Vec<T>> {
    let content = fs::read_to_string(filepath)?;
    let mut reader = Reader::from_reader(content.as_bytes());
    let mut messages = Vec::new();

    for (line_num, result) in reader.deserialize::<DatabentoMbp10CsvRow>().enumerate() {
        let row = result.map_err(|e| anyhow!("Failed to parse CSV line {}: {:?}", line_num, e))?;
        let message = T::from_mbp10_csv(&row).map_err(|e| {
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

impl FromMbp10Row for BookSnapshot {
    fn from_mbp10_csv(row: &DatabentoMbp10CsvRow) -> Result<Self> {
        Ok(Self {
            timestamp: row.ts_event,
            best_bid_price: [
                row.bid_px_0,
                row.bid_px_1,
                row.bid_px_2,
                row.bid_px_3,
                row.bid_px_4,
                row.bid_px_5,
                row.bid_px_6,
                row.bid_px_7,
                row.bid_px_8,
                row.bid_px_9,
            ],
            best_bid_volume: [
                row.bid_sz_0,
                row.bid_sz_1,
                row.bid_sz_2,
                row.bid_sz_3,
                row.bid_sz_4,
                row.bid_sz_5,
                row.bid_sz_6,
                row.bid_sz_7,
                row.bid_sz_8,
                row.bid_sz_9,
            ],
            best_bid_quantity: [
                row.bid_ct_0,
                row.bid_ct_1,
                row.bid_ct_2,
                row.bid_ct_3,
                row.bid_ct_4,
                row.bid_ct_5,
                row.bid_ct_6,
                row.bid_ct_7,
                row.bid_ct_8,
                row.bid_ct_9,
            ],
            best_ask_price: [
                row.ask_px_0,
                row.ask_px_1,
                row.ask_px_2,
                row.ask_px_3,
                row.ask_px_4,
                row.ask_px_5,
                row.ask_px_6,
                row.ask_px_7,
                row.ask_px_8,
                row.ask_px_9,
            ],
            best_ask_volume: [
                row.ask_sz_0,
                row.ask_sz_1,
                row.ask_sz_2,
                row.ask_sz_3,
                row.ask_sz_4,
                row.ask_sz_5,
                row.ask_sz_6,
                row.ask_sz_7,
                row.ask_sz_8,
                row.ask_sz_9,
            ],
            best_ask_quantity: [
                row.ask_ct_0,
                row.ask_ct_1,
                row.ask_ct_2,
                row.ask_ct_3,
                row.ask_ct_4,
                row.ask_ct_5,
                row.ask_ct_6,
                row.ask_ct_7,
                row.ask_ct_8,
                row.ask_ct_9,
            ],
            _padding_0: [0u8; 32],
            _padding_1: [0u8; 16],
            _padding_2: [0u8; 8],
        })
    }
}
