use anchor_lang::prelude::*;

declare_id!("4XPutW9b9V1Wm4RFHsyWafo86uUkSfK1TgEQSnmFqqdL");

#[program]
pub mod puppet_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

#[account]
pub struct Data {
    pub data: u64,
}

#[derive(Accounts)]
pub struct Initialize {}
