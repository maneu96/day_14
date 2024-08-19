use anchor_lang::prelude::*;

declare_id!("3WhAhhRKRxMsjYH5tdTcUeTiFDpKYxf54KerPbUNxgwp");

#[program]
pub mod day_14 {
    use super::*;

    pub fn initialize_three_sign(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1 = &mut ctx.accounts.signer1;
        let the_signer2 = &mut ctx.accounts.signer2;
        let the_signer3 = &mut ctx.accounts.signer3;
        
        msg!("The signer1: {:?}", the_signer1.key);
        msg!("The signer2: {:?}", the_signer2.key);
        msg!("The signer3: {:?}", the_signer3.key);

        Ok(())
    }


    pub fn initialize_two_sign(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1 = &mut ctx.accounts.signer1;
        let the_signer2 = &mut ctx.accounts.signer2;
        
        msg!("The signer1: {:?}", the_signer1.key);
        msg!("The signer2: {:?}", the_signer2.key);

        Ok(())
    }


    pub fn initialize(ctx: Context<Initialize>) -> Result<()>{
        let the_signer1 = &mut ctx.accounts.signer1;
        
        msg!("The signer1: {:?}", the_signer1.key);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>
}
