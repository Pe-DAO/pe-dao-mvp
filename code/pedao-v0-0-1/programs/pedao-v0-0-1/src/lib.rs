use anchor_lang::prelude::*;

declare_id!("5JZfW3awchtCe3t51Ro5Dopsdutdma7znBF8bRRhyqn");

#[program]
pub mod pedao_v0_0_1 {
    use super::*;


}

#[account]
pub struct RateTheStreet{
    pub author: Pubkey,
    pub name: String,
    pub timestamp: i64,
    pub image_link: String,
    pub content: String,
    pub lat: f64,
    pub lng: f64 
}

const DISCRIMINATION_LENGTH:usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;
const MAX_IMAGE_LINK_LENGTH: usize = 200 * 4;
const LAT_LENGTH: usize = 8;
const LNG_LENGTH: usize = 8;

impl  RateTheStreet {
    const LEN: usize = DISCRIMINATION_LENGTH + PUBLIC_KEY_LENGTH + 
    TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH 
    + STRING_LENGTH_PREFIX + MAX_IMAGE_LINK_LENGTH + LAT_LENGTH + LAT_LENGTH;
}
