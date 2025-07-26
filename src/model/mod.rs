use bincode::{Decode, Encode};

pub type Timestamp = u64;
pub type Price = i64;
pub type Sequence = u32;
pub type Size = u32;
pub type InstrumentId = u32;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum Side {
    None = 0,
    Bid = 1,
    Ask = 2,
}

// Do alignment for all
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct TickData {
    pub timestamp: Timestamp,
    pub price: Price,
    pub sequence: Sequence,
    pub size: Size,
    pub instrument_id: InstrumentId,
    pub side: Side,
}
