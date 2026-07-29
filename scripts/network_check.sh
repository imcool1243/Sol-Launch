#!/bin/bash

# Sol-Launch Network Check Script
# Verifies network configuration for mainnet deployment

set -e

echo "=== NETWORK CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

NETWORK_ERRORS=0

# Check Solana CLI
echo "1. Checking Solana CLI installation..."
if command -v solana &> /dev/null; then
    echo -e "${GREEN}✓${NC} Solana CLI installed"
else
    echo -e "${RED}✗${NC} Solana CLI not installed"
    NETWORK_ERRORS=$((NETWORK_ERRORS + 1))
fi
echo ""

# Check current network
echo "2. Checking network configuration..."
CURRENT_NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
echo "   Current network: $CURRENT_NETWORK"

if [[ "$CURRENT_NETWORK" == *"mainnet"* ]]; then
    echo -e "${GREEN}✓${NC} Configured for mainnet"
else
    echo -e "${RED}✗${NC} Not configured for mainnet"
    echo "   Run: solana config set --url mainnet-beta"
    NETWORK_ERRORS=$((NETWORK_ERRORS + 1))
fi
echo ""

# Check network connectivity
echo "3. Checking network connectivity..."
if solana cluster-version &> /dev/null; then
    echo -e "${GREEN}✓${NC} Network is reachable"
    solana cluster-version
else
    echo -e "${RED}✗${NC} Network is not reachable"
    NETWORK_ERRORS=$((NETWORK_ERRORS + 1))
fi
echo ""

# Check for mainnet-beta specifically
echo "4. Verifying mainnet-beta endpoint..."
if [[ "$CURRENT_NETWORK" == *"mainnet-beta"* ]]; then
    echo -e "${GREEN}✓${NC} Using official mainnet-beta endpoint"
else
    echo -e "${YELLOW}⚠${NC} Using custom RPC endpoint: $CURRENT_NETWORK"
    echo "   Ensure this endpoint is reliable for mainnet"
fi
echo ""

# Check network health
echo "5. Checking network health..."
if solana gossip &> /dev/null; then
    echo -e "${GREEN}✓${NC} Network gossip is working"
else
    echo -e "${YELLOW}⚠${NC} Network gossip check failed (may be temporary)"
fi
echo ""

# Check current slot
echo "6. Checking current slot..."
CURRENT_SLOT=$(solana slot)
if [ -n "$CURRENT_SLOT" ]; then
    echo -e "${GREEN}✓${NC} Current slot: $CURRENT_SLOT"
else
    echo -e "${RED}✗${NC} Cannot get current slot"
    NETWORK_ERRORS=$((NETWORK_ERRORS + 1))
fi
echo ""

# Check for sufficient RPC endpoint
echo "7. Checking RPC endpoint performance..."
START_TIME=$(date +%s%N)
solana get-block-height &> /dev/null
END_TIME=$(date +%s%N)
DURATION=$(( ($END_TIME - $START_TIME) / 1000000 ))

if [ $DURATION -lt 5000 ]; then
    echo -e "${GREEN}✓${NC} RPC endpoint responding quickly (${DURATION}ms)"
else
    echo -e "${YELLOW}⚠${NC} RPC endpoint slow (${DURATION}ms) - consider alternative endpoint"
fi
echo ""

# Check network congestion
echo "8. Checking network congestion..."
if solana transaction-count &> /dev/null; then
    echo -e "${GREEN}✓${NC} Can read transaction count"
else
    echo -e "${YELLOW}⚠${NC} Cannot read transaction count (network may be congested)"
fi
echo ""

# Warn about devnet/testnet
echo "9. Checking for test network usage..."
if [[ "$CURRENT_NETWORK" == *"devnet"* ]] || [[ "$CURRENT_NETWORK" == *"testnet"* ]]; then
    echo -e "${RED}✗${NC} Currently on test network: $CURRENT_NETWORK"
    echo "   This is NOT mainnet. Do not deploy to mainnet from test network."
    NETWORK_ERRORS=$((NETWORK_ERRORS + 1))
else
    echo -e "${GREEN}✓${NC} Not on test network"
fi
echo ""

# Final summary
echo "=== NETWORK CHECK SUMMARY ==="
if [ $NETWORK_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Network configuration is correct for mainnet"
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $NETWORK_ERRORS network issues found"
    exit 1
fi