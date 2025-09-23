use crate::model::{Price, Timestamp, Volume};
use bytemuck::{Pod, Zeroable};

type Quantity = u32;

pub const BOOK_DEPTH: usize = 10;

#[repr(C, align(64))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct BookSnapshot {
    pub timestamp: Timestamp,
    pub best_bid_price: [Price; BOOK_DEPTH],
    pub best_bid_volume: [Volume; BOOK_DEPTH],
    pub best_bid_quantity: [Quantity; BOOK_DEPTH],
    pub best_ask_price: [Price; BOOK_DEPTH],
    pub best_ask_volume: [Volume; BOOK_DEPTH],
    pub best_ask_quantity: [Quantity; BOOK_DEPTH],
    pub _padding_0: [u8; 32],
    pub _padding_1: [u8; 16],
    pub _padding_2: [u8; 8],
}
