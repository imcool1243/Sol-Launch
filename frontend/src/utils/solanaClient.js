import { Connection, PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { Program, AnchorProvider, web3, BN } from '@project-serum/anchor';
import { IDL as SolLaunchIDL } from '../idl/sol_launch.json';

const PROGRAM_ID = new PublicKey('2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj');

class SolanaClient {
  constructor() {
    this.connection = new Connection('https://api.devnet.solana.com');
    this.program = null;
    this.provider = null;
  }

  async initialize(wallet) {
    this.provider = new AnchorProvider(
      this.connection,
      wallet,
      { commitment: 'confirmed' }
    );
    
    this.program = new Program(
      SolLaunchIDL,
      PROGRAM_ID,
      this.provider
    );
    
    return this.program;
  }

  async getLaunchState(launchPubkey) {
    try {
      const launchAccount = await this.program.account.launchState.fetch(launchPubkey);
      return launchAccount;
    } catch (error) {
      console.error('Failed to fetch launch state:', error);
      throw error;
    }
  }

  async getTradeState(tradePubkey) {
    try {
      const tradeAccount = await this.program.account.tradeState.fetch(tradePubkey);
      return tradeAccount;
    } catch (error) {
      console.error('Failed to fetch trade state:', error);
      throw error;
    }
  }

  async initializeLaunch(params) {
    try {
      const launchKeypair = web3.Keypair.generate();
      const vaultKeypair = web3.Keypair.generate();
      
      const tx = await this.program.methods
        .initializeLaunch(
          new BN(params.maxBuy),
          new BN(params.maxWallet),
          new BN(params.cooldownSeconds),
          new BN(params.totalSupply),
          params.sniperProtectionEnabled,
          new BN(params.minTradingDuration),
          // Enhanced anti-sniper parameters
          params.walletBlacklistEnabled,
          params.progressiveLimitsEnabled,
          new BN(params.initialMaxBuy),
          new BN(params.initialMaxWallet),
          new BN(params.limitIncreaseInterval),
          new BN(params.limitIncreaseMultiplier),
          params.antiScamEnabled,
          new BN(params.maxTradesPerUser),
        )
        .accounts({
          launch: launchKeypair.publicKey,
          authority: this.provider.wallet.publicKey,
          tokenMint: new PublicKey(params.tokenMint),
          vault: vaultKeypair.publicKey,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
          systemProgram: SystemProgram.programId,
        })
        .signers([launchKeypair, vaultKeypair])
        .rpc();

      return {
        launchPubkey: launchKeypair.publicKey.toString(),
        vaultPubkey: vaultKeypair.publicKey.toString(),
        signature: tx
      };
    } catch (error) {
      console.error('Failed to initialize launch:', error);
      throw error;
    }
  }

  async depositTokens(launchPubkey, amount) {
    try {
      const fromTokenAccount = await this.getTokenAccount(this.provider.wallet.publicKey);
      const launchState = await this.getLaunchState(launchPubkey);
      
      const tx = await this.program.methods
        .depositTokens(new BN(amount))
        .accounts({
          authority: this.provider.wallet.publicKey,
          launch: new PublicKey(launchPubkey),
          from: fromTokenAccount,
          vault: launchState.vault,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Failed to deposit tokens:', error);
      throw error;
    }
  }

  async withdrawTokens(launchPubkey, amount) {
    try {
      const toTokenAccount = await this.getTokenAccount(this.provider.wallet.publicKey);
      const launchState = await this.getLaunchState(launchPubkey);
      
      const tx = await this.program.methods
        .withdrawTokens(new BN(amount))
        .accounts({
          authority: this.provider.wallet.publicKey,
          launch: new PublicKey(launchPubkey),
          vault: launchState.vault,
          to: toTokenAccount,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Failed to withdraw tokens:', error);
      throw error;
    }
  }

  async enableTrading(launchPubkey) {
    try {
      const tx = await this.program.methods
        .enableTrading()
        .accounts({
          authority: this.provider.wallet.publicKey,
          launch: new PublicKey(launchPubkey),
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Failed to enable trading:', error);
      throw error;
    }
  }

  async disableTrading(launchPubkey) {
    try {
      const tx = await this.program.methods
        .disableTrading()
        .accounts({
          authority: this.provider.wallet.publicKey,
          launch: new PublicKey(launchPubkey),
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Failed to disable trading:', error);
      throw error;
    }
  }

  async executeTrade(launchPubkey, amount) {
    try {
      const launchState = await this.getLaunchState(launchPubkey);
      const userTokenAccount = await this.getTokenAccount(this.provider.wallet.publicKey);
      
      const [tradeStatePubkey] = await PublicKey.findProgramAddress(
        [
          Buffer.from('trade'),
          this.provider.wallet.publicKey.toBuffer(),
          new PublicKey(launchPubkey).toBuffer(),
        ],
        PROGRAM_ID
      );

      const tx = await this.program.methods
        .executeTrade(new BN(amount))
        .accounts({
          launch: new PublicKey(launchPubkey),
          tradeState: tradeStatePubkey,
          vault: launchState.vault,
          userToken: userTokenAccount,
          authority: this.provider.wallet.publicKey,
          tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      return tx;
    } catch (error) {
      console.error('Failed to execute trade:', error);
      throw error;
    }
  }

  async getTokenAccount(owner) {
    // Simplified token account lookup - in production, use proper ATA logic
    const tokenAccounts = await this.connection.getTokenAccountsByOwner(owner);
    if (tokenAccounts.value.length > 0) {
      return new PublicKey(tokenAccounts.value[0].pubkey);
    }
    throw new Error('No token account found');
  }

  async getLaunchesByAuthority(authorityPubkey) {
    // This would require implementing proper filtering logic
    // For now, return empty array
    return [];
  }
}

export default new SolanaClient();