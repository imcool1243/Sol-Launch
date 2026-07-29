import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import solanaClient from '../utils/solanaClient';
import './VaultManagement.css';

function VaultManagement({ launch, onBack }) {
  const { connected } = useWallet();
  const [vaultBalance, setVaultBalance] = useState(0);
  const [depositAmount, setDepositAmount] = useState('');
  const [withdrawAmount, setWithdrawAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  useEffect(() => {
    if (launch) {
      loadVaultInfo();
    }
  }, [launch]);

  const loadVaultInfo = async () => {
    try {
      // In production, this would fetch actual vault balance
      setVaultBalance(launch.totalSupply - launch.totalTraded);
    } catch (err) {
      console.error('Failed to load vault info:', err);
    }
  };

  const handleDeposit = async (e) => {
    e.preventDefault();
    
    if (!connected) {
      setError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const amount = Number(depositAmount);
      if (amount <= 0) {
        throw new Error('Amount must be greater than 0');
      }

      const tx = await solanaClient.depositTokens(launch.tokenMint, amount);
      setSuccess(`Tokens deposited successfully! Signature: ${tx}`);
      setDepositAmount('');
      await loadVaultInfo();
    } catch (err) {
      setError('Failed to deposit tokens: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleWithdraw = async (e) => {
    e.preventDefault();
    
    if (!connected) {
      setError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const amount = Number(withdrawAmount);
      if (amount <= 0) {
        throw new Error('Amount must be greater than 0');
      }
      if (amount > vaultBalance) {
        throw new Error('Insufficient vault balance');
      }

      const tx = await solanaClient.withdrawTokens(launch.tokenMint, amount);
      setSuccess(`Tokens withdrawn successfully! Signature: ${tx}`);
      setWithdrawAmount('');
      await loadVaultInfo();
    } catch (err) {
      setError('Failed to withdraw tokens: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  if (!launch) {
    return (
      <div className="vault-management">
        <div className="card">
          <h2>Vault Management</h2>
          <p>No launch selected. Please select a launch from the dashboard.</p>
          <button className="button" onClick={onBack}>
            Back to Dashboard
          </button>
        </div>
      </div>
    );
  }

  if (!connected) {
    return (
      <div className="vault-management">
        <div className="card">
          <h2>Vault Management</h2>
          <p>Please connect your wallet to manage the vault.</p>
          <button className="button" onClick={onBack}>
            Back to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="vault-management">
      <div className="card">
        <div className="form-header">
          <h2>Vault Management</h2>
          <button className="back-button" onClick={onBack}>
            ← Back
          </button>
        </div>

        {error && <div className="error">{error}</div>}
        {success && <div className="success">{success}</div>}

        <div className="vault-info">
          <div className="vault-stats">
            <div className="stat">
              <span className="stat-label">Token Mint:</span>
              <span className="stat-value">{launch.tokenMint.slice(0, 8)}...{launch.tokenMint.slice(-8)}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Vault Balance:</span>
              <span className="stat-value">{vaultBalance.toLocaleString()}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Total Supply:</span>
              <span className="stat-value">{launch.totalSupply.toLocaleString()}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Total Traded:</span>
              <span className="stat-value">{launch.totalTraded.toLocaleString()}</span>
            </div>
          </div>
        </div>

        <div className="vault-actions">
          <div className="action-section">
            <h3>Deposit Tokens</h3>
            <form onSubmit={handleDeposit}>
              <div className="form-group">
                <label className="label">Amount</label>
                <input
                  className="input"
                  type="number"
                  value={depositAmount}
                  onChange={(e) => setDepositAmount(e.target.value)}
                  placeholder="Enter amount to deposit"
                  min="1"
                  required
                />
              </div>
              <button 
                className="button" 
                type="submit" 
                disabled={loading}
                style={{ width: '100%' }}
              >
                {loading ? 'Depositing...' : 'Deposit Tokens'}
              </button>
            </form>
          </div>

          <div className="action-section">
            <h3>Withdraw Tokens</h3>
            <form onSubmit={handleWithdraw}>
              <div className="form-group">
                <label className="label">Amount</label>
                <input
                  className="input"
                  type="number"
                  value={withdrawAmount}
                  onChange={(e) => setWithdrawAmount(e.target.value)}
                  placeholder="Enter amount to withdraw"
                  min="1"
                  max={vaultBalance}
                  required
                />
              </div>
              <button 
                className="button" 
                type="submit" 
                disabled={loading}
                style={{ width: '100%' }}
              >
                {loading ? 'Withdrawing...' : 'Withdraw Tokens'}
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

export default VaultManagement;