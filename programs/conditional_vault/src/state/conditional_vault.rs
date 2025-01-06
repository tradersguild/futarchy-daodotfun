use super::*;
use anchor_spl::token_2022;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    Active,
    Finalized,
    Reverted,
}

#[account]
pub struct ConditionalVault {
    pub question: Pubkey,
    pub underlying_token_mint: Pubkey,
    pub underlying_token_account: Pubkey,
    pub conditional_token_mints: Vec<Pubkey>,
    pub pda_bump: u8,
    pub decimals: u8,
    pub seq_num: u64,
}

impl ConditionalVault {
    /// Checks that the vault's assets are always greater than its potential
    /// liabilities. Should be called anytime you mint or burn conditional
    /// tokens.
    ///
    /// `conditional_token_supplies` should be in the same order as
    /// `vault.conditional_token_mints`.
    pub fn invariant(
        &self,
        question: &Question,
        conditional_token_supplies: Vec<u64>,
        vault_underlying_balance: u64,
    ) -> Result<()> {
        // if the question isn't resolved, the vault should have more underlying
        // tokens than ANY conditional token mint's supply

        // if the question is resolved, the vault should have more underlying
        // tokens than the sum of the conditional token mint's supplies multiplied
        // by their respective payouts

        let max_possible_liability = if !question.is_resolved() {
            // safe because conditional_token_supplies is non-empty
            *conditional_token_supplies.iter().max().unwrap()
        } else {
            conditional_token_supplies
                .iter()
                .enumerate()
                .map(|(i, supply)| {
                    *supply as u128 * question.payout_numerators[i] as u128
                        / question.payout_denominator as u128
                })
                .sum::<u128>() as u64
        };

        require_gte!(vault_underlying_balance, max_possible_liability, VaultError::AssertFailed);

        Ok(())
    }

    pub fn validate_token_program(&self, token_program: &Pubkey) -> Result<()> {
        require_keys_eq!(
            *token_program,
            token_2022::ID,
            ConditionalVaultError::InvalidTokenProgram
        );
        Ok(())
    }

    pub fn validate_token_extensions(&self, mint: &Account<Mint>) -> Result<()> {
        if token_2022::has_transfer_fee(mint)? {
            return Err(ConditionalVaultError::TransferFeesNotSupported.into());
        }
        if token_2022::is_interest_bearing(mint)? {
            return Err(ConditionalVaultError::InterestBearingNotSupported.into());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! generate_vault_seeds {
    ($vault:expr) => {{
        &[
            b"conditional_vault",
            $vault.question.as_ref(),
            $vault.underlying_token_mint.as_ref(),
            &[$vault.pda_bump],
        ]
    }};
}
