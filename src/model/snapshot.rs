use crate::model::{Price, Timestamp, Volume};
use bytemuck::{Pod, Zeroable};

type Quantity = u32;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct PriceLevel {
    pub price: Price,
    pub volume: Volume,
    pub quantity: Quantity,
}
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct BookSnapshot {
    pub best_bid_table: [PriceLevel; 10],
    pub best_ask_table: [PriceLevel; 10],
    pub timestamp: Timestamp,
    pub _padding_0: [u8; 32],
    pub _padding_1: [u8; 16],
    pub _padding_2: [u8; 8],
}
