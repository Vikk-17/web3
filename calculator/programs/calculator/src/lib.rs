use anchor_lang::prelude::*;

declare_id!("BVDfvLSCQuLWoqLH1xFpPDkJ4WAji3PXyMttuYbCPm7j");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn addition(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        let result_safe = a.checked_add(b).unwrap();
        msg!("Addition of {} & {} is {}", a, b, result_safe);
        Ok(())
    }

    pub fn substraction(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        if b > a {
            return err!(MyError::BisTooBig);
        }
        else {
            let result_safe = a.checked_sub(b).unwrap();
            msg!("Substraction of {} & {} is {}", a, b, result_safe);
            Ok(())
        }
    }

    pub fn multiplication(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        let result_safe = a.checked_mul(b).unwrap();
        msg!("Multiplication of {} & {} is {}", a, b, result_safe);
        Ok(())
    }

    pub fn division(
        ctx: Context<Initialize>,
        a: u64,
        b: u64
    ) -> Result<()> {
        let result_safe = a.checked_div(b).unwrap();
        msg!("Division of {} & {} is {}", a, b, result_safe);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>
}

#[error_code]
pub enum MyError {
    #[msg("A is too Big")]
    AisTooBig,
    #[msg("B is too Big")]
    BisTooBig,
}
