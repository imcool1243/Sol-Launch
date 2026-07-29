#!/bin/bash

# Sol-Launch Deployment Script for Devnet
# This script deploys the sol-launch program to Solana devnet

set -e

echo "🚀 Starting Sol-Launch deployment to devnet..."

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI is not installed. Please install it first."
    exit 1
fi

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor is not installed. Please install it first."
    exit 1
fi

# Configure Solana for devnet
echo "📡 Configuring Solana for devnet..."
solana config set --url devnet
solana config set --keypair ~/.config/solana/id.json

# Check wallet balance
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance | awk '{print $1}')
echo "Current balance: $BALANCE SOL"

if [ "$BALANCE" = "0" ]; then
    echo "⚠️  Wallet balance is 0 SOL. Requesting airdrop..."
    solana airdrop 1 || echo "❌ Airdrop failed. Please manually fund your wallet."
fi

# Build the program
echo "🔨 Building the program..."
anchor build

# Deploy the program
echo "📦 Deploying program to devnet..."
anchor deploy --provider-cluster devnet

echo "✅ Deployment successful!"
echo ""
echo "📋 Next steps:"
echo "1. Update your frontend with the new program ID"
echo "2. Test the program on devnet"
echo "3. Verify all functionality works as expected"
echo ""
echo "🔗 Program ID: 2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"