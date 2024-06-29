use anchor_lang::prelude::*;

declare_id!("4XPutW9b9V1Wm4RFHsyWafo86uUkSfK1TgEQSnmFqqdL");

#[program]
pub mod puppet_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
