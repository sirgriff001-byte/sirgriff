use solana_program::program_error::ProgramError;

#[repr(u8)]
pub enum StakingInstruction {
    /// Initialize the staking pool
    /// Accounts expected by this instruction:
    /// 0. `[signer, writable]` Initializer account
    /// 1. `[writable]` Staking pool PDA
    /// 2. `[writable]` Mint account for staking token
    /// 3. `[writable]` Pool token account (for staked tokens)
    /// 4. `[writable]` Pool reward account
    /// 5. `[]` Rent sysvar
    /// 6. `[]` System program
    /// 7. `[]` Token program
    Initialize {
        reward_per_slot: u64,
        lock_period: u64,
    } = 0,

    /// Stake tokens
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User token account (ATA)
    /// 2. `[writable]` Pool token account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[writable]` Staking pool PDA
    /// 5. `[]` Mint account
    /// 6. `[]` System program
    /// 7. `[]` Token program
    Stake { amount: u64 } = 1,

    /// Unstake tokens
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User token account (ATA)
    /// 2. `[writable]` Pool token account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[writable]` Staking pool PDA
    /// 5. `[]` Mint account
    /// 6. `[]` Token program
    Unstake { amount: u64 } = 2,

    /// Claim staking rewards
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User reward token account (ATA)
    /// 2. `[writable]` Pool reward account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[writable]` Staking pool PDA
    /// 5. `[]` Mint account
    /// 6. `[]` Token program
    ClaimRewards = 3,

    /// Query staking pool state
    /// Accounts expected:
    /// 0. `[]` Staking pool PDA
    GetPoolState = 4,

    /// Query user stake record
    /// Accounts expected:
    /// 0. `[]` User stake record PDA
    GetUserStake = 5,

    /// Admin: Withdraw accumulated fees
    /// Accounts expected:
    /// 0. `[signer]` Admin account
    /// 1. `[writable]` Admin wallet (ATA)
    /// 2. `[writable]` Pool reward account
    /// 3. `[writable]` Staking pool PDA
    /// 4. `[]` Mint account
    /// 5. `[]` Token program
    WithdrawFees { amount: u64 } = 6,

    /// Admin: Update reward rate
    /// Accounts expected:
    /// 0. `[signer]` Admin account
    /// 1. `[writable]` Staking pool PDA
    UpdateRewardRate { new_reward_per_slot: u64 } = 7,

    /// Admin: Update lock period
    /// Accounts expected:
    /// 0. `[signer]` Admin account
    /// 1. `[writable]` Staking pool PDA
    UpdateLockPeriod { new_lock_period: u64 } = 8,
}

impl StakingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let reward_per_slot = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                let lock_period = u64::from_le_bytes(
                    rest[8..16]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::Initialize {
                    reward_per_slot,
                    lock_period,
                }
            }
            1 => {
                let amount = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::Stake { amount }
            }
            2 => {
                let amount = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::Unstake { amount }
            }
            3 => Self::ClaimRewards,
            4 => Self::GetPoolState,
            5 => Self::GetUserStake,
            6 => {
                let amount = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::WithdrawFees { amount }
            }
            7 => {
                let new_reward_per_slot = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::UpdateRewardRate { new_reward_per_slot }
            }
            8 => {
                let new_lock_period = u64::from_le_bytes(
                    rest[0..8]
                        .try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Self::UpdateLockPeriod { new_lock_period }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
