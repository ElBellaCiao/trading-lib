use bytemuck::{Pod, Zeroable};

pub type Timestamp = u64;
pub type Price = i64;
pub type Sequence = u32;
pub type Size = u32;
pub type InstrumentId = u32;

// bytemuck::Pod does not support enums
// https://github.com/Lokathor/bytemuck/issues/84
pub type Side = u8;
pub const SIDE_NONE: Side = 0;
pub const SIDE_BID: Side = 1;
pub const SIDE_ASK: Side = 2;

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TickData {
    pub timestamp: Timestamp,
    pub price: Price,
    pub sequence: Sequence,
    pub size: Size,
    pub instrument_id: InstrumentId,
    pub side: Side,
    pub _padding: [u8; 3],
}

pub const TICK_DATA_SIZE: usize = size_of::<TickData>();
