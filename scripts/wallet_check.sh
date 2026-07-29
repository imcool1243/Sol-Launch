#!/bin/bash

# Sol-Launch Wallet Check Script
# Verifies wallet configuration for mainnet deployment

set -e

echo "=== WALLET CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

WALLET_ERRORS=0

# Check if Solana CLI is installed
echo "1. Checking Solana CLI installation..."
if command -v solana &> /dev/null; then
    echo -e "${GREEN}✓${NC} Solana CLI installed"
    solana --version
else
    echo -e "${RED}✗${NC} Solana CLI not installed"
    WALLET_ERRORS=$((WALLET_ERRORS + 1))
fi
echo ""

# Check current network
echo "2. Checking network configuration..."
NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [[ "$NETWORK" == *"mainnet"* ]]; then
    echo -e "${GREEN}✓${NC} Configured for mainnet: $NETWORK"
else
    echo -e "${RED}✗${NC} Not configured for mainnet: $NETWORK"
    echo "   Run: solana config set --url mainnet-beta"
    WALLET_ERRORS=$((WALLET_ERRORS + 1))
fi
echo ""

# Check wallet exists
echo "3. Checking wallet file..."
WALLET_FILE="$HOME/.config/solana/id.json"
if [ -f "$WALLET_FILE" ]; then
    echo -e "${GREEN}✓${NC} Wallet file exists: $WALLET_FILE"
    WALLET_ADDRESS=$(solana address)
    echo "   Wallet address: $WALLET_ADDRESS"
else
    echo -e "${RED}✗${NC} Wallet file not found: $WALLET_FILE"
    echo "   Run: solana-keygen new"
    WALLET_ERRORS=$((WALLET_ERRORS + 1))
fi
echo ""

# Check wallet balance
echo "4. Checking wallet balance..."
if [ -f "$WALLET_FILE" ]; then
    BALANCE=$(solana balance | awk '{print $1}')
    if [[ "$BALANCE" == *"SOL"* ]]; then
        BALANCE_NUM=$(echo $BALANCE | awk '{print $1}')
        echo -e "${GREEN}✓${NC} Wallet balance: $BALANCE"
        
        # Check if balance is sufficient for deployment (need at least 2 SOL)
        if (( $(echo "$BALANCE_NUM >= 2" | bc -l) )); then
            echo -e "${GREEN}✓${NC} Sufficient balance for deployment"
        else
            echo -e "${YELLOW}⚠${NC} Low balance for deployment (recommended: 5+ SOL)"
            WALLET_ERRORS=$((WALLET_ERRORS + 1))
        fi
    else
        echo -e "${RED}✗${NC} Cannot read wallet balance"
        WALLET_ERRORS=$((WALLET_ERRORS + 1))
    fi
fi
echo ""

# Check wallet permissions
echo "5. Checking wallet file permissions..."
if [ -f "$WALLET_FILE" ]; then
    PERMISSIONS=$(stat -c "%a" "$WALLET_FILE")
    if [ "$PERMISSIONS" == "600" ]; then
        echo -e "${GREEN}✓${NC} Wallet file has secure permissions: $PERMISSIONS"
    else
        echo -e "${YELLOW}⚠${NC} Wallet file permissions: $PERMISSIONS (recommended: 600)"
        echo "   Run: chmod 600 $WALLET_FILE"
        WALLET_ERRORS=$((WALLET_ERRORS + 1))
    fi
fi
echo ""

# Check if wallet is keypair
echo "6. Verifying wallet is keypair..."
if [ -f "$WALLET_FILE" ]; then
    if solana-keygen pubkey "$WALLET_FILE" &> /dev/null; then
        echo -e "${GREEN}✓${NC} Wallet is valid keypair"
    else
        echo -e "${RED}✗${NC} Wallet is not valid keypair"
        WALLET_ERRORS=$((WALLET_ERRORS + 1))
    fi
fi
echo ""

# Check for hardware wallet (optional but recommended)
echo "7. Checking for hardware wallet..."
if command -v solana-keygen pubkey usb://ledger &> /dev/null; then
    echo -e "${GREEN}✓${NC} Hardware wallet detected (Ledger)"
elif command -v solana-keygen pubkey usb://trezor &> /dev/null; then
    echo -e "${GREEN}✓${NC} Hardware wallet detected (Trezor)"
else
    echo -e "${YELLOW}⚠${NC} No hardware wallet detected (recommended for mainnet)"
fi
echo ""

# Final summary
echo "=== WALLET CHECK SUMMARY ==="
if [ $WALLET_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Wallet configuration is correct"
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $WALLET_ERRORS wallet issues found"
    exit 1
fi