import React, { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import solanaClient from '../utils/solanaClient';
import './LaunchForm.css';

function LaunchForm({ onBack }) {
  const { connected, publicKey } = useWallet();
  const [formData, setFormData] = useState({
    tokenMint: '',
    maxBuy: 1000,
    maxWallet: 5000,
    cooldownSeconds: 60,
    totalSupply: 1000000,
    sniperProtectionEnabled: true,
    minTradingDuration: 300,
    // Enhanced anti-sniper features
    walletBlacklistEnabled: false,
    progressiveLimitsEnabled: true,
    initialMaxBuy: 500,
    initialMaxWallet: 2500,
    limitIncreaseInterval: 300, // 5 minutes
    limitIncreaseMultiplier: 1, // 1x increase per interval
    antiScamEnabled: true,
    maxTradesPerUser: 20,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : 
               type === 'number' ? Number(value) : value
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    
    if (!connected) {
      setError('Please connect your wallet first');
      return;
    }

    try {
      setLoading(true);
      setError(null);
      setSuccess(null);

      const result = await solanaClient.initializeLaunch(formData);
      
      setSuccess(`Launch created successfully! Launch pubkey: ${result.launchPubkey}`);
      // Reset form
      setFormData({
        tokenMint: '',
        maxBuy: 1000,
        maxWallet: 5000,
        cooldownSeconds: 60,
        totalSupply: 1000000,
        sniperProtectionEnabled: true,
        minTradingDuration: 300,
        walletBlacklistEnabled: false,
        progressiveLimitsEnabled: true,
        initialMaxBuy: 500,
        initialMaxWallet: 2500,
        limitIncreaseInterval: 300,
        limitIncreaseMultiplier: 1,
        antiScamEnabled: true,
        maxTradesPerUser: 20,
      });
    } catch (err) {
      setError('Failed to create launch: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="launch-form">
        <div className="card">
          <h2>Create Secure Launch</h2>
          <p>Please connect your wallet to create a secure token launch.</p>
          <button className="button" onClick={onBack}>
            Back to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="launch-form">
      <div className="card">
        <div className="form-header">
          <h2>Create Secure Launch</h2>
          <button className="back-button" onClick={onBack}>
            ← Back
          </button>
        </div>

        {error && <div className="error">{error}</div>}
        {success && <div className="success">{success}</div>}

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="label">Token Mint Address</label>
            <input
              className="input"
              type="text"
              name="tokenMint"
              value={formData.tokenMint}
              onChange={handleChange}
              placeholder="Enter token mint address"
              required
            />
          </div>

          <div className="form-row">
            <div className="form-group">
              <label className="label">Max Buy Amount</label>
              <input
                className="input"
                type="number"
                name="maxBuy"
                value={formData.maxBuy}
                onChange={handleChange}
                min="1"
                required
              />
            </div>

            <div className="form-group">
              <label className="label">Max Wallet Amount</label>
              <input
                className="input"
                type="number"
                name="maxWallet"
                value={formData.maxWallet}
                onChange={handleChange}
                min="1"
                required
              />
            </div>
          </div>

          <div className="form-row">
            <div className="form-group">
              <label className="label">Cooldown (seconds)</label>
              <input
                className="input"
                type="number"
                name="cooldownSeconds"
                value={formData.cooldownSeconds}
                onChange={handleChange}
                min="1"
                required
              />
            </div>

            <div className="form-group">
              <label className="label">Total Supply</label>
              <input
                className="input"
                type="number"
                name="totalSupply"
                value={formData.totalSupply}
                onChange={handleChange}
                min="1"
                required
              />
            </div>
          </div>

          <div className="form-group">
            <label className="label">Min Trading Duration (seconds)</label>
            <input
              className="input"
              type="number"
              name="minTradingDuration"
              value={formData.minTradingDuration}
              onChange={handleChange}
              min="0"
              required
            />
          </div>

          <div className="form-group checkbox-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                name="sniperProtectionEnabled"
                checked={formData.sniperProtectionEnabled}
                onChange={handleChange}
              />
              <span>Enable Sniper Protection</span>
            </label>
            <p className="checkbox-description">
              Prevents early trading disable attacks by enforcing minimum trading duration
            </p>
          </div>

          {/* Enhanced Anti-Sniper Features */}
          <div className="section-divider"></div>
          <h3>Advanced Anti-Sniper Features</h3>

          <div className="form-group checkbox-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                name="progressiveLimitsEnabled"
                checked={formData.progressiveLimitsEnabled}
                onChange={handleChange}
              />
              <span>Enable Progressive Limits</span>
            </label>
            <p className="checkbox-description">
              Gradually increase buy/wallet limits over time to discourage early sniping
            </p>
          </div>

          {formData.progressiveLimitsEnabled && (
            <>
              <div className="form-row">
                <div className="form-group">
                  <label className="label">Initial Max Buy</label>
                  <input
                    className="input"
                    type="number"
                    name="initialMaxBuy"
                    value={formData.initialMaxBuy}
                    onChange={handleChange}
                    min="1"
                    max={formData.maxBuy}
                  />
                </div>

                <div className="form-group">
                  <label className="label">Initial Max Wallet</label>
                  <input
                    className="input"
                    type="number"
                    name="initialMaxWallet"
                    value={formData.initialMaxWallet}
                    onChange={handleChange}
                    min="1"
                    max={formData.maxWallet}
                  />
                </div>
              </div>

              <div className="form-row">
                <div className="form-group">
                  <label className="label">Limit Increase Interval (seconds)</label>
                  <input
                    className="input"
                    type="number"
                    name="limitIncreaseInterval"
                    value={formData.limitIncreaseInterval}
                    onChange={handleChange}
                    min="1"
                  />
                </div>

                <div className="form-group">
                  <label className="label">Limit Increase Multiplier</label>
                  <input
                    className="input"
                    type="number"
                    name="limitIncreaseMultiplier"
                    value={formData.limitIncreaseMultiplier}
                    onChange={handleChange}
                    min="1"
                  />
                </div>
              </div>
            </>
          )}

          <div className="form-group checkbox-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                name="antiScamEnabled"
                checked={formData.antiScamEnabled}
                onChange={handleChange}
              />
              <span>Enable Anti-Scam Protection</span>
            </label>
            <p className="checkbox-description">
              Limit maximum trades per user to prevent bot accumulation
            </p>
          </div>

          {formData.antiScamEnabled && (
            <div className="form-group">
              <label className="label">Max Trades Per User</label>
              <input
                className="input"
                type="number"
                name="maxTradesPerUser"
                value={formData.maxTradesPerUser}
                onChange={handleChange}
                min="1"
              />
            </div>
          )}

          <div className="form-group checkbox-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                name="walletBlacklistEnabled"
                checked={formData.walletBlacklistEnabled}
                onChange={handleChange}
              />
              <span>Enable Wallet Blacklist</span>
            </label>
            <p className="checkbox-description">
              Allows blocking specific wallets from trading (for known bots/scammers)
            </p>
          </div>

          <button 
            className="button" 
            type="submit" 
            disabled={loading}
            style={{ width: '100%' }}
          >
            {loading ? 'Creating Launch...' : 'Create Secure Launch'}
          </button>
        </form>
      </div>
    </div>
  );
}

export default LaunchForm;