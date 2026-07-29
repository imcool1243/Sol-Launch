import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import solanaClient from '../utils/solanaClient';
import './LaunchDashboard.css';

function LaunchDashboard({ onLaunchSelect }) {
  const { connected, publicKey } = useWallet();
  const [launch, setLaunch] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (connected && publicKey) {
      loadLaunchStatus();
    }
  }, [connected, publicKey]);

  const loadLaunchStatus = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // In production, this would fetch the actual launch state
      // For now, we'll use mock data with enhanced features
      const mockLaunch = {
        authority: publicKey.toString(),
        tradingEnabled: false,
        phase: 1, // READY
        maxBuy: 1000,
        maxWallet: 5000,
        cooldownSeconds: 60,
        tokenMint: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
        vault: '8zK8j2M4J5L5v4B9t3N7qR9sP5t5M5v4B9t3N7qR9sP5t5',
        totalSupply: 1000000,
        totalTraded: 0,
        sniperProtectionEnabled: true,
        minTradingDuration: 300,
        // Enhanced features
        walletBlacklistEnabled: false,
        progressiveLimitsEnabled: true,
        initialMaxBuy: 500,
        initialMaxWallet: 2500,
        limitIncreaseInterval: 300,
        limitIncreaseMultiplier: 1,
        antiScamEnabled: true,
        maxTradesPerUser: 20,
        totalTraders: 0,
      };
      
      setLaunch(mockLaunch);
      onLaunchSelect(mockLaunch);
    } catch (err) {
      setError('Failed to load launch status: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  const getPhaseName = (phase) => {
    switch (phase) {
      case 1: return 'Ready';
      case 2: return 'Active';
      case 3: return 'Paused';
      default: return 'Unknown';
    }
  };

  const getStatusColor = (phase, tradingEnabled) => {
    if (phase === 2 && tradingEnabled) return '#10b981';
    if (phase === 3) return '#f59e0b';
    return '#6b7280';
  };

  if (!connected) {
    return (
      <div className="launch-dashboard">
        <div className="card">
          <h2>Secure Token Launch Platform</h2>
          <p>Please connect your wallet to view and manage the secure token launch.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="launch-dashboard">
        <div className="loading">Loading launch status...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="launch-dashboard">
        <div className="error">{error}</div>
      </div>
    );
  }

  return (
    <div className="launch-dashboard">
      <div className="dashboard-header">
        <h2>Launch Status</h2>
        <button className="button" onClick={loadLaunchStatus}>
          Refresh
        </button>
      </div>

      {launch ? (
        <div className="launch-status-card">
          <div className="launch-header">
            <h3>Single Token Launch</h3>
            <div 
              className="status-badge"
              style={{ backgroundColor: getStatusColor(launch.phase, launch.tradingEnabled) }}
            >
              {getPhaseName(launch.phase)} - {launch.tradingEnabled ? 'Trading Active' : 'Trading Inactive'}
            </div>
          </div>
          
          <div className="launch-stats">
            <div className="stat-group">
              <h4>Trading Configuration</h4>
              <div className="stat">
                <span className="stat-label">Max Buy:</span>
                <span className="stat-value">{launch.maxBuy.toLocaleString()}</span>
              </div>
              <div className="stat">
                <span className="stat-label">Max Wallet:</span>
                <span className="stat-value">{launch.maxWallet.toLocaleString()}</span>
              </div>
              <div className="stat">
                <span className="stat-label">Cooldown:</span>
                <span className="stat-value">{launch.cooldownSeconds}s</span>
              </div>
            </div>

            <div className="stat-group">
              <h4>Token Information</h4>
              <div className="stat">
                <span className="stat-label">Total Supply:</span>
                <span className="stat-value">{launch.totalSupply.toLocaleString()}</span>
              </div>
              <div className="stat">
                <span className="stat-label">Total Traded:</span>
                <span className="stat-value">{launch.totalTraded.toLocaleString()}</span>
              </div>
              <div className="stat">
                <span className="stat-label">Total Traders:</span>
                <span className="stat-value">{launch.totalTraders.toLocaleString()}</span>
              </div>
            </div>

            <div className="stat-group">
              <h4>Anti-Sniper Protection</h4>
              <div className="stat">
                <span className="stat-label">Sniper Protection:</span>
                <span className="stat-value">
                  {launch.sniperProtectionEnabled ? '✅ Enabled' : '❌ Disabled'}
                </span>
              </div>
              <div className="stat">
                <span className="stat-label">Min Trading Duration:</span>
                <span className="stat-value">{launch.minTradingDuration}s</span>
              </div>
            </div>

            <div className="stat-group">
              <h4>Advanced Features</h4>
              <div className="stat">
                <span className="stat-label">Progressive Limits:</span>
                <span className="stat-value">
                  {launch.progressiveLimitsEnabled ? '✅ Enabled' : '❌ Disabled'}
                </span>
              </div>
              {launch.progressiveLimitsEnabled && (
                <>
                  <div className="stat">
                    <span className="stat-label">Initial Max Buy:</span>
                    <span className="stat-value">{launch.initialMaxBuy.toLocaleString()}</span>
                  </div>
                  <div className="stat">
                    <span className="stat-label">Initial Max Wallet:</span>
                    <span className="stat-value">{launch.initialMaxWallet.toLocaleString()}</span>
                  </div>
                </>
              )}
              <div className="stat">
                <span className="stat-label">Anti-Scam Protection:</span>
                <span className="stat-value">
                  {launch.antiScamEnabled ? '✅ Enabled' : '❌ Disabled'}
                </span>
              </div>
              {launch.antiScamEnabled && (
                <div className="stat">
                  <span className="stat-label">Max Trades Per User:</span>
                  <span className="stat-value">{launch.maxTradesPerUser}</span>
                </div>
              )}
              <div className="stat">
                <span className="stat-label">Wallet Blacklist:</span>
                <span className="stat-value">
                  {launch.walletBlacklistEnabled ? '✅ Enabled' : '❌ Disabled'}
                </span>
              </div>
            </div>
          </div>
        </div>
      ) : (
        <div className="card">
          <p>No launch configured. Configure your secure token launch to get started!</p>
        </div>
      )}
    </div>
  );
}

export default LaunchDashboard;