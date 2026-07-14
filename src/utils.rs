use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

use crate::state::{StakingPool, StakeRecord};
use crate::error::StakingError;

/// Calculate pending rewards for a user
pub fn calculate_rewards(
    pool: &StakingPool,
    stake_record: &StakeRecord,
    current_slot: u64,
) -> Result<u64, StakingError> {
    if stake_record.staked_amount == 0 {
        return Ok(0);
    }

    let slots_staked = current_slot
        .checked_sub(stake_record.last_claim_slot)
        .ok_or(StakingError::CalculationOverflow)?;

    let reward_amount = pool
        .reward_per_slot
        .checked_mul(slots_staked)
        .and_then(|r| r.checked_mul(stake_record.staked_amount))
        .ok_or(StakingError::CalculationOverflow)?;

    Ok(reward_amount)
}

/// Validate that lock period has expired
pub fn is_lock_period_expired(
    stake_record: &StakeRecord,
    pool: &StakingPool,
    current_slot: u64,
) -> bool {
    let slots_elapsed = current_slot.saturating_sub(stake_record.staked_at_slot);
    slots_elapsed >= pool.lock_period
}

/// Verify token account ownership
pub fn verify_token_account(
    account: &AccountInfo,
    owner: &Pubkey,
) -> ProgramResult {
    let token_account = TokenAccount::unpack(&account.data.borrow())?;
    if token_account.owner != *owner {
        return Err(StakingError::InvalidOwner.into());
    }
    Ok(())
}

/// Get PDA for staking pool
pub fn get_staking_pool_pda(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"staking_pool", mint.as_ref()], program_id)
}

/// Get PDA for stake record
pub fn get_stake_record_pda(program_id: &Pubkey, user: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"stake_record", user.as_ref(), mint.as_ref()],
        program_id,
    )
}

/// Get PDA for pool token account
pub fn get_pool_token_account_pda(
    program_id: &Pubkey,
    mint: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"pool_tokens", mint.as_ref()],
        program_id,
    )
}
