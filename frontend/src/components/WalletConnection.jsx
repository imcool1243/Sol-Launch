import React from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletModalButton } from '@solana/wallet-adapter-react-ui';
import './WalletConnection.css';

function WalletConnection() {
  const { publicKey, connected, disconnect } = useWallet();

  const handleDisconnect = async () => {
    try {
      await disconnect();
    } catch (error) {
      console.error('Failed to disconnect wallet:', error);
    }
  };

  if (connected && publicKey) {
    return (
      <div className="wallet-connection">
        <div className="wallet-info">
          <span className="wallet-label">Connected:</span>
          <span className="wallet-address">
            {publicKey.toString().slice(0, 4)}...{publicKey.toString().slice(-4)}
          </span>
        </div>
        <button className="disconnect-button" onClick={handleDisconnect}>
          Disconnect
        </button>
      </div>
    );
  }

  return (
    <div className="wallet-connection">
      <WalletModalButton className="connect-button">
        Connect Wallet
      </WalletModalButton>
    </div>
  );
}

export default WalletConnection;