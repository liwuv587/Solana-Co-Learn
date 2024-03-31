use anchor_lang::prelude::*;

declare_id!("7dsPHAwPMArDiW4bX6n2s4C1S7qix2h7hfq2qZrPaGb6");

#[program]
pub mod esp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
