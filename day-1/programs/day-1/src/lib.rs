use anchor_lang::prelude::*;

declare_id!("BDVDs1odqZysA3FNcia5vaycpBai1dq6PBvQThkKWr3M");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize1(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
