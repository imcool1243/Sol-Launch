#!/bin/bash

# Sol-Launch Initialization Check Script
# Verifies Sol-Launch smart contract initialization for mainnet

set -e

echo "=== SOL-LAUNCH INITIALIZATION CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SOL_LAUNCH_ERRORS=0

# Expected program ID
EXPECTED_PROGRAM_ID="2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"

# Check program deployment
echo "1. Checking Sol-Launch program deployment..."
if [ -f "target/deploy/sol_launch-keypair.json" ]; then
    PROGRAM_ID=$(solana-keygen pubkey target/deploy/sol_launch-keypair.json)
    if [ "$PROGRAM_ID" == "$EXPECTED_PROGRAM_ID" ]; then
        echo -e "${GREEN}✓${NC} Program ID correct: $PROGRAM_ID"
    else
        echo -e "${RED}✗${NC} Program ID mismatch"
        SOL_LAUNCH_ERRORS=$((SOL_LAUNCH_ERRORS + 1))
    fi
else
    echo -e "${RED}✗${NC} Program keypair not found"
    SOL_LAUNCH_ERRORS=$((SOL_LAUNCH_ERRORS + 1))
fi
echo ""

# Check if program is deployed on current network
echo "2. Checking program deployment on network..."
CURRENT_NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
if solana program show "$EXPECTED_PROGRAM_ID" &> /dev/null; then
    echo -e "${GREEN}✓${NC} Program deployed on network: $CURRENT_NETWORK"
else
    echo -e "${YELLOW}⚠${NC} Program not yet deployed on network"
    echo "   Run: anchor deploy --provider-cluster mainnet-beta"
fi
echo ""

# Check anti-sniper parameters
echo "3. Checking anti-sniper parameter configuration..."
if [ -n "$MAX_BUY" ] && [ -n "$MAX_WALLET" ] && [ -n "$COOLDOWN_SECONDS" ]; then
    echo -e "${GREEN}✓${NC} Anti-sniper parameters configured:"
    echo "   Max Buy: $MAX_BUY"
    echo "   Max Wallet: $MAX_WALLET"
    echo "   Cooldown: $COOLDOWN_SECONDS seconds"
else
    echo -e "${YELLOW}⚠${NC} Anti-sniper parameters not fully configured"
    echo "   Set environment variables:"
    echo "   export MAX_BUY=\"1000\""
    echo "   export MAX_WALLET=\"5000\""
    echo "   export COOLDOWN_SECONDS=\"60\""
fi
echo ""

# Check progressive limits configuration
echo "4. Checking progressive limits configuration..."
if [ -n "$PROGRESSIVE_LIMITS_ENABLED" ]; then
    echo -e "${GREEN}✓${NC} Progressive limits configured: $PROGRESSIVE_LIMITS_ENABLED"
    if [ "$PROGRESSIVE_LIMITS_ENABLED" == "true" ]; then
        echo "   Initial Max Buy: ${INITIAL_MAX_BUY:-not set}"
        echo "   Initial Max Wallet: ${INITIAL_MAX_WALLET:-not set}"
        echo "   Limit Increase Interval: ${LIMIT_INCREASE_INTERVAL:-not set}"
    fi
else
    echo -e "${YELLOW}⚠${NC} Progressive limits not configured"
    echo "   Set: export PROGRESSIVE_LIMITS_ENABLED=\"true\""
fi
echo ""

# Check anti-scam configuration
echo "5. Checking anti-scam configuration..."
if [ -n "$ANTI_SCAM_ENABLED" ] && [ -n "$MAX_TRADES_PER_USER" ]; then
    echo -e "${GREEN}✓${NC} Anti-scam configuration:"
    echo "   Anti-Scam Enabled: $ANTI_SCAM_ENABLED"
    echo "   Max Trades Per User: $MAX_TRADES_PER_USER"
else
    echo -e "${YELLOW}⚠${NC} Anti-scam parameters not configured"
    echo "   Set: export ANTI_SCAM_ENABLED=\"true\""
    echo "   Set: export MAX_TRADES_PER_USER=\"20\""
fi
echo ""

# Check token mint for initialization
echo "6. Checking token mint for initialization..."
if [ -n "$TOKEN_MINT" ]; then
    echo -e "${GREEN}✓${NC} Token mint configured: $TOKEN_MINT"
    if spl-token display --mint "$TOKEN_MINT" &> /dev/null; then
        echo -e "${GREEN}✓${NC} Token mint exists on-chain"
    else
        echo -e "${YELLOW}⚠${NC} Token mint not yet on-chain"
    fi
else
    echo -e "${RED}✗${NC} Token mint not configured (required for initialization)"
    echo "   Set: export TOKEN_MINT=\"<your_token_mint_address>\""
    SOL_LAUNCH_ERRORS=$((SOL_LAUNCH_ERRORS + 1))
fi
echo ""

# Check total supply configuration
echo "7. Checking total supply configuration..."
if [ -n "$TOTAL_SUPPLY" ]; then
    echo -e "${GREEN}✓${NC} Total supply configured: $TOTAL_SUPPLY"
else
    echo -e "${YELLOW}⚠${NC} Total supply not configured"
    echo "   Set: export TOTAL_SUPPLY=\"1000000\""
fi
echo ""

# Check wallet authority
echo "8. Checking wallet authority..."
WALLET_ADDRESS=$(solana address)
echo "   Current wallet: $WALLET_ADDRESS"
if [ -n "$AUTHORITY_WALLET" ]; then
    echo "   Authority wallet: $AUTHORITY_WALLET"
    if [ "$WALLET_ADDRESS" == "$AUTHORITY_WALLET" ]; then
        echo -e "${GREEN}✓${NC} Current wallet matches authority wallet"
    else
        echo -e "${YELLOW}⚠${NC} Current wallet differs from authority wallet"
    fi
else
    echo -e "${YELLOW}⚠${NC} Authority wallet not specified (will use current wallet)"
fi
echo ""

# Check initialization prerequisites
echo "9. Checking initialization prerequisites..."
PREREQUISITES=true

if ! solana program show "$EXPECTED_PROGRAM_ID" &> /dev/null; then
    echo -e "${RED}✗${NC} Program not deployed"
    PREREQUISITES=false
fi

if [ -z "$TOKEN_MINT" ]; then
    echo -e "${RED}✗${NC} Token mint not configured"
    PREREQUISITES=false
fi

if [ -z "$MAX_BUY" ] || [ -z "$MAX_WALLET" ] || [ -z "$COOLDOWN_SECONDS" ]; then
    echo -e "${RED}✗${NC} Required anti-sniper parameters not configured"
    PREREQUISITES=false
fi

if $PREREQUISITES; then
    echo -e "${GREEN}✓${NC} Initialization prerequisites met"
else
    echo -e "${RED}✗${NC} Initialization prerequisites not met"
    SOL_LAUNCH_ERRORS=$((SOL_LAUNCH_ERRORS + 1))
fi
echo ""

# Check IDL file availability
echo "10. Checking IDL file availability..."
if [ -f "target/idl/sol_launch.json" ]; then
    echo -e "${GREEN}✓${NC} IDL file exists"
else
    echo -e "${RED}✗${NC} IDL file not found"
    SOL_LAUNCH_ERRORS=$((SOL_LAUNCH_ERRORS + 1))
fi
echo ""

# Final summary
echo "=== SOL-LAUNCH INITIALIZATION CHECK SUMMARY ==="
if [ $SOL_LAUNCH_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Sol-Launch initialization is ready"
    echo ""
    echo "Next steps:"
    echo "1. Initialize Sol-Launch: Use frontend or script with configured parameters"
    echo "2. Deposit tokens to vault"
    echo "3. Enable trading"
    echo "4. Monitor protected trading period"
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $SOL_LAUNCH_ERRORS Sol-Launch initialization issues found"
    exit 1
fi