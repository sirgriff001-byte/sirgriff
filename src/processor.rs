use crate::{
    error::StakingError,
    state::{StakingPool, StakeRecord},
};
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
    sysvar::Sysvar,
};
use solana_program::program_pack::Pack;
use spl_token::instruction as token_instruction;

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
        let rent = Rent::get()?;

        // Check if initializer is signer
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check if staking pool is rent exempt
        if !rent.is_exempt(staking_pool.lamports(), staking_pool.data_len()) {
            return Err(StakingError::NotRentExempt.into());
        }

        let mut pool_data = staking_pool.data.borrow_mut();
        let mut pool = StakingPool::unpack_unchecked(&pool_data)?;

        if pool.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        pool.is_initialized = true;
        pool.owner = initializer.key.to_bytes();
        pool.mint = mint.key.to_bytes();
        pool.reward_per_slot = reward_per_slot;
        pool.lock_period = lock_period;
        pool.total_staked = 0;
        pool.last_reward_slot = 0;

        StakingPool::pack(pool, &mut pool_data)?;

        msg!("Staking pool initialized");
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
        let token_program = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if amount == 0 {
            return Err(StakingError::InvalidStakeAmount.into());
        }

        // Initialize stake record if needed
        if stake_record.lamports() == 0 {
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(65);

            invoke(
                &system_instruction::create_account(
                    user.key,
                    stake_record.key,
                    lamports,
                    65,
                    program_id,
                ),
                &[user.clone(), stake_record.clone(), system_program.clone()],
            )?;

            let mut record_data = stake_record.data.borrow_mut();
            let mut record = StakeRecord::default();
            record.is_initialized = true;
            record.user = user.key.to_bytes();
            record.staked_at_slot = 0;
            record.last_claim_slot = 0;
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

        StakeRecord::pack(record, &mut record_data)?;

        msg!("Staked {} tokens", amount);
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
        let token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut record_data = stake_record.data.borrow_mut();
        let mut record = StakeRecord::unpack(&record_data)?;

        if record.staked_amount < amount {
            return Err(StakingError::InsufficientFunds.into());
        }

        // Transfer tokens from pool back to user
        invoke(
            &token_instruction::transfer(
                token_program.key,
                pool_token_account.key,
                user_token_account.key,
                program_id,
                &[],
                amount,
            )?,
            &[pool_token_account.clone(), user_token_account.clone(), token_program.clone()],
        )?;

        record.staked_amount = record.staked_amount.checked_sub(amount)
            .ok_or(StakingError::CalculationOverflow)?;

        StakeRecord::pack(record, &mut record_data)?;

        msg!("Unstaked {} tokens", amount);
        Ok(())
    }

    pub fn claim_rewards(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        let user = next_account_info(account_iter)?;
        let _user_reward_account = next_account_info(account_iter)?;
        let _pool_reward_account = next_account_info(account_iter)?;
        let stake_record = next_account_info(account_iter)?;
        let _staking_pool = next_account_info(account_iter)?;
        let _token_program = next_account_info(account_iter)?;

        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut record_data = stake_record.data.borrow_mut();
        let mut record = StakeRecord::unpack(&record_data)?;

        if record.staked_amount == 0 {
            return Err(StakingError::NoStakedTokens.into());
        }

        // Placeholder for reward calculation logic
        // In a real implementation, you would calculate rewards based on:
        // - Current slot vs last_claim_slot
        // - Staked amount
        // - reward_per_slot from the pool

        msg!("Rewards claimed");
        Ok(())
    }
}
