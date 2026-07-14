use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
};
use solana_program::program_pack::Pack;

use crate::state::StakingPool;

/// Query the staking pool state
pub fn get_pool_state(
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_iter)?;

    let pool_data = pool_account.data.borrow();
    let pool = StakingPool::unpack(&pool_data)?;

    if !pool.is_initialized {
        msg!("Pool not initialized");
        return Ok(());
    }

    msg!("Pool State:");
    msg!("  Total Staked: {}", pool.total_staked);
    msg!("  Reward per Slot: {}", pool.reward_per_slot);
    msg!("  Lock Period (slots): {}", pool.lock_period);
    msg!("  Last Reward Slot: {}", pool.last_reward_slot);

    Ok(())
}

/// Query user stake record
pub fn get_user_stake(
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let stake_record_account = next_account_info(account_iter)?;

    let record_data = stake_record_account.data.borrow();
    let record = crate::state::StakeRecord::unpack(&record_data)?;

    if !record.is_initialized {
        msg!("Stake record not initialized");
        return Ok(());
    }

    msg!("Stake Record:");
    msg!("  Staked Amount: {}", record.staked_amount);
    msg!("  Staked at Slot: {}", record.staked_at_slot);
    msg!("  Last Claim Slot: {}", record.last_claim_slot);
    msg!("  Total Rewards Claimed: {}", record.total_rewards_claimed);

    Ok(())
}
