use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
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
use spl_token::instruction as token_instruction;

mod error;
mod instruction;
mod processor;
mod state;
mod utils;
mod query;
mod token_helper;
mod admin;

pub use error::StakingError;
pub use instruction::StakingInstruction;
pub use processor::Processor;
pub use admin::AdminProcessor;
pub use state::*;

// Program entry point
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Sirgriff Token Staking Program");

    let instruction = StakingInstruction::unpack(instruction_data)?;

    match instruction {
        StakingInstruction::Initialize {
            reward_per_slot,
            lock_period,
        } => {
            msg!("Initializing staking pool");
            Processor::initialize(
                program_id,
                accounts,
                reward_per_slot,
                lock_period,
            )
        }
        StakingInstruction::Stake { amount } => {
            msg!("Staking tokens: {}", amount);
            Processor::stake(program_id, accounts, amount)
        }
        StakingInstruction::Unstake { amount } => {
            msg!("Unstaking tokens: {}", amount);
            Processor::unstake(program_id, accounts, amount)
        }
        StakingInstruction::ClaimRewards => {
            msg!("Claiming staking rewards");
            Processor::claim_rewards(program_id, accounts)
        }
        StakingInstruction::GetPoolState => {
            msg!("Querying pool state");
            query::get_pool_state(accounts)
        }
        StakingInstruction::GetUserStake => {
            msg!("Querying user stake");
            query::get_user_stake(accounts)
        }
        StakingInstruction::WithdrawFees { amount } => {
            msg!("Admin withdrawing fees: {}", amount);
            AdminProcessor::withdraw_fees(program_id, accounts, amount)
        }
        StakingInstruction::UpdateRewardRate { new_reward_per_slot } => {
            msg!("Admin updating reward rate: {}", new_reward_per_slot);
            AdminProcessor::update_reward_rate(program_id, accounts, new_reward_per_slot)
        }
        StakingInstruction::UpdateLockPeriod { new_lock_period } => {
            msg!("Admin updating lock period: {}", new_lock_period);
            AdminProcessor::update_lock_period(program_id, accounts, new_lock_period)
        }
    }
}
