use anchor_lang::prelude::*;
pub mod errors;

declare_id!("75C83spJ9MzptEHEcQNisghBqsCacQ8jho29oqVGxBWt");

#[program]
pub mod state_progam_channel {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
