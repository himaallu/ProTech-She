use anchor_lang::prelude::*;

declare_id!("FBDPY83Qa7Hm81qq5SCVHwebthwFoVsQA2TGVXi94P31");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
    pub fn increment(ctx: Context<increment>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data += 1;
        Ok(())
    }
    pub fn decrement(ctx: Context<decrement>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data -=1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}
pub struct increment<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}
pub struct decrement<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
