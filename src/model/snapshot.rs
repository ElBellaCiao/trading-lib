use crate::model::{Price, Timestamp, Volume};
use bytemuck::{Pod, Zeroable};

type Quantity = u32;

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct BookSnapshot {
    pub timestamp: Timestamp,
    pub best_bid_price: [Price; 10],
    pub best_bid_volume: [Volume; 10],
    pub best_bid_quantity: [Quantity; 10],
    pub best_ask_price: [Price; 10],
    pub best_ask_volume: [Volume; 10],
    pub best_ask_quantity: [Quantity; 10],
    pub _padding_0: [u8; 32],
    pub _padding_1: [u8; 16],
    pub _padding_2: [u8; 8],
}
