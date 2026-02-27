use anchor_lang::prelude::*;
#[account]
pub struct UserPosition {
    user: Pubkey,
    market: Pubkey,
    commitment: [u8; 32],
    amount: u64,
    claimed: bool,
    created_at: i64,
    bump: u8,
}
