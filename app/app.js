const connectButton = document.getElementById('connect-wallet');
const statusText = document.getElementById('wallet-status');

async function connectWallet() {
  if (!window.solana?.connect) {
    statusText.textContent = 'Install a Solana wallet like Phantom to continue.';
    return;
  }

  try {
    const response = await window.solana.connect();
    const address = response.publicKey?.toBase58?.() || 'wallet connected';
    statusText.textContent = `Connected: ${address}`;
  } catch (error) {
    statusText.textContent = 'Connection cancelled or unavailable.';
  }
}

connectButton?.addEventListener('click', connectWallet);
