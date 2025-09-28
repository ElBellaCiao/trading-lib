use crate::model::{BookSnapshot, SIDE_ASK, SIDE_BID, from_databento_to_price};
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
    pub bid_px_00: i64,
    pub bid_px_01: i64,
    pub bid_px_02: i64,
    pub bid_px_03: i64,
    pub bid_px_04: i64,
    pub bid_px_05: i64,
    pub bid_px_06: i64,
    pub bid_px_07: i64,
    pub bid_px_08: i64,
    pub bid_px_09: i64,
    pub ask_px_00: i64,
    pub ask_px_01: i64,
    pub ask_px_02: i64,
    pub ask_px_03: i64,
    pub ask_px_04: i64,
    pub ask_px_05: i64,
    pub ask_px_06: i64,
    pub ask_px_07: i64,
    pub ask_px_08: i64,
    pub ask_px_09: i64,
    pub bid_sz_00: u32,
    pub bid_sz_01: u32,
    pub bid_sz_02: u32,
    pub bid_sz_03: u32,
    pub bid_sz_04: u32,
    pub bid_sz_05: u32,
    pub bid_sz_06: u32,
    pub bid_sz_07: u32,
    pub bid_sz_08: u32,
    pub bid_sz_09: u32,
    pub ask_sz_00: u32,
    pub ask_sz_01: u32,
    pub ask_sz_02: u32,
    pub ask_sz_03: u32,
    pub ask_sz_04: u32,
    pub ask_sz_05: u32,
    pub ask_sz_06: u32,
    pub ask_sz_07: u32,
    pub ask_sz_08: u32,
    pub ask_sz_09: u32,
    pub bid_ct_00: u32,
    pub bid_ct_01: u32,
    pub bid_ct_02: u32,
    pub bid_ct_03: u32,
    pub bid_ct_04: u32,
    pub bid_ct_05: u32,
    pub bid_ct_06: u32,
    pub bid_ct_07: u32,
    pub bid_ct_08: u32,
    pub bid_ct_09: u32,
    pub ask_ct_00: u32,
    pub ask_ct_01: u32,
    pub ask_ct_02: u32,
    pub ask_ct_03: u32,
    pub ask_ct_04: u32,
    pub ask_ct_05: u32,
    pub ask_ct_06: u32,
    pub ask_ct_07: u32,
    pub ask_ct_08: u32,
    pub ask_ct_09: u32,
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
                from_databento_to_price(SIDE_BID, &row.bid_px_00),
                from_databento_to_price(SIDE_BID, &row.bid_px_01),
                from_databento_to_price(SIDE_BID, &row.bid_px_02),
                from_databento_to_price(SIDE_BID, &row.bid_px_03),
                from_databento_to_price(SIDE_BID, &row.bid_px_04),
                from_databento_to_price(SIDE_BID, &row.bid_px_05),
                from_databento_to_price(SIDE_BID, &row.bid_px_06),
                from_databento_to_price(SIDE_BID, &row.bid_px_07),
                from_databento_to_price(SIDE_BID, &row.bid_px_08),
                from_databento_to_price(SIDE_BID, &row.bid_px_09),
            ],
            best_bid_volume: [
                row.bid_sz_00,
                row.bid_sz_01,
                row.bid_sz_02,
                row.bid_sz_03,
                row.bid_sz_04,
                row.bid_sz_05,
                row.bid_sz_06,
                row.bid_sz_07,
                row.bid_sz_08,
                row.bid_sz_09,
            ],
            best_bid_quantity: [
                row.bid_ct_00,
                row.bid_ct_01,
                row.bid_ct_02,
                row.bid_ct_03,
                row.bid_ct_04,
                row.bid_ct_05,
                row.bid_ct_06,
                row.bid_ct_07,
                row.bid_ct_08,
                row.bid_ct_09,
            ],
            best_ask_price: [
                from_databento_to_price(SIDE_ASK, &row.ask_px_00),
                from_databento_to_price(SIDE_ASK, &row.ask_px_01),
                from_databento_to_price(SIDE_ASK, &row.ask_px_02),
                from_databento_to_price(SIDE_ASK, &row.ask_px_03),
                from_databento_to_price(SIDE_ASK, &row.ask_px_04),
                from_databento_to_price(SIDE_ASK, &row.ask_px_05),
                from_databento_to_price(SIDE_ASK, &row.ask_px_06),
                from_databento_to_price(SIDE_ASK, &row.ask_px_07),
                from_databento_to_price(SIDE_ASK, &row.ask_px_08),
                from_databento_to_price(SIDE_ASK, &row.ask_px_09),
            ],
            best_ask_volume: [
                row.ask_sz_00,
                row.ask_sz_01,
                row.ask_sz_02,
                row.ask_sz_03,
                row.ask_sz_04,
                row.ask_sz_05,
                row.ask_sz_06,
                row.ask_sz_07,
                row.ask_sz_08,
                row.ask_sz_09,
            ],
            best_ask_quantity: [
                row.ask_ct_00,
                row.ask_ct_01,
                row.ask_ct_02,
                row.ask_ct_03,
                row.ask_ct_04,
                row.ask_ct_05,
                row.ask_ct_06,
                row.ask_ct_07,
                row.ask_ct_08,
                row.ask_ct_09,
            ],
            _padding_0: [0u8; 32],
            _padding_1: [0u8; 16],
            _padding_2: [0u8; 8],
        })
    }
}
