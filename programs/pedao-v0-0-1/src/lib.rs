use anchor_lang::prelude::*;

declare_id!("AMuUBpBnpMQzBkYr7JyURsYJKGSNiJYdGMarN9hEmJb7");

#[program]
pub mod pedao_v0_0_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
