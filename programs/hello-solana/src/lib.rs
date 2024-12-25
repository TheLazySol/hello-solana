use anchor_lang::prelude::*;

declare_id!("Exm4TyzRD2Wxt8SXfFKJDQKgCyaoWyKeGZ8JEmG3fHbK");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
