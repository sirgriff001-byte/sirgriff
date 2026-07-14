export const PROGRAM_ID = 'YOUR_PROGRAM_ID_HERE';

// Instruction discriminators
export enum StakingInstruction {
  Initialize = 0,
  Stake = 1,
  Unstake = 2,
  ClaimRewards = 3,
  GetPoolState = 4,
  GetUserStake = 5,
  WithdrawFees = 6,
  UpdateRewardRate = 7,
  UpdateLockPeriod = 8,
}

// PDA Seeds
export const STAKING_POOL_SEED = 'staking_pool';
export const STAKE_RECORD_SEED = 'stake_record';
export const POOL_TOKENS_SEED = 'pool_tokens';
