#!/bin/bash

# Sol-Launch Authority Check Script
# Verifies authority configuration and security for mainnet

set -e

echo "=== AUTHORITY CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

AUTHORITY_ERRORS=0

# Check current wallet
echo "1. Checking current wallet authority..."
WALLET_ADDRESS=$(solana address)
echo "   Current wallet: $WALLET_ADDRESS"
if [ -n "$WALLET_ADDRESS" ]; then
    echo -e "${GREEN}✓${NC} Wallet address valid"
else
    echo -e "${RED}✗${NC} Cannot get wallet address"
    AUTHORITY_ERRORS=$((AUTHORITY_ERRORS + 1))
fi
echo ""

# Check wallet keypair permissions
echo "2. Checking wallet keypair permissions..."
WALLET_FILE="$HOME/.config/solana/id.json"
if [ -f "$WALLET_FILE" ]; then
    PERMISSIONS=$(stat -c "%a" "$WALLET_FILE")
    if [ "$PERMISSIONS" == "600" ]; then
        echo -e "${GREEN}✓${NC} Wallet keypair has secure permissions: $PERMISSIONS"
    else
        echo -e "${RED}✗${NC} Wallet keypair permissions insecure: $PERMISSIONS"
        echo "   Run: chmod 600 $WALLET_FILE"
        AUTHORITY_ERRORS=$((AUTHORITY_ERRORS + 1))
    fi
else
    echo -e "${RED}✗${NC} Wallet keypair file not found"
    AUTHORITY_ERRORS=$((AUTHORITY_ERRORS + 1))
fi
echo ""

# Check program authority
echo "3. Checking program authority..."
EXPECTED_PROGRAM_ID="2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"
if solana program show "$EXPECTED_PROGRAM_ID" &> /dev/null; then
    PROGRAM_AUTHORITY=$(solana program show "$EXPECTED_PROGRAM_ID" | grep "Authority" | awk '{print $2}')
    echo "   Program authority: $PROGRAM_AUTHORITY"
    
    if [ "$PROGRAM_AUTHORITY" == "$WALLET_ADDRESS" ]; then
        echo -e "${GREEN}✓${NC} Program authority matches current wallet"
    else
        echo -e "${YELLOW}⚠${NC} Program authority differs from current wallet"
        echo "   This may be intentional if using authority wallet"
    fi
else
    echo -e "${YELLOW}⚠${NC} Program not yet deployed (cannot check authority)"
fi
echo ""

# Check for hardware wallet
echo "4. Checking for hardware wallet (recommended for mainnet)..."
if command -v solana-keygen pubkey usb://ledger &> /dev/null; then
    echo -e "${GREEN}✓${NC} Hardware wallet detected (Ledger)"
    HARDWARE_WALLET=true
elif command -v solana-keygen pubkey usb://trezor &> /dev/null; then
    echo -e "${GREEN}✓${NC} Hardware wallet detected (Trezor)"
    HARDWARE_WALLET=true
else
    echo -e "${YELLOW}⚠${NC} No hardware wallet detected"
    echo "   Hardware wallet recommended for mainnet authority"
    HARDWARE_WALLET=false
fi
echo ""

# Check for multisig configuration
echo "5. Checking for multisig configuration..."
if [ -n "$MULTISIG_WALLET" ]; then
    echo -e "${GREEN}✓${NC} Multisig wallet configured: $MULTISIG_WALLET"
    echo "   Multisig is recommended for team-managed projects"
else
    echo -e "${YELLOW}⚠${NC} No multisig wallet configured"
    echo "   Consider multisig for team-managed projects"
fi
echo ""

# Check wallet balance
echo "6. Checking wallet balance for authority operations..."
WALLET_BALANCE=$(solana balance | awk '{print $1}')
if [[ "$WALLET_BALANCE" == *"SOL"* ]]; then
    BALANCE_NUM=$(echo $WALLET_BALANCE | awk '{print $1}')
    echo "   Wallet balance: $WALLET_BALANCE"
    
    # Need at least 1 SOL for authority operations
    if (( $(echo "$BALANCE_NUM >= 1" | bc -l) )); then
        echo -e "${GREEN}✓${NC} Sufficient balance for authority operations"
    else
        echo -e "${YELLOW}⚠${NC} Low balance for authority operations (recommended: 2+ SOL)"
        AUTHORITY_ERRORS=$((AUTHORITY_ERRORS + 1))
    fi
fi
echo ""

# Check authority wallet separation
echo "7. Checking authority wallet separation..."
if [ -n "$AUTHORITY_WALLET" ]; then
    if [ "$AUTHORITY_WALLET" != "$WALLET_ADDRESS" ]; then
        echo -e "${GREEN}✓${NC} Authority wallet separated from current wallet"
        echo "   Authority wallet: $AUTHORITY_WALLET"
        echo "   Current wallet: $WALLET_ADDRESS"
    else
        echo -e "${YELLOW}⚠${NC} Authority wallet same as current wallet"
        echo "   Consider using separate authority wallet for security"
    fi
else
    echo -e "${YELLOW}⚠${NC} No separate authority wallet configured"
    echo "   Consider using separate authority wallet for security"
fi
echo ""

# Check for backup of authority wallet
echo "8. Checking for authority wallet backup..."
if [ -f "BACKUP_OFFLINE/wallets/id.json" ]; then
    echo -e "${GREEN}✓${NC} Authority wallet backup found"
else
    echo -e "${YELLOW}⚠${NC} No authority wallet backup found"
    echo "   Ensure authority wallet is backed up securely"
fi
echo ""

# Security best practices check
echo "9. Checking security best practices..."
SECURITY_SCORE=0

if [ "$PERMISSIONS" == "600" ]; then
    SECURITY_SCORE=$((SECURITY_SCORE + 1))
fi

if $HARDWARE_WALLET; then
    SECURITY_SCORE=$((SECURITY_SCORE + 1))
fi

if [ -n "$MULTISIG_WALLET" ]; then
    SECURITY_SCORE=$((SECURITY_SCORE + 1))
fi

if [ -n "$AUTHORITY_WALLET" ] && [ "$AUTHORITY_WALLET" != "$WALLET_ADDRESS" ]; then
    SECURITY_SCORE=$((SECURITY_SCORE + 1))
fi

if [ -f "BACKUP_OFFLINE/wallets/id.json" ]; then
    SECURITY_SCORE=$((SECURITY_SCORE + 1))
fi

echo "   Security score: $SECURITY_SCORE/5"
if [ $SECURITY_SCORE -ge 3 ]; then
    echo -e "${GREEN}✓${NC} Security practices are reasonable"
else
    echo -e "${YELLOW}⚠${NC} Security practices need improvement"
    echo "   Consider: hardware wallet, multisig, separate authority wallet"
fi
echo ""

# Check for authority transfer capability
echo "10. Checking authority transfer capability..."
if [ -f "target/idl/sol_launch.json" ]; then
    if grep -q "transfer_authority" target/idl/sol_launch.json; then
        echo -e "${GREEN}✓${NC} Authority transfer instruction available"
        echo "   Authority can be transferred if needed"
    else
        echo -e "${YELLOW}⚠${NC} Authority transfer instruction not found"
    fi
else
    echo -e "${YELLOW}⚠${NC} IDL file not found (cannot check authority transfer)"
fi
echo ""

# Final summary
echo "=== AUTHORITY CHECK SUMMARY ==="
if [ $AUTHORITY_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Authority configuration is acceptable"
    echo ""
    echo "Security recommendations:"
    if ! $HARDWARE_WALLET; then
        echo "- Consider using hardware wallet for mainnet authority"
    fi
    if [ -z "$MULTISIG_WALLET" ]; then
        echo "- Consider multisig for team-managed projects"
    fi
    if [ -z "$AUTHORITY_WALLET" ] || [ "$AUTHORITY_WALLET" == "$WALLET_ADDRESS" ]; then
        echo "- Consider using separate authority wallet"
    fi
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $AUTHORITY_ERRORS authority issues found"
    exit 1
fi