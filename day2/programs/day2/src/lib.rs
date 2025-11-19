use anchor_lang::prelude::*;

declare_id!("2qmnwXjoBaYYDkJh7U8tPp8Nvq22yxEf9xGUfJLwFXBh");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        a: u64,
        b: u64,
        message: String
    ) -> Result<()> {
        msg!("The string you have sent: {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(
        ctx: Context<Initialize>,
        arr: Vec<u64>
    ) -> Result<()> {
        msg!("The array is : {:?}", arr);
        Ok(())
    }

    pub fn underflowtest(
        ctx: Context<Initialize>,
        x: u64,
        y: u64,
    ) -> Result<()> {
        let res_safe: u64 = x - y; 
        msg!("The return result is : {}", res_safe);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
