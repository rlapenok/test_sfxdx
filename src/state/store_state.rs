use borsh::{BorshDeserialize, BorshSerialize};

pub const SEED: &str = "sfxdx";

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Store {
    pub init: [u8; 32],
    pub price: f64,
}

impl Store {
    pub fn new(price: f64, pubkey: [u8; 32]) -> Self {
        Self {
            init: pubkey,
            price: price,
        }
    }
    pub fn update(&mut self, price: f64) {
        self.price = price;
    }
}
