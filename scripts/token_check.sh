#!/bin/bash

# Sol-Launch Token Creation Check Script
# Verifies token configuration for mainnet

set -e

echo "=== TOKEN CREATION CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TOKEN_ERRORS=0

# Check if SPL Token CLI is installed
echo "1. Checking SPL Token CLI installation..."
if command -v spl-token &> /dev/null; then
    echo -e "${GREEN}✓${NC} SPL Token CLI installed"
else
    echo -e "${RED}✗${NC} SPL Token CLI not installed"
    echo "   Run: cargo install spl-token-cli"
    TOKEN_ERRORS=$((TOKEN_ERRORS + 1))
fi
echo ""

# Check if token mint exists (optional - user may not have created yet)
echo "2. Checking for existing token mint..."
if [ -n "$TOKEN_MINT" ]; then
    echo "   Checking token mint: $TOKEN_MINT"
    if spl-token display --mint "$TOKEN_MINT" &> /dev/null; then
        echo -e "${GREEN}✓${NC} Token mint exists and is valid"
        spl-token display --mint "$TOKEN_MINT"
    else
        echo -e "${RED}✗${NC} Token mint does not exist or is invalid"
        TOKEN_ERRORS=$((TOKEN_ERRORS + 1))
    fi
else
    echo -e "${YELLOW}⚠${NC} No token mint specified (TOKEN_MINT environment variable not set)"
    echo "   Token mint will be created during launch process"
fi
echo ""

# Check wallet has sufficient SOL for token creation
echo "3. Checking wallet SOL balance for token creation..."
WALLET_BALANCE=$(solana balance | awk '{print $1}')
if [[ "$WALLET_BALANCE" == *"SOL"* ]]; then
    BALANCE_NUM=$(echo $WALLET_BALANCE | awk '{print $1}')
    echo "   Wallet balance: $WALLET_BALANCE"
    
    # Need at least 0.5 SOL for token creation + fees
    if (( $(echo "$BALANCE_NUM >= 0.5" | bc -l) )); then
        echo -e "${GREEN}✓${NC} Sufficient balance for token creation"
    else
        echo -e "${YELLOW}⚠${NC} Low balance for token creation (recommended: 1+ SOL)"
        TOKEN_ERRORS=$((TOKEN_ERRORS + 1))
    fi
fi
echo ""

# Check if user has decided on token parameters
echo "4. Checking token parameter preparation..."
if [ -n "$TOKEN_NAME" ] && [ -n "$TOKEN_SYMBOL" ] && [ -n "$TOKEN_SUPPLY" ]; then
    echo -e "${GREEN}✓${NC} Token parameters configured:"
    echo "   Name: $TOKEN_NAME"
    echo "   Symbol: $TOKEN_SYMBOL"
    echo "   Supply: $TOKEN_SUPPLY"
else
    echo -e "${YELLOW}⚠${NC} Token parameters not fully configured"
    echo "   Set environment variables:"
    echo "   export TOKEN_NAME=\"Your Token Name\""
    echo "   export TOKEN_SYMBOL=\"SYMBOL\""
    echo "   export TOKEN_SUPPLY=\"1000000\""
fi
echo ""

# Check for metadata preparation
echo "5. Checking metadata preparation..."
if [ -n "$TOKEN_METADATA_URI" ]; then
    echo -e "${GREEN}✓${NC} Metadata URI configured: $TOKEN_METADATA_URI"
else
    echo -e "${YELLOW}⚠${NC} Metadata URI not configured (optional but recommended)"
    echo "   Set: export TOKEN_METADATA_URI=\"https://your-site.com/metadata.json\""
fi
echo ""

# Check if token decimals are set
echo "6. Checking token decimals configuration..."
if [ -n "$TOKEN_DECIMALS" ]; then
    echo -e "${GREEN}✓${NC} Token decimals configured: $TOKEN_DECIMALS"
else
    echo -e "${YELLOW}⚠${NC} Token decimals not configured (will use default: 9)"
    echo "   Set: export TOKEN_DECIMALS=\"9\""
fi
echo ""

# Verify token creation prerequisites
echo "7. Verifying token creation prerequisites..."
PREREQUISITES=true

if ! command -v spl-token &> /dev/null; then
    echo -e "${RED}✗${NC} SPL Token CLI not installed"
    PREREQUISITES=false
fi

if [ -n "$TOKEN_MINT" ]; then
    if ! spl-token display --mint "$TOKEN_MINT" &> /dev/null; then
        echo -e "${RED}✗${NC} Token mint invalid"
        PREREQUISITES=false
    fi
fi

if $PREREQUISITES; then
    echo -e "${GREEN}✓${NC} Token creation prerequisites met"
else
    echo -e "${RED}✗${NC} Token creation prerequisites not met"
    TOKEN_ERRORS=$((TOKEN_ERRORS + 1))
fi
echo ""

# Check for existing token accounts
echo "8. Checking existing token accounts..."
if [ -n "$TOKEN_MINT" ]; then
    if spl-token accounts &> /dev/null; then
        echo -e "${GREEN}✓${NC} Can list token accounts"
        echo "   Token accounts:"
        spl-token accounts | head -5
    else
        echo -e "${YELLOW}⚠${NC} Cannot list token accounts"
    fi
fi
echo ""

# Final summary
echo "=== TOKEN CREATION CHECK SUMMARY ==="
if [ $TOKEN_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Token creation is ready"
    echo ""
    echo "Next steps:"
    echo "1. Create token: spl-token create-token"
    echo "2. Create token account: spl-token create-account <TOKEN_MINT>"
    echo "3. Mint supply: spl-token mint <TOKEN_MINT> <AMOUNT>"
    echo "4. Revoke mint authority: spl-token authorize <TOKEN_MINT> mint --disable"
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $TOKEN_ERRORS token creation issues found"
    exit 1
fi