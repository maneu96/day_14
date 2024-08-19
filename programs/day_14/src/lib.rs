use anchor_lang::prelude::*;

declare_id!("3WhAhhRKRxMsjYH5tdTcUeTiFDpKYxf54KerPbUNxgwp");

const OWNER: &str = "ESyHSWtc6YwaFHW3kGrL9bA1A8qzKB8eyAYgz9y4G3KX";

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
    #[access_control(check(&ctx))]
    pub fn hello_owner(ctx: Context<Initialize>) -> Result<()> {
        msg!("Holla, I'm the owner.");
        Ok(())
    }

  
}

fn check(ctx: &Context<Initialize>) -> Result<()>{
    require_keys_eq!(
        ctx.accounts.signer1.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>
}


#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}