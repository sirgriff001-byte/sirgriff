use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::{clock::Clock, Sysvar},
};
use solana_program::program_pack::Pack;
use spl_token::instruction as token_instruction;
use spl_associated_token_account::get_associated_token_address;

use crate::{
    error::StakingError,
    state::{StakingPool, StakeRecord},
    utils,
    token_helper::TokenHelper,
};

pub struct AdminProcessor;

impl AdminProcessor {
    /// Withdraw accumulated rewards to admin wallet
    pub fn withdraw_fees(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let admin = next_account_info(account_iter)?;
        let admin_wallet = next_account_info(account_iter)?;
        let pool_reward_account = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;
        let mint = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        if !admin.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let pool_data = staking_pool.data.borrow();
        let pool = StakingPool::unpack(&pool_data)?;

        // Verify admin
        if pool.owner != admin.key.to_bytes() {
            return Err(StakingError::InvalidOwner.into());
        }
        drop(pool_data);

        let (_, bump) = utils::get_staking_pool_pda(program_id, mint.key);

        invoke_signed(
            &token_instruction::transfer(
                token_program.key,
                pool_reward_account.key,
                admin_wallet.key,
                staking_pool.key,
                &[],
                amount,
            )?,
            &[pool_reward_account.clone(), admin_wallet.clone(), staking_pool.clone(), token_program.clone()],
            &[&[b"staking_pool", mint.key.as_ref(), &[bump]]],
        )?;

        msg!("Withdrew {} tokens to admin wallet", amount);
        Ok(())
    }

    /// Update reward rate (admin only)
    pub fn update_reward_rate(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_reward_per_slot: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let admin = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;

        if !admin.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack(&pool_data)?;

        if pool.owner != admin.key.to_bytes() {
            return Err(StakingError::InvalidOwner.into());
        }

        let old_rate = pool.reward_per_slot;
        pool.reward_per_slot = new_reward_per_slot;

        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Updated reward rate from {} to {} per slot", old_rate, new_reward_per_slot);
        Ok(())
    }

    /// Update lock period (admin only)
    pub fn update_lock_period(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_lock_period: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let admin = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;

        if !admin.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack(&pool_data)?;

        if pool.owner != admin.key.to_bytes() {
            return Err(StakingError::InvalidOwner.into());
        }

        let old_period = pool.lock_period;
        pool.lock_period = new_lock_period;

        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Updated lock period from {} to {} slots", old_period, new_lock_period);
        Ok(())
    }
}
