import { PublicKey, Connection, TransactionInstruction, SystemProgram } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createTransferInstruction } from '@solana/spl-token';
import { PROGRAM_ID, StakingInstruction, STAKING_POOL_SEED, STAKE_RECORD_SEED, POOL_TOKENS_SEED } from './constants';

export class StakingClient {
  private connection: Connection;
  private programId: PublicKey;

  constructor(endpoint: string, programId: string = PROGRAM_ID) {
    this.connection = new Connection(endpoint);
    this.programId = new PublicKey(programId);
  }

  /**
   * Derive staking pool PDA
   */
  async getStakingPoolPda(mint: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );
  }

  /**
   * Derive stake record PDA
   */
  async getStakeRecordPda(user: PublicKey, mint: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from(STAKE_RECORD_SEED), user.toBuffer(), mint.toBuffer()],
      this.programId
    );
  }

  /**
   * Derive pool token account PDA
   */
  async getPoolTokenAccountPda(mint: PublicKey): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress(
      [Buffer.from(POOL_TOKENS_SEED), mint.toBuffer()],
      this.programId
    );
  }

  /**
   * Create Initialize instruction
   */
  createInitializeInstruction(
    initializer: PublicKey,
    mint: PublicKey,
    poolTokenAccount: PublicKey,
    poolRewardAccount: PublicKey,
    rewardPerSlot: bigint,
    lockPeriod: bigint
  ): TransactionInstruction {
    const [stakingPool] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );

    const data = Buffer.alloc(17);
    data[0] = StakingInstruction.Initialize;
    data.writeBigUInt64LE(rewardPerSlot, 1);
    data.writeBigUInt64LE(lockPeriod, 9);

    return new TransactionInstruction({
      programId: this.programId,
      keys: [
        { pubkey: initializer, isSigner: true, isWritable: true },
        { pubkey: stakingPool, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: false },
        { pubkey: poolTokenAccount, isSigner: false, isWritable: true },
        { pubkey: poolRewardAccount, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      data,
    });
  }

  /**
   * Create Stake instruction
   */
  createStakeInstruction(
    user: PublicKey,
    userTokenAccount: PublicKey,
    poolTokenAccount: PublicKey,
    mint: PublicKey,
    amount: bigint
  ): TransactionInstruction {
    const [stakingPool] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );

    const [stakeRecord] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKE_RECORD_SEED), user.toBuffer(), mint.toBuffer()],
      this.programId
    );

    const data = Buffer.alloc(9);
    data[0] = StakingInstruction.Stake;
    data.writeBigUInt64LE(amount, 1);

    return new TransactionInstruction({
      programId: this.programId,
      keys: [
        { pubkey: user, isSigner: true, isWritable: false },
        { pubkey: userTokenAccount, isSigner: false, isWritable: true },
        { pubkey: poolTokenAccount, isSigner: false, isWritable: true },
        { pubkey: stakeRecord, isSigner: false, isWritable: true },
        { pubkey: stakingPool, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      data,
    });
  }

  /**
   * Create Unstake instruction
   */
  createUnstakeInstruction(
    user: PublicKey,
    userTokenAccount: PublicKey,
    poolTokenAccount: PublicKey,
    mint: PublicKey,
    amount: bigint
  ): TransactionInstruction {
    const [stakingPool] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );

    const [stakeRecord] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKE_RECORD_SEED), user.toBuffer(), mint.toBuffer()],
      this.programId
    );

    const data = Buffer.alloc(9);
    data[0] = StakingInstruction.Unstake;
    data.writeBigUInt64LE(amount, 1);

    return new TransactionInstruction({
      programId: this.programId,
      keys: [
        { pubkey: user, isSigner: true, isWritable: false },
        { pubkey: userTokenAccount, isSigner: false, isWritable: true },
        { pubkey: poolTokenAccount, isSigner: false, isWritable: true },
        { pubkey: stakeRecord, isSigner: false, isWritable: true },
        { pubkey: stakingPool, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      data,
    });
  }

  /**
   * Create ClaimRewards instruction
   */
  createClaimRewardsInstruction(
    user: PublicKey,
    userRewardAccount: PublicKey,
    poolRewardAccount: PublicKey,
    mint: PublicKey
  ): TransactionInstruction {
    const [stakingPool] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );

    const [stakeRecord] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKE_RECORD_SEED), user.toBuffer(), mint.toBuffer()],
      this.programId
    );

    const data = Buffer.alloc(1);
    data[0] = StakingInstruction.ClaimRewards;

    return new TransactionInstruction({
      programId: this.programId,
      keys: [
        { pubkey: user, isSigner: true, isWritable: false },
        { pubkey: userRewardAccount, isSigner: false, isWritable: true },
        { pubkey: poolRewardAccount, isSigner: false, isWritable: true },
        { pubkey: stakeRecord, isSigner: false, isWritable: true },
        { pubkey: stakingPool, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ],
      data,
    });
  }

  /**
   * Get pool information
   */
  async getPoolInfo(mint: PublicKey): Promise<any> {
    const [stakingPool] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKING_POOL_SEED), mint.toBuffer()],
      this.programId
    );

    const account = await this.connection.getAccountInfo(stakingPool);
    if (!account) throw new Error('Pool account not found');

    // Parse pool data (simplified)
    return {
      address: stakingPool.toString(),
      data: account.data,
    };
  }

  /**
   * Get user stake information
   */
  async getUserStake(user: PublicKey, mint: PublicKey): Promise<any> {
    const [stakeRecord] = PublicKey.findProgramAddressSync(
      [Buffer.from(STAKE_RECORD_SEED), user.toBuffer(), mint.toBuffer()],
      this.programId
    );

    const account = await this.connection.getAccountInfo(stakeRecord);
    if (!account) throw new Error('Stake record not found');

    return {
      address: stakeRecord.toString(),
      data: account.data,
    };
  }
}
