use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bundle {
    pub bundle_identifier: u64,
    pub interval: i64,
    pub amount_per_interval: u64,
    pub last_paid: i64,
}