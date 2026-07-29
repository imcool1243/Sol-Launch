#!/bin/bash

# Sol-Launch Monitoring Script
# This script monitors the deployed program on devnet

set -e

PROGRAM_ID="2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"

echo "📊 Sol-Launch Monitoring for Devnet"
echo "====================================="
echo ""

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI is not installed."
    exit 1
fi

# Check configuration
echo "📡 Current Solana Configuration:"
solana config get
echo ""

# Check wallet balance
echo "💰 Wallet Balance:"
solana balance
echo ""

# Check program deployment
echo "📦 Program Status:"
if solana program show $PROGRAM_ID > /dev/null 2>&1; then
    echo "✅ Program is deployed: $PROGRAM_ID"
    solana program show $PROGRAM_ID
else
    echo "❌ Program is not deployed: $PROGRAM_ID"
fi
echo ""

# Get recent program logs
echo "📋 Recent Program Logs:"
solana logs $PROGRAM_ID --limit 5 || echo "No recent logs available"
echo ""

echo "✅ Monitoring complete!"
echo ""
echo "🔗 Useful Commands:"
echo "  - Get program logs: solana logs $PROGRAM_ID"
echo "  - Get program data: solana program show $PROGRAM_ID"
echo "  - Get account data: solana account <PUBKEY>"
echo "  - Check wallet: solana balance"