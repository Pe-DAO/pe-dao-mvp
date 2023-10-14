
use anchor_lang::prelude::*;


declare_id!("5JZfW3awchtCe3t51Ro5Dopsdutdma7znBF8bRRhyqn");

#[program]
pub mod pedao_v0_0_1 {
    use super::*;


    pub fn send_a_rate_street(send_a_rate_ctx:Context<SendARateStreet> , name: String, image_link: String, content: String, lat: f64, lng: f64) -> Result<( )>{
        if name.chars().count() > 50 {
            return err!(ErrorsRateTheStreet::NameTooLong);
        }
        if image_link.chars().count() > 200 {
            return err!(ErrorsRateTheStreet::ImageLinkTooLong);
        }
        if content.chars().count() > 280 {
            return err!(ErrorsRateTheStreet::ContentTooLong);
        }
        let rate_the_street = &mut send_a_rate_ctx.accounts.my_rate;
        let sender_of_rate = &mut send_a_rate_ctx.accounts.sender_of_rate;
        let clock = Clock::get().unwrap();

        rate_the_street.author = *sender_of_rate.key;
        rate_the_street.name = name;
        rate_the_street.content = content;
        rate_the_street.image_link = image_link;
        rate_the_street.lat = lat;
        rate_the_street.lng = lng;
        rate_the_street.timestamp = clock.unix_timestamp;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct SendARateStreet<'info>{
    #[account(init, payer=sender_of_rate, space=RateTheStreet::LEN )]
    pub my_rate: Account<'info, RateTheStreet>,
    #[account(mut)]
    pub sender_of_rate: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[error_code]
pub enum ErrorsRateTheStreet{
    #[msg("Name is too long")]
    NameTooLong,
    #[msg("Image is too long")]
    ImageLinkTooLong,
    #[msg("Content is too long")]
    ContentTooLong,
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

impl RateTheStreet {
    const LEN: usize = DISCRIMINATION_LENGTH + PUBLIC_KEY_LENGTH + 
    TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH 
    + STRING_LENGTH_PREFIX + MAX_IMAGE_LINK_LENGTH + LAT_LENGTH + LAT_LENGTH;
}
