use anchor_lang::prelude::*;

declare_id!("7dsPHAwPMArDiW4bX6n2s4C1S7qix2h7hfq2qZrPaGb6");

#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        if ctx.accounts.token_account.amount > 0 {
            ctx.accounts.my_account.data = data;
        }
        Ok(())
    }
}


#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64,
    mint: Pubkey
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(
        constraint = my_account.mint == token_account.mint,
        has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>
}
