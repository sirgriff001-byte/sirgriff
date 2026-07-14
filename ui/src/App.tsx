import React, { useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { StakingClient } from 'sirgriff-staking-client';
import './App.css';

interface PoolInfo {
  totalStaked: number;
  rewardPerSlot: number;
  lockPeriod: number;
}

interface UserStake {
  stakedAmount: number;
  stakedAtSlot: number;
  lastClaimSlot: number;
  totalRewardsClaimed: number;
}

function App() {
  const { publicKey, sendTransaction } = useWallet();
  const [client, setClient] = useState<StakingClient | null>(null);
  const [poolInfo, setPoolInfo] = useState<PoolInfo | null>(null);
  const [userStake, setUserStake] = useState<UserStake | null>(null);
  const [stakeAmount, setStakeAmount] = useState('');
  const [unstakeAmount, setUnstakeAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  // Initialize client on mount
  useEffect(() => {
    const staking = new StakingClient('https://api.devnet.solana.com');
    setClient(staking);
  }, []);

  // Fetch pool and user data
  useEffect(() => {
    if (client && publicKey) {
      fetchPoolInfo();
      fetchUserStake();
    }
  }, [client, publicKey]);

  const fetchPoolInfo = async () => {
    try {
      // TODO: Parse and set pool info
      setMessage('Pool info loaded');
    } catch (error) {
      setMessage('Error loading pool info');
    }
  };

  const fetchUserStake = async () => {
    try {
      // TODO: Parse and set user stake
      setMessage('User stake loaded');
    } catch (error) {
      setMessage('Error loading user stake');
    }
  };

  const handleStake = async () => {
    if (!client || !publicKey || !stakeAmount) return;

    setLoading(true);
    try {
      // TODO: Implement stake transaction
      setMessage(`Staking ${stakeAmount} tokens...`);
      setStakeAmount('');
      await fetchUserStake();
    } catch (error) {
      setMessage('Staking failed');
    } finally {
      setLoading(false);
    }
  };

  const handleUnstake = async () => {
    if (!client || !publicKey || !unstakeAmount) return;

    setLoading(true);
    try {
      // TODO: Implement unstake transaction
      setMessage(`Unstaking ${unstakeAmount} tokens...`);
      setUnstakeAmount('');
      await fetchUserStake();
    } catch (error) {
      setMessage('Unstaking failed');
    } finally {
      setLoading(false);
    }
  };

  const handleClaimRewards = async () => {
    if (!client || !publicKey) return;

    setLoading(true);
    try {
      // TODO: Implement claim rewards transaction
      setMessage('Claiming rewards...');
      await fetchUserStake();
    } catch (error) {
      setMessage('Claim failed');
    } finally {
      setLoading(false);
    }
  };

  if (!publicKey) {
    return (
      <div className="container">
        <div className="header">
          <h1>🚀 Sirgriff Token Staking</h1>
          <p>Connect your wallet to start staking</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container">
      <div className="header">
        <h1>🚀 Sirgriff Token Staking</h1>
        <p>Connected: {publicKey.toString().slice(0, 8)}...</p>
      </div>

      <div className="content">
        {/* Pool Info */}
        {poolInfo && (
          <div className="card pool-info">
            <h2>📊 Pool Information</h2>
            <div className="info-row">
              <span>Total Staked:</span>
              <strong>{poolInfo.totalStaked} tokens</strong>
            </div>
            <div className="info-row">
              <span>Reward Rate:</span>
              <strong>{poolInfo.rewardPerSlot} tokens/slot</strong>
            </div>
            <div className="info-row">
              <span>Lock Period:</span>
              <strong>{poolInfo.lockPeriod} slots</strong>
            </div>
          </div>
        )}

        {/* User Stake */}
        {userStake && (
          <div className="card user-stake">
            <h2>👤 Your Stake</h2>
            <div className="info-row">
              <span>Staked Amount:</span>
              <strong>{userStake.stakedAmount} tokens</strong>
            </div>
            <div className="info-row">
              <span>Rewards Claimed:</span>
              <strong>{userStake.totalRewardsClaimed} tokens</strong>
            </div>
          </div>
        )}

        {/* Stake Input */}
        <div className="card action">
          <h2>📥 Stake Tokens</h2>
          <div className="input-group">
            <input
              type="number"
              placeholder="Amount to stake"
              value={stakeAmount}
              onChange={(e) => setStakeAmount(e.target.value)}
              disabled={loading}
            />
            <button onClick={handleStake} disabled={loading}>
              {loading ? 'Processing...' : 'Stake'}
            </button>
          </div>
        </div>

        {/* Unstake Input */}
        <div className="card action">
          <h2>📤 Unstake Tokens</h2>
          <div className="input-group">
            <input
              type="number"
              placeholder="Amount to unstake"
              value={unstakeAmount}
              onChange={(e) => setUnstakeAmount(e.target.value)}
              disabled={loading}
            />
            <button onClick={handleUnstake} disabled={loading}>
              {loading ? 'Processing...' : 'Unstake'}
            </button>
          </div>
        </div>

        {/* Claim Rewards */}
        <div className="card action">
          <h2>🎁 Claim Rewards</h2>
          <button onClick={handleClaimRewards} disabled={loading} className="primary">
            {loading ? 'Processing...' : 'Claim Rewards'}
          </button>
        </div>

        {/* Messages */}
        {message && <div className="message">{message}</div>}
      </div>
    </div>
  );
}

export default App;
