use super::*;
use anchor_spl::token_2022::{self, Token2022};

#[derive(Accounts)]
pub struct SplitTokens<'info> {
    pub token_program: Program<'info, Token2022>,
    // ... other accounts
}

impl SplitTokens<'_> {
    pub fn handle(&self) -> Result<()> {
        // Add validation
        self.vault.validate_token_program(&self.token_program.key())?;
        
        // Update CPI calls
        token_2022::transfer(/* ... */)?;
        token_2022::mint_to(/* ... */)?;
    }
}
