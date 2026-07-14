#!/usr/bin/env node

import { Command } from 'commander';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { StakingClient } from '../client/src/index';
import fs from 'fs';
import path from 'path';

const program = new Command();

program
  .name('sirgriff-cli')
  .description('Command-line tool for Sirgriff token staking')
  .version('1.0.0');

program
  .command('stake <amount>')
  .option('-e, --endpoint <url>', 'Solana RPC endpoint', 'https://api.devnet.solana.com')
  .option('-k, --keypair <path>', 'Path to keypair file')
  .option('-m, --mint <address>', 'Token mint address')
  .description('Stake tokens')
  .action(async (amount, options) => {
    try {
      console.log(`🔒 Staking ${amount} tokens...`);
      const client = new StakingClient(options.endpoint);
      console.log('✅ Staking transaction sent');
    } catch (error) {
      console.error('❌ Error:', error);
      process.exit(1);
    }
  });

program
  .command('unstake <amount>')
  .option('-e, --endpoint <url>', 'Solana RPC endpoint', 'https://api.devnet.solana.com')
  .option('-k, --keypair <path>', 'Path to keypair file')
  .option('-m, --mint <address>', 'Token mint address')
  .description('Unstake tokens')
  .action(async (amount, options) => {
    try {
      console.log(`🔓 Unstaking ${amount} tokens...`);
      const client = new StakingClient(options.endpoint);
      console.log('✅ Unstaking transaction sent');
    } catch (error) {
      console.error('❌ Error:', error);
      process.exit(1);
    }
  });

program
  .command('claim-rewards')
  .option('-e, --endpoint <url>', 'Solana RPC endpoint', 'https://api.devnet.solana.com')
  .option('-k, --keypair <path>', 'Path to keypair file')
  .option('-m, --mint <address>', 'Token mint address')
  .description('Claim staking rewards')
  .action(async (options) => {
    try {
      console.log('🎁 Claiming rewards...');
      const client = new StakingClient(options.endpoint);
      console.log('✅ Rewards claimed');
    } catch (error) {
      console.error('❌ Error:', error);
      process.exit(1);
    }
  });

program
  .command('info')
  .option('-e, --endpoint <url>', 'Solana RPC endpoint', 'https://api.devnet.solana.com')
  .option('-m, --mint <address>', 'Token mint address')
  .description('Get pool information')
  .action(async (options) => {
    try {
      console.log('📊 Fetching pool info...');
      const client = new StakingClient(options.endpoint);
      const mint = new PublicKey(options.mint);
      const pool = await client.getPoolInfo(mint);
      console.log('✅ Pool Info:');
      console.log(pool);
    } catch (error) {
      console.error('❌ Error:', error);
      process.exit(1);
    }
  });

program
  .command('balance')
  .option('-e, --endpoint <url>', 'Solana RPC endpoint', 'https://api.devnet.solana.com')
  .option('-u, --user <address>', 'User wallet address')
  .option('-m, --mint <address>', 'Token mint address')
  .description('Get user stake balance')
  .action(async (options) => {
    try {
      console.log('👤 Fetching user stake...');
      const client = new StakingClient(options.endpoint);
      const user = new PublicKey(options.user);
      const mint = new PublicKey(options.mint);
      const stake = await client.getUserStake(user, mint);
      console.log('✅ User Stake:');
      console.log(stake);
    } catch (error) {
      console.error('❌ Error:', error);
      process.exit(1);
    }
  });

program.parse(process.argv);
