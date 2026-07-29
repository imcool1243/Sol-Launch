import React, { useState, useEffect } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import WalletConnection from './components/WalletConnection';
import LaunchDashboard from './components/LaunchDashboard';
import LaunchForm from './components/LaunchForm';
import VaultManagement from './components/VaultManagement';
import TradingInterface from './components/TradingInterface';
import './App.css';

const wallets = [
  new PhantomWalletAdapter(),
];

function App() {
  const [currentView, setCurrentView] = useState('dashboard');
  const [selectedLaunch, setSelectedLaunch] = useState(null);

  const renderView = () => {
    switch (currentView) {
      case 'dashboard':
        return <LaunchDashboard onLaunchSelect={setSelectedLaunch} />;
      case 'create':
        return <LaunchForm onBack={() => setCurrentView('dashboard')} />;
      case 'vault':
        return <VaultManagement launch={selectedLaunch} onBack={() => setCurrentView('dashboard')} />;
      case 'trade':
        return <TradingInterface launch={selectedLaunch} onBack={() => setCurrentView('dashboard')} />;
      default:
        return <LaunchDashboard onLaunchSelect={setSelectedLaunch} />;
    }
  };

  return (
    <ConnectionProvider endpoint="https://api.devnet.solana.com">
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="app">
            <header className="app-header">
              <h1>Sol-Launch</h1>
              <p>Secure Single Token Launch Platform</p>
              <WalletConnection />
            </header>
            
            <nav className="app-nav">
              <button 
                className={`nav-button ${currentView === 'dashboard' ? 'active' : ''}`}
                onClick={() => setCurrentView('dashboard')}
              >
                Launch Status
              </button>
              <button 
                className={`nav-button ${currentView === 'create' ? 'active' : ''}`}
                onClick={() => setCurrentView('create')}
              >
                Configure Launch
              </button>
              {selectedLaunch && (
                <>
                  <button 
                    className={`nav-button ${currentView === 'vault' ? 'active' : ''}`}
                    onClick={() => setCurrentView('vault')}
                  >
                    Token Vault
                  </button>
                  <button 
                    className={`nav-button ${currentView === 'trade' ? 'active' : ''}`}
                    onClick={() => setCurrentView('trade')}
                  >
                    Secure Trading
                  </button>
                </>
              )}
            </nav>

            <main className="app-main">
              {renderView()}
            </main>
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;