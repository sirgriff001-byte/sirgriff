use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use std::mem::size_of;

/// Staking Pool State
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StakingPool {
    pub is_initialized: bool,
    pub owner: [u8; 32],
    pub mint: [u8; 32],
    pub token_account: [u8; 32],
    pub reward_account: [u8; 32],
    pub reward_per_slot: u64,
    pub lock_period: u64,
    pub total_staked: u64,
    pub last_reward_slot: u64,
}

impl Sealed for StakingPool {}

impl IsInitialized for StakingPool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for StakingPool {
    const LEN: usize = 1 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        if src.len() != Self::LEN {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }

        Ok(StakingPool {
            is_initialized: src[0] != 0,
            owner: {
                let mut owner = [0u8; 32];
                owner.copy_from_slice(&src[1..33]);
                owner
            },
            mint: {
                let mut mint = [0u8; 32];
                mint.copy_from_slice(&src[33..65]);
                mint
            },
            token_account: {
                let mut token_account = [0u8; 32];
                token_account.copy_from_slice(&src[65..97]);
                token_account
            },
            reward_account: {
                let mut reward_account = [0u8; 32];
                reward_account.copy_from_slice(&src[97..129]);
                reward_account
            },
            reward_per_slot: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[129..137]);
                bytes
            }),
            lock_period: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[137..145]);
                bytes
            }),
            total_staked: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[145..153]);
                bytes
            }),
            last_reward_slot: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[153..161]);
                bytes
            }),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        if dst.len() != Self::LEN {
            panic!("Buffer size mismatch");
        }

        dst[0] = self.is_initialized as u8;
        dst[1..33].copy_from_slice(&self.owner);
        dst[33..65].copy_from_slice(&self.mint);
        dst[65..97].copy_from_slice(&self.token_account);
        dst[97..129].copy_from_slice(&self.reward_account);
        dst[129..137].copy_from_slice(&self.reward_per_slot.to_le_bytes());
        dst[137..145].copy_from_slice(&self.lock_period.to_le_bytes());
        dst[145..153].copy_from_slice(&self.total_staked.to_le_bytes());
        dst[153..161].copy_from_slice(&self.last_reward_slot.to_le_bytes());
    }
}

/// User Stake Record
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StakeRecord {
    pub is_initialized: bool,
    pub user: [u8; 32],
    pub staked_amount: u64,
    pub staked_at_slot: u64,
    pub last_claim_slot: u64,
    pub total_rewards_claimed: u64,
}

impl Sealed for StakeRecord {}

impl IsInitialized for StakeRecord {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for StakeRecord {
    const LEN: usize = 1 + 32 + 8 + 8 + 8 + 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        if src.len() != Self::LEN {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }

        Ok(StakeRecord {
            is_initialized: src[0] != 0,
            user: {
                let mut user = [0u8; 32];
                user.copy_from_slice(&src[1..33]);
                user
            },
            staked_amount: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[33..41]);
                bytes
            }),
            staked_at_slot: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[41..49]);
                bytes
            }),
            last_claim_slot: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[49..57]);
                bytes
            }),
            total_rewards_claimed: u64::from_le_bytes({
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&src[57..65]);
                bytes
            }),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        if dst.len() != Self::LEN {
            panic!("Buffer size mismatch");
        }

        dst[0] = self.is_initialized as u8;
        dst[1..33].copy_from_slice(&self.user);
        dst[33..41].copy_from_slice(&self.staked_amount.to_le_bytes());
        dst[41..49].copy_from_slice(&self.staked_at_slot.to_le_bytes());
        dst[49..57].copy_from_slice(&self.last_claim_slot.to_le_bytes());
        dst[57..65].copy_from_slice(&self.total_rewards_claimed.to_le_bytes());
    }
}
