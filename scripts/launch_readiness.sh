#!/bin/bash

# Sol-Launch Mainnet Launch Readiness Check
# Comprehensive verification before mainnet deployment

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           SOL-LAUNCH MAINNET LAUNCH READINESS CHECK            ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32M'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

TOTAL_ERRORS=0
TOTAL_WARNINGS=0

# Array to track check results
declare -a CHECK_RESULTS

# Function to run individual checks
run_check() {
    local check_name=$1
    local check_script=$2
    
    echo "▶ Running: $check_name"
    echo "─────────────────────────────────────────────────────────────"
    
    if bash "$check_script"; then
        echo -e "${GREEN}✓${NC} $check_name: PASSED"
        CHECK_RESULTS+=("$check_name:PASSED")
        echo ""
    else
        echo -e "${RED}✗${NC} $check_name: FAILED"
        CHECK_RESULTS+=("$check_name:FAILED")
        TOTAL_ERRORS=$((TOTAL_ERRORS + 1))
        echo ""
    fi
}

# Change to scripts directory
cd "$(dirname "$0")"

# Run all checks
echo "PHASE 1: PRE-DEPLOYMENT CHECKS"
echo "═════════════════════════════════════════════════════════════════"
echo ""

run_check "Wallet Check" "./wallet_check.sh"
run_check "Network Check" "./network_check.sh"
run_check "Program Deployment Check" "./program_check.sh"

echo "PHASE 2: TOKEN CONFIGURATION CHECKS"
echo "═════════════════════════════════════════════════════════════════"
echo ""

run_check "Token Creation Check" "./token_check.sh"
run_check "Metadata Check" "./metadata_check.sh"

echo "PHASE 3: SOL-LAUNCH INITIALIZATION CHECKS"
echo "═════════════════════════════════════════════════════════════════"
echo ""

run_check "Sol-Launch Initialization Check" "./sol_launch_check.sh"
run_check "Authority Check" "./authority_check.sh"

# Final summary
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                  LAUNCH READINESS SUMMARY                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

echo "CHECK RESULTS:"
echo "─────────────────────────────────────────────────────────────"
for result in "${CHECK_RESULTS[@]}"; do
    CHECK_NAME=$(echo "$result" | cut -d':' -f1)
    CHECK_STATUS=$(echo "$result" | cut -d':' -f2)
    
    if [ "$CHECK_STATUS" == "PASSED" ]; then
        echo -e "${GREEN}✓${NC} $CHECK_NAME"
    else
        echo -e "${RED}✗${NC} $CHECK_NAME"
    fi
done
echo ""

echo "TOTAL ERRORS: $TOTAL_ERRORS"
echo "TOTAL WARNINGS: $TOTAL_WARNINGS"
echo ""

# Final determination
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                      FINAL STATUS                               ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

if [ $TOTAL_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓✓✓ READY TO LAUNCH ✓✓✓${NC}"
    echo ""
    echo "All critical checks have passed. The Sol-Launch system is ready"
    echo "for mainnet deployment."
    echo ""
    echo "Next steps:"
    echo "1. Review all check results above"
    echo "2. Ensure environment variables are set correctly"
    echo "3. Follow MAINNET_LAUNCH_GUIDE.md for deployment"
    echo "4. Monitor launch continuously"
    echo ""
    echo "Remember:"
    echo "- Keep wallet files secure"
    echo "- Have emergency procedures ready"
    echo "- Monitor network congestion"
    echo "- Test on devnet first if uncertain"
    exit 0
else
    echo -e "${RED}✗✗✗ BLOCKED ✗✗✗${NC}"
    echo ""
    echo "Critical issues found that must be resolved before mainnet deployment."
    echo ""
    echo "FAILED CHECKS:"
    for result in "${CHECK_RESULTS[@]}"; do
        CHECK_NAME=$(echo "$result" | cut -d':' -f1)
        CHECK_STATUS=$(echo "$result" | cut -d':' -f2)
        
        if [ "$CHECK_STATUS" == "FAILED" ]; then
            echo -e "${RED}✗${NC} $CHECK_NAME"
        fi
    done
    echo ""
    echo "Resolution steps:"
    echo "1. Fix the failed checks above"
    echo "2. Re-run this script: bash scripts/launch_readiness.sh"
    echo "3. Ensure all checks pass before deployment"
    echo ""
    echo "Common issues:"
    echo "- Network not configured for mainnet (run: solana config set --url mainnet-beta)"
    echo "- Insufficient SOL balance (need 5+ SOL for deployment)"
    echo "- Program not deployed (run: anchor deploy --provider-cluster mainnet-beta)"
    echo "- Token mint not created (run: spl-token create-token)"
    echo "- Missing environment variables (see individual check scripts)"
    exit 1
fi