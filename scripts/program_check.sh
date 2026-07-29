#!/bin/bash

# Sol-Launch Program Deployment Check Script
# Verifies smart contract deployment for mainnet

set -e

echo "=== PROGRAM DEPLOYMENT CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PROGRAM_ERRORS=0

# Expected program ID
EXPECTED_PROGRAM_ID="2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"

# Check if Anchor is installed
echo "1. Checking Anchor CLI installation..."
if command -v anchor &> /dev/null; then
    echo -e "${GREEN}✓${NC} Anchor CLI installed"
    anchor --version
else
    echo -e "${RED}✗${NC} Anchor CLI not installed"
    echo "   Run: npm install -g @coral-xyz/anchor-cli"
    PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
fi
echo ""

# Check program build
echo "2. Checking program build..."
if [ -f "target/deploy/sol_launch.so" ]; then
    echo -e "${GREEN}✓${NC} Program binary exists: target/deploy/sol_launch.so"
    FILE_SIZE=$(stat -c%s "target/deploy/sol_launch.so")
    echo "   File size: $FILE_SIZE bytes"
else
    echo -e "${RED}✗${NC} Program binary not found"
    echo "   Run: anchor build"
    PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
fi
echo ""

# Check program keypair
echo "3. Checking program keypair..."
if [ -f "target/deploy/sol_launch-keypair.json" ]; then
    echo -e "${GREEN}✓${NC} Program keypair exists"
    PROGRAM_KEYPAIR_ADDRESS=$(solana-keygen pubkey target/deploy/sol_launch-keypair.json)
    echo "   Program keypair address: $PROGRAM_KEYPAIR_ADDRESS"
else
    echo -e "${RED}✗${NC} Program keypair not found"
    echo "   Run: anchor keys list"
    PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
fi
echo ""

# Check IDL file
echo "4. Checking IDL file..."
if [ -f "target/idl/sol_launch.json" ]; then
    echo -e "${GREEN}✓${NC} IDL file exists: target/idl/sol_launch.json"
else
    echo -e "${RED}✗${NC} IDL file not found"
    echo "   Run: anchor build"
    PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
fi
echo ""

# Check program ID consistency
echo "5. Checking program ID consistency..."
if [ -f "target/deploy/sol_launch-keypair.json" ]; then
    CURRENT_PROGRAM_ID=$(solana-keygen pubkey target/deploy/sol_launch-keypair.json)
    if [ "$CURRENT_PROGRAM_ID" == "$EXPECTED_PROGRAM_ID" ]; then
        echo -e "${GREEN}✓${NC} Program ID matches expected: $EXPECTED_PROGRAM_ID"
    else
        echo -e "${RED}✗${NC} Program ID mismatch"
        echo "   Expected: $EXPECTED_PROGRAM_ID"
        echo "   Current:  $CURRENT_PROGRAM_ID"
        PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
    fi
fi
echo ""

# Check if program is deployed (on current network)
echo "6. Checking if program is deployed on network..."
CURRENT_NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [ -f "target/deploy/sol_launch-keypair.json" ]; then
    PROGRAM_ID=$(solana-keygen pubkey target/deploy/sol_launch-keypair.json)
    if solana program show "$PROGRAM_ID" &> /dev/null; then
        echo -e "${GREEN}✓${NC} Program is deployed on network: $CURRENT_NETWORK"
        PROGRAM_INFO=$(solana program show "$PROGRAM_ID")
        echo "   Program details:"
        echo "$PROGRAM_INFO" | while IFS= read -r line; do
            echo "   $line"
        done
    else
        echo -e "${YELLOW}⚠${NC} Program not yet deployed on network: $CURRENT_NETWORK"
        echo "   Run: anchor deploy --provider-cluster mainnet-beta"
    fi
fi
echo ""

# Check program authority
echo "7. Checking program authority..."
if [ -f "target/deploy/sol_launch-keypair.json" ]; then
    PROGRAM_ID=$(solana-keygen pubkey target/deploy/sol_launch-keypair.json)
    if solana program show "$PROGRAM_ID" &> /dev/null; then
        AUTHORITY=$(solana program show "$PROGRAM_ID" | grep "Authority" | awk '{print $2}')
        WALLET_ADDRESS=$(solana address)
        if [ "$AUTHORITY" == "$WALLET_ADDRESS" ]; then
            echo -e "${GREEN}✓${NC} Program authority matches current wallet"
        else
            echo -e "${YELLOW}⚠${NC} Program authority differs from current wallet"
            echo "   Authority: $AUTHORITY"
            echo "   Current wallet: $WALLET_ADDRESS"
        fi
    fi
fi
echo ""

# Check program size (BPF upgradeable size limit)
echo "8. Checking program size..."
if [ -f "target/deploy/sol_launch.so" ]; then
    FILE_SIZE=$(stat -c%s "target/deploy/sol_launch.so")
    MAX_SIZE=200000 # 200KB limit for BPF upgradeable programs
    if [ $FILE_SIZE -lt $MAX_SIZE ]; then
        echo -e "${GREEN}✓${NC} Program size within limits: $FILE_SIZE bytes (max: $MAX_SIZE)"
    else
        echo -e "${RED}✗${NC} Program size exceeds limits: $FILE_SIZE bytes (max: $MAX_SIZE)"
        PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
    fi
fi
echo ""

# Check for any compilation errors
echo "9. Running build verification..."
if anchor build --skip-local-validator &> /dev/null; then
    echo -e "${GREEN}✓${NC} Program builds successfully"
else
    echo -e "${RED}✗${NC} Program build failed"
    echo "   Check build errors above"
    PROGRAM_ERRORS=$((PROGRAM_ERRORS + 1))
fi
echo ""

# Final summary
echo "=== PROGRAM DEPLOYMENT CHECK SUMMARY ==="
if [ $PROGRAM_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Program deployment is ready"
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $PROGRAM_ERRORS program deployment issues found"
    exit 1
fi