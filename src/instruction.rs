use solana_program::program_error::ProgramError;

pub enum StakingInstruction {
    /// Initialize the staking pool
    /// Accounts expected by this instruction:
    /// 0. `[signer, writable]` Initializer account
    /// 1. `[writable]` Staking pool PDA
    /// 2. `[writable]` Mint account for staking token
    /// 3. `[]` Rent sysvar
    /// 4. `[]` System program
    Initialize {
        reward_per_slot: u64,
        lock_period: u64,
    },

    /// Stake tokens
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User token account (ATA)
    /// 2. `[writable]` Staking pool token account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[]` Token program
    /// 5. `[]` System program
    Stake { amount: u64 },

    /// Unstake tokens
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User token account (ATA)
    /// 2. `[writable]` Staking pool token account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[]` Token program
    Unstake { amount: u64 },

    /// Claim staking rewards
    /// Accounts expected by this instruction:
    /// 0. `[signer]` User account
    /// 1. `[writable]` User reward token account (ATA)
    /// 2. `[writable]` Staking pool reward account
    /// 3. `[writable]` User stake record PDA
    /// 4. `[writable]` Staking pool PDA
    /// 5. `[]` Token program
    ClaimRewards,
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
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
