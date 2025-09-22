use bytemuck::{Pod, Zeroable};
use chrono::{DateTime, Utc};
use std::fmt;

pub type Timestamp = u64;
pub type Price = i64;
pub type Sequence = u32;
pub type Volume = u32;
pub type InstrumentId = u32;
pub type OrderId = u64;

// bytemuck::Pod does not support enums
// https://github.com/Lokathor/bytemuck/issues/84
pub type Side = u8;
pub const SIDE_NONE: Side = b'N';
pub const SIDE_ASK: Side = b'A';
pub const SIDE_BID: Side = b'B';

// Action constants
pub type Action = u8;
pub const ACTION_NONE: Action = b'N';
pub const ACTION_ADD: Action = b'A';
pub const ACTION_MODIFY: Action = b'M';
pub const ACTION_CANCEL: Action = b'C';
pub const ACTION_CLEAR: Action = b'R';
pub const ACTION_TRADE: Action = b'T';
pub const ACTION_FILL: Action = b'F';

// Order matters for Pod
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct TickData {
    pub timestamp: Timestamp,
    pub price: Price,
    pub order_id: OrderId,
    pub sequence: Sequence,
    pub size: Volume,
    pub instrument_id: InstrumentId,
    pub side: Side,
    pub action: Action,
    pub _padding: [u8; 26],
}

pub const TICK_DATA_SIZE: usize = size_of::<TickData>();

impl fmt::Display for TickData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert timestamp to UTC datetime
        let datetime = DateTime::<Utc>::from_timestamp(self.timestamp as i64, 0)
            .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());

        // Convert price from nano-units to actual price
        let actual_price = self.price as f64 / 1e9;

        // Convert side and action to chars
        let side_char = self.side as char;
        let action_char = self.action as char;

        write!(
            f,
            "TickData {{ timestamp: {}, price: {:.9}, order_id: {}, sequence: {}, size: {}, instrument_id: {}, side: '{}', action: '{}' }}",
            datetime.format("%Y-%m-%d %H:%M:%S UTC"),
            actual_price,
            self.order_id,
            self.sequence,
            self.size,
            self.instrument_id,
            side_char,
            action_char
        )
    }
}
