use anchor_lang::prelude::*;

declare_id!("3Z9WEGXY1V7RC5yEsCxyr7F9iS7mpm4rn4hzMrQnZanJ");

#[program]
pub mod puppet_master {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
