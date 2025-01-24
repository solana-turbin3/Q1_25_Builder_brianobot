use anchor_lang::prelude::*;

declare_id!("ASnvhxNh4U9fwETxahiCCGa18LXjRNUDfvJnNAC5tRyg");

mod instructions;

#[program]
pub mod escrow {
    use super::*; // this brings everthing from the parent scope into the escrow mod scope

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
