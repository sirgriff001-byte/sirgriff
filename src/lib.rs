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

pub use error::StakingError;
pub use instruction::StakingInstruction;
pub use processor::Processor;
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
    }
}
