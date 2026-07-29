# Mainnet Launch Assistant Guide

The Sol-Launch Mainnet Launch Assistant provides automated verification to prevent mistakes during mainnet deployment.

## Overview

The launch assistant runs comprehensive checks to ensure the system is ready for mainnet deployment and provides a clear **READY TO LAUNCH** or **BLOCKED** status.

## Usage

### Quick Start

```bash
# Run the mainnet launcher
./mainnet_launcher.sh
```

This will run all verification checks and provide the final launch readiness status.

### Individual Checks

You can also run individual check scripts if needed:

```bash
# Wallet configuration check
./scripts/wallet_check.sh

# Network configuration check
./scripts/network_check.sh

# Program deployment check
./scripts/program_check.sh

# Token creation check
./scripts/token_check.sh

# Metadata check
./scripts/metadata_check.sh

# Sol-Launch initialization check
./scripts/sol_launch_check.sh

# Authority check
./scripts/authority_check.sh

# Comprehensive launch readiness check
./scripts/launch_readiness.sh
```

## Check Categories

### Phase 1: Pre-Deployment Checks

#### Wallet Check (`wallet_check.sh`)
- Solana CLI installation
- Network configuration (must be mainnet)
- Wallet file existence and validity
- Wallet balance (need 5+ SOL recommended)
- Wallet file permissions (must be 600)
- Hardware wallet detection (recommended)

#### Network Check (`network_check.sh`)
- Solana CLI installation
- Network configuration (must be mainnet-beta)
- Network connectivity
- RPC endpoint verification
- Network health and performance
- Current slot and transaction count

#### Program Check (`program_check.sh`)
- Anchor CLI installation
- Program binary existence
- Program keypair validation
- IDL file verification
- Program ID consistency
- Program deployment status
- Program authority verification
- Program size limits

### Phase 2: Token Configuration Checks

#### Token Check (`token_check.sh`)
- SPL Token CLI installation
- Token mint existence
- Wallet SOL balance for token creation
- Token parameter configuration
- Metadata URI configuration
- Token decimals configuration
- Token account verification

#### Metadata Check (`metadata_check.sh`)
- Token mint availability
- Metadata file validation
- Metadata URI accessibility
- Required metadata fields
- Image and external URL metadata
- Logo file verification
- On-chain metadata verification

### Phase 3: Sol-Launch Initialization Checks

#### Sol-Launch Check (`sol_launch_check.sh`)
- Program deployment verification
- Program deployment on network
- Anti-sniper parameter configuration
- Progressive limits configuration
- Anti-scam configuration
- Token mint for initialization
- Total supply configuration
- Wallet authority verification
- Initialization prerequisites
- IDL file availability

#### Authority Check (`authority_check.sh`)
- Current wallet authority
- Wallet keypair permissions
- Program authority verification
- Hardware wallet detection
- Multisig configuration
- Wallet balance for authority operations
- Authority wallet separation
- Authority wallet backup
- Security best practices
- Authority transfer capability

## Environment Variables

Configure these environment variables before running the checks:

### Token Configuration
```bash
export TOKEN_MINT="<your_token_mint_address>"
export TOKEN_NAME="Your Token Name"
export TOKEN_SYMBOL="SYMBOL"
export TOKEN_SUPPLY="1000000"
export TOKEN_DECIMALS="9"
export TOKEN_METADATA_URI="https://your-site.com/metadata.json"
```

### Anti-Sniper Configuration
```bash
export MAX_BUY="1000"
export MAX_WALLET="5000"
export COOLDOWN_SECONDS="60"
export PROGRESSIVE_LIMITS_ENABLED="true"
export INITIAL_MAX_BUY="500"
export INITIAL_MAX_WALLET="2500"
export LIMIT_INCREASE_INTERVAL="300"
export LIMIT_INCREASE_MULTIPLIER="1"
export ANTI_SCAM_ENABLED="true"
export MAX_TRADES_PER_USER="20"
```

### Authority Configuration
```bash
export AUTHORITY_WALLET="<authority_wallet_address>"
export MULTISIG_WALLET="<multisig_wallet_address>"
```

## Output Format

### READY TO LAUNCH
```
╔════════════════════════════════════════════════════════════════╗
║                    READY TO LAUNCH                             ║
╚════════════════════════════════════════════════════════════════╝

All critical checks have passed. The Sol-Launch system is ready
for mainnet deployment.

Next steps:
1. Review all check results above
2. Ensure environment variables are set correctly
3. Follow MAINNET_LAUNCH_GUIDE.md for deployment
4. Monitor launch continuously
```

### BLOCKED
```
╔════════════════════════════════════════════════════════════════╗
║                      BLOCKED                                   ║
╚════════════════════════════════════════════════════════════════╝

Critical issues found that must be resolved before mainnet deployment.

FAILED CHECKS:
✗ Network Check
✗ Token Creation Check

Resolution steps:
1. Fix the failed checks above
2. Re-run this script: bash scripts/launch_readiness.sh
3. Ensure all checks pass before deployment
```

## Common Issues and Solutions

### Network not configured for mainnet
**Issue**: System is on devnet or testnet
**Solution**: `solana config set --url mainnet-beta`

### Insufficient SOL balance
**Issue**: Wallet doesn't have enough SOL for deployment
**Solution**: Add 5+ SOL to wallet for deployment costs

### Program not deployed
**Issue**: Smart contract not deployed to mainnet
**Solution**: `anchor deploy --provider-cluster mainnet-beta`

### Token mint not created
**Issue**: Token doesn't exist on-chain
**Solution**: `spl-token create-token`

### Missing environment variables
**Issue**: Required configuration not set
**Solution**: Set environment variables before running checks

### Wallet permissions insecure
**Issue**: Wallet file has wrong permissions
**Solution**: `chmod 600 ~/.config/solana/id.json`

## Security Features

The launch assistant includes security checks to prevent common mistakes:

- **Network verification**: Ensures deployment to correct network
- **Authority validation**: Confirms wallet permissions are secure
- **Balance verification**: Ensures sufficient funds for operations
- **Hardware wallet detection**: Recommends hardware wallet for mainnet
- **Backup verification**: Checks for wallet file backups
- **Configuration validation**: Verifies all parameters are set correctly

## Best Practices

### Before Running Checks
1. Ensure all environment variables are set
2. Configure wallet for mainnet (hardware wallet recommended)
3. Ensure sufficient SOL balance (5+ SOL)
4. Have wallet files backed up securely
5. Test on devnet first if uncertain

### During Checks
1. Review each check result carefully
2. Address any warnings or errors
3. Verify all parameters are correct
4. Ensure network is configured for mainnet

### After Successful Checks
1. Follow MAINNET_LAUNCH_GUIDE.md step by step
2. Monitor launch continuously
3. Have emergency procedures ready
4. Keep backup files secure

## Integration with Deployment Process

The launch assistant is designed to be run immediately before mainnet deployment:

1. **Run launch assistant**: `./mainnet_launcher.sh`
2. **Review results**: If READY, proceed; if BLOCKED, fix issues
3. **Configure parameters**: Set all required environment variables
4. **Deploy smart contract**: Follow MAINNET_LAUNCH_GUIDE.md
5. **Create token**: Follow TOKEN_CREATION_GUIDE.md
6. **Initialize Sol-Launch**: Configure anti-sniper protections
7. **Enable trading**: Start protected trading period
8. **Monitor launch**: Watch for issues and be ready to intervene

## Troubleshooting

### Script fails to run
- Ensure script has execute permissions: `chmod +x mainnet_launcher.sh`
- Check that you're in the project directory
- Verify bash is installed: `which bash`

### Check fails unexpectedly
- Review the specific check script output
- Check network connectivity
- Verify wallet file exists and is valid
- Ensure Solana CLI is installed and configured

### Environment variables not recognized
- Export variables before running script
- Verify variable names are correct
- Check for typos in values
- Ensure terminal session has the variables set

## Maintenance

The launch assistant scripts are designed to be maintainable:

- Each check is independent and can be modified separately
- Output format is consistent across all checks
- Error codes are standardized (0 = success, 1 = failure)
- Documentation is included in each script header

## Support

If you encounter issues with the launch assistant:

1. Review individual check scripts for detailed error messages
2. Check the project documentation in MAINNET_LAUNCH_GUIDE.md
3. Verify your environment matches the prerequisites
4. Test on devnet first to isolate issues

## Conclusion

The Mainnet Launch Assistant provides automated verification to prevent mistakes during mainnet deployment. It ensures all critical checks pass before allowing deployment, reducing the risk of errors during the mainnet launch process.

**Always run the launch assistant before mainnet deployment to ensure system readiness.**