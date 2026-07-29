#!/bin/bash

# Sol-Launch Mainnet Launcher
# Main script to run all checks and provide launch readiness status

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              SOL-LAUNCH MAINNET LAUNCH ASSISTANT               ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "This script will run all verification checks to determine if"
echo "the Sol-Launch system is ready for mainnet deployment."
echo ""
echo "Press Enter to continue or Ctrl+C to cancel..."
read

# Change to project directory
cd "$(dirname "$0")"

# Run the comprehensive launch readiness check
if bash scripts/launch_readiness.sh; then
    echo ""
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                    READY TO LAUNCH                             ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "The Sol-Launch system has passed all critical checks."
    echo ""
    echo "Before proceeding to mainnet deployment:"
    echo "1. Review the check results above"
    echo "2. Ensure you have sufficient SOL (5+ SOL recommended)"
    echo "3. Verify all environment variables are set correctly"
    echo "4. Read MAINNET_LAUNCH_GUIDE.md completely"
    echo "5. Consider testing on devnet first"
    echo ""
    echo "To proceed with deployment:"
    echo "1. Follow MAINNET_LAUNCH_GUIDE.md step by step"
    echo "2. Monitor the launch continuously"
    echo "3. Have emergency procedures ready"
    echo ""
    echo "Good luck with your launch!"
    exit 0
else
    echo ""
    echo "╔════════════════════════════════════════════════════════════════╗"
    echo "║                      BLOCKED                                   ║"
    echo "╚════════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Critical issues must be resolved before mainnet deployment."
    echo "Please fix the failed checks and run this script again."
    exit 1
fi