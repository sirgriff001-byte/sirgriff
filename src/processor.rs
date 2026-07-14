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
};

pub struct Processor;

impl Processor {
    pub fn initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        reward_per_slot: u64,
        lock_period: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let initializer = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;
        let mint = next_account_info(account_iter)?;
        let pool_token_account = next_account_info(account_iter)?;
        let pool_reward_account = next_account_info(account_iter)?;
        let rent = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        // Check if initializer is signer
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent_sysvar = Rent::get()?;

        // Check if staking pool is rent exempt
        if !rent_sysvar.is_exempt(staking_pool.lamports(), staking_pool.data_len()) {
            return Err(StakingError::NotRentExempt.into());
        }

        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack_unchecked(&pool_data)?;

        if pool.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let clock = Clock::get()?;

        pool.is_initialized = true;
        pool.owner = initializer.key.to_bytes();
        pool.mint = mint.key.to_bytes();
        pool.token_account = pool_token_account.key.to_bytes();
        pool.reward_account = pool_reward_account.key.to_bytes();
        pool.reward_per_slot = reward_per_slot;
        pool.lock_period = lock_period;
        pool.total_staked = 0;
        pool.last_reward_slot = clock.slot;

        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Staking pool initialized with reward_per_slot: {}, lock_period: {}", reward_per_slot, lock_period);
        Ok(())
    }

    pub fn stake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user = next_account_info(account_iter)?;
        let user_token_account = next_account_info(account_iter)?;
        let pool_token_account = next_account_info(account_iter)?;
        let stake_record = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;
        let mint = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if amount == 0 {
            return Err(StakingError::InvalidStakeAmount.into());
        }

        let clock = Clock::get()?;
        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack(&pool_data)?;

        // Initialize stake record if needed
        if stake_record.lamports() == 0 {
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(65);
            let (stake_record_pda, bump) = utils::get_stake_record_pda(program_id, user.key, mint.key);

            if stake_record.key != &stake_record_pda {
                return Err(ProgramError::InvalidSeeds);
            }

            invoke_signed(
                &system_instruction::create_account(
                    user.key,
                    stake_record.key,
                    lamports,
                    65,
                    program_id,
                ),
                &[user.clone(), stake_record.clone(), system_program.clone()],
                &[&[b"stake_record", user.key.as_ref(), mint.key.as_ref(), &[bump]]],
            )?;

            let mut record_data = stake_record.data.borrow_mut();
            let mut record = StakeRecord::default();
            record.is_initialized = true;
            record.user = user.key.to_bytes();
            record.staked_at_slot = clock.slot;
            record.last_claim_slot = clock.slot;
            record.total_rewards_claimed = 0;

            StakeRecord::pack(record, &mut record_data)?;
        }

        // Transfer tokens from user to pool
        invoke(
            &token_instruction::transfer(
                token_program.key,
                user_token_account.key,
                pool_token_account.key,
                user.key,
                &[],
                amount,
            )?,
            &[user_token_account.clone(), pool_token_account.clone(), user.clone(), token_program.clone()],
        )?;

        // Update stake record
        let mut record_data = stake_record.data.borrow_mut();
        let mut record = StakeRecord::unpack(&record_data)?;
        record.staked_amount = record.staked_amount.checked_add(amount)
            .ok_or(StakingError::CalculationOverflow)?;
        record.last_claim_slot = clock.slot;

        StakeRecord::pack(record, &mut record_data)?;

        // Update pool
        pool.total_staked = pool.total_staked.checked_add(amount)
            .ok_or(StakingError::CalculationOverflow)?;
        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Staked {} tokens for user {}", amount, user.key);
        Ok(())
    }

    pub fn unstake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user = next_account_info(account_iter)?;
        let user_token_account = next_account_info(account_iter)?;
        let pool_token_account = next_account_info(account_iter)?;
        let stake_record = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;
        let mint = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let clock = Clock::get()?;
        let mut record_data = stake_record.data.borrow_mut();
        let mut record = StakeRecord::unpack(&record_data)?;

        if record.staked_amount < amount {
            return Err(StakingError::InsufficientFunds.into());
        }

        // Check lock period
        let pool_data = staking_pool.data.borrow();
        let pool = StakingPool::unpack(&pool_data)?;
        if !utils::is_lock_period_expired(&record, &pool, clock.slot) {
            return Err(StakingError::LockPeriodNotExpired.into());
        }
        drop(pool_data);

        // Get bump seed for PDA signer
        let (_, bump) = utils::get_pool_token_account_pda(program_id, mint.key);

        // Transfer tokens from pool back to user
        invoke_signed(
            &token_instruction::transfer(
                token_program.key,
                pool_token_account.key,
                user_token_account.key,
                staking_pool.key,
                &[],
                amount,
            )?,
            &[pool_token_account.clone(), user_token_account.clone(), staking_pool.clone(), token_program.clone()],
            &[&[b"pool_tokens", mint.key.as_ref(), &[bump]]],
        )?;

        record.staked_amount = record.staked_amount.checked_sub(amount)
            .ok_or(StakingError::CalculationOverflow)?;

        StakeRecord::pack(record, &mut record_data)?;

        // Update pool
        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack(&pool_data)?;
        pool.total_staked = pool.total_staked.checked_sub(amount)
            .ok_or(StakingError::CalculationOverflow)?;
        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Unstaked {} tokens for user {}", amount, user.key);
        Ok(())
    }

    pub fn claim_rewards(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user = next_account_info(account_iter)?;
        let user_reward_account = next_account_info(account_iter)?;
        let pool_reward_account = next_account_info(account_iter)?;
        let stake_record = next_account_info(account_iter)?;
        let staking_pool = next_account_info(account_iter)?;
        let mint = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let clock = Clock::get()?;
        let mut record_data = stake_record.data.borrow_mut();
        let mut record = StakeRecord::unpack(&record_data)?;

        if record.staked_amount == 0 {
            return Err(StakingError::NoStakedTokens.into());
        }

        let pool_data = staking_pool.data.borrow();
        let pool = StakingPool::unpack(&pool_data)?;

        // Calculate rewards
        let reward_amount = utils::calculate_rewards(&pool, &record, clock.slot)?;

        if reward_amount == 0 {
            msg!("No rewards to claim yet");
            return Ok(());
        }

        // Get bump seed for PDA signer
        let (_, bump) = utils::get_staking_pool_pda(program_id, mint.key);

        // Transfer reward tokens
        invoke_signed(
            &token_instruction::transfer(
                token_program.key,
                pool_reward_account.key,
                user_reward_account.key,
                staking_pool.key,
                &[],
                reward_amount,
            )?,
            &[pool_reward_account.clone(), user_reward_account.clone(), staking_pool.clone(), token_program.clone()],
            &[&[b"staking_pool", mint.key.as_ref(), &[bump]]],
        )?;

        record.last_claim_slot = clock.slot;
        record.total_rewards_claimed = record.total_rewards_claimed.checked_add(reward_amount)
            .ok_or(StakingError::CalculationOverflow)?;

        StakeRecord::pack(record, &mut record_data)?;

        msg!("Claimed {} reward tokens for user {}", reward_amount, user.key);
        Ok(())
    }
}
