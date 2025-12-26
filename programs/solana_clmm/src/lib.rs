use anchor_lang::prelude::*;

declare_id!("8HigeMisv1m1fPyQuDKjfJ2ptwrNb66NTWuGLVwFWGkf");

#[program]
pub mod solana_clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
