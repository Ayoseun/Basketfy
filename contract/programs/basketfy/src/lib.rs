use anchor_lang::prelude::*;

declare_id!("5PhybSd1vd9RaBjQ8R2cdf5mz2ogemo82RR2gajEGKTg");

#[program]
pub mod basketfy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
