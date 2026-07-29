import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import solanaClient from '../utils/solanaClient';
import './TradingInterface.css';

function TradingInterface({ launch, onBack }) {
  const { connected, publicKey } = useWallet();
  const [tradeAmount, setTradeAmount] = useState('');
  const [userBalance, setUserBalance] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  useEffect(() => {
    if (launch && publicKey) {
      loadUserInfo();
    }
  }, [launch, publicKey]);

  const loadUserInfo = async () => {
    try {
      // In production, this would fetch actual user trade state
      setUserBalance(0);
    } catch (err) {
      console.error('Failed to load user info:', err);
    }
  };

  const handleTrade = async (e) => {
    e.preventDefault();
    
    if (!connected) {
      setError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const amount = Number(tradeAmount);
      if (amount <= 0) {
        throw new Error('Amount must be greater than 0');
      }
      if (amount > launch.maxBuy) {
        throw new Error(`Amount exceeds max buy limit of ${launch.maxBuy}`);
      }
      if (userBalance + amount > launch.maxWallet) {
        throw new Error(`Amount would exceed max wallet limit of ${launch.maxWallet}`);
      }

      const tx = await solanaClient.executeTrade(launch.tokenMint, amount);
      setSuccess(`Trade executed successfully! Signature: ${tx}`);
      setTradeAmount('');
      await loadUserInfo();
    } catch (err) {
      setError('Failed to execute trade: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleEnableTrading = async () => {
    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const tx = await solanaClient.enableTrading(launch.tokenMint);
      setSuccess(`Trading enabled successfully! Signature: ${tx}`);
    } catch (err) {
      setError('Failed to enable trading: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleDisableTrading = async () => {
    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const tx = await solanaClient.disableTrading(launch.tokenMint);
      setSuccess(`Trading disabled successfully! Signature: ${tx}`);
    } catch (err) {
      setError('Failed to disable trading: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  if (!launch) {
    return (
      <div className="trading-interface">
        <div className="card">
          <h2>Trading Interface</h2>
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
      <div className="trading-interface">
        <div className="card">
          <h2>Trading Interface</h2>
          <p>Please connect your wallet to trade.</p>
          <button className="button" onClick={onBack}>
            Back to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="trading-interface">
      <div className="card">
        <div className="form-header">
          <h2>Trading Interface</h2>
          <button className="back-button" onClick={onBack}>
            ← Back
          </button>
        </div>

        {error && <div className="error">{error}</div>}
        {success && <div className="success">{success}</div>}

        <div className="trading-status">
          <div className="status-header">
            <h3>Launch Status</h3>
            <div className={`status-badge ${launch.tradingEnabled ? 'active' : 'inactive'}`}>
              {launch.tradingEnabled ? 'Trading Active' : 'Trading Inactive'}
            </div>
          </div>

          <div className="launch-info">
            <div className="info-row">
              <span className="info-label">Phase:</span>
              <span className="info-value">
                {launch.phase === 1 ? 'Ready' : launch.phase === 2 ? 'Active' : 'Paused'}
              </span>
            </div>
            <div className="info-row">
              <span className="info-label">Max Buy:</span>
              <span className="info-value">{launch.maxBuy.toLocaleString()}</span>
            </div>
            <div className="info-row">
              <span className="info-label">Max Wallet:</span>
              <span className="info-value">{launch.maxWallet.toLocaleString()}</span>
            </div>
            <div className="info-row">
              <span className="info-label">Cooldown:</span>
              <span className="info-value">{launch.cooldownSeconds}s</span>
            </div>
            <div className="info-row">
              <span className="info-label">Your Balance:</span>
              <span className="info-value">{userBalance.toLocaleString()}</span>
            </div>
          </div>

          <div className="trading-controls">
            {!launch.tradingEnabled ? (
              <button 
                className="button enable-button" 
                onClick={handleEnableTrading}
                disabled={loading}
              >
                {loading ? 'Enabling...' : 'Enable Trading'}
              </button>
            ) : (
              <button 
                className="button disable-button" 
                onClick={handleDisableTrading}
                disabled={loading}
              >
                {loading ? 'Disabling...' : 'Disable Trading'}
              </button>
            )}
          </div>
        </div>

        {launch.tradingEnabled && (
          <div className="trade-form">
            <h3>Execute Trade</h3>
            <form onSubmit={handleTrade}>
              <div className="form-group">
                <label className="label">Trade Amount</label>
                <input
                  className="input"
                  type="number"
                  value={tradeAmount}
                  onChange={(e) => setTradeAmount(e.target.value)}
                  placeholder="Enter amount to trade"
                  min="1"
                  max={Math.min(launch.maxBuy, launch.maxWallet - userBalance)}
                  required
                />
                <div className="trade-limits">
                  <span>Max per trade: {launch.maxBuy.toLocaleString()}</span>
                  <span>Max wallet: {launch.maxWallet.toLocaleString()}</span>
                </div>
              </div>
              <button 
                className="button" 
                type="submit" 
                disabled={loading}
                style={{ width: '100%' }}
              >
                {loading ? 'Trading...' : 'Execute Trade'}
              </button>
            </form>
          </div>
        )}

        {launch.sniperProtectionEnabled && (
          <div className="security-info">
            <h4>🛡️ Security Features Active</h4>
            <ul>
              <li>Sniper Protection: Enabled</li>
              <li>Min Trading Duration: {launch.minTradingDuration}s</li>
              <li>Anti-Bot Rate Limiting: Active</li>
            </ul>
          </div>
        )}
      </div>
    </div>
  );
}

export default TradingInterface;