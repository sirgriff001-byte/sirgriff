use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::instruction as token_instruction;
use spl_associated_token_account::get_associated_token_address;

pub struct TokenHelper;

impl TokenHelper {
    /// Create token account with required SOL rent
    pub fn create_token_account(
        user: &AccountInfo,
        token_account: &AccountInfo,
        mint: &Pubkey,
        owner: &Pubkey,
        rent: &Rent,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        // Calculate rent required
        let rent_lamports = rent.minimum_balance(165); // 165 is token account size

        // Create account
        invoke(
            &system_instruction::create_account(
                user.key,
                token_account.key,
                rent_lamports,
                165,
                token_program.key,
            ),
            &[user.clone(), token_account.clone(), system_program.clone()],
        )?;

        // Initialize token account
        invoke(
            &token_instruction::initialize_account(
                token_program.key,
                token_account.key,
                mint,
                owner,
            )?,
            &[token_account.clone(), mint.clone(), token_program.clone()],
        )?;

        Ok(())
    }

    /// Mint tokens to a specific account
    pub fn mint_tokens(
        mint: &AccountInfo,
        destination: &AccountInfo,
        mint_authority: &AccountInfo,
        amount: u64,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        invoke(
            &token_instruction::mint_to(
                token_program.key,
                mint.key,
                destination.key,
                mint_authority.key,
                &[],
                amount,
            )?,
            &[mint.clone(), destination.clone(), mint_authority.clone(), token_program.clone()],
        )
    }
}
