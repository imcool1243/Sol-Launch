#!/bin/bash

# Sol-Launch Metadata Check Script
# Verifies token metadata configuration for mainnet

set -e

echo "=== METADATA CHECK ==="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

METADATA_ERRORS=0

# Check if token mint is provided
echo "1. Checking token mint availability..."
if [ -n "$TOKEN_MINT" ]; then
    echo -e "${GREEN}✓${NC} Token mint provided: $TOKEN_MINT"
else
    echo -e "${YELLOW}⚠${NC} No token mint provided (TOKEN_MINT environment variable not set)"
    echo "   Set: export TOKEN_MINT=\"<your_token_mint_address>\""
fi
echo ""

# Check for metadata file
echo "2. Checking for metadata file..."
if [ -f "metadata.json" ]; then
    echo -e "${GREEN}✓${NC} Metadata file exists: metadata.json"
    
    # Validate JSON structure
    if python3 -m json.tool metadata.json &> /dev/null; then
        echo -e "${GREEN}✓${NC} Metadata file is valid JSON"
    else
        echo -e "${RED}✗${NC} Metadata file is not valid JSON"
        METADATA_ERRORS=$((METADATA_ERRORS + 1))
    fi
else
    echo -e "${YELLOW}⚠${NC} No metadata file found (optional but recommended)"
    echo "   Create metadata.json with token information"
fi
echo ""

# Check metadata URI if set
echo "3. Checking metadata URI configuration..."
if [ -n "$TOKEN_METADATA_URI" ]; then
    echo -e "${GREEN}✓${NC} Metadata URI configured: $TOKEN_METADATA_URI"
    
    # Try to fetch metadata (optional)
    if command -v curl &> /dev/null; then
        if curl -s --head "$TOKEN_METADATA_URI" | head -n 1 | grep "HTTP" &> /dev/null; then
            echo -e "${GREEN}✓${NC} Metadata URI is accessible"
        else
            echo -e "${YELLOW}⚠${NC} Metadata URI may not be accessible"
        fi
    fi
else
    echo -e "${YELLOW}⚠${NC} No metadata URI configured"
    echo "   Set: export TOKEN_METADATA_URI=\"https://your-site.com/metadata.json\""
fi
echo ""

# Check for required metadata fields
echo "4. Checking metadata field requirements..."
if [ -f "metadata.json" ]; then
    REQUIRED_FIELDS=("name" "symbol" "description")
    MISSING_FIELDS=0
    
    for field in "${REQUIRED_FIELDS[@]}"; do
        if grep -q "\"$field\"" metadata.json; then
            echo -e "${GREEN}✓${NC} Field '$field' present"
        else
            echo -e "${YELLOW}⚠${NC} Field '$field' missing (recommended)"
            MISSING_FIELDS=$((MISSING_FIELDS + 1))
        fi
    done
    
    if [ $MISSING_FIELDS -eq 0 ]; then
        echo -e "${GREEN}✓${NC} All recommended metadata fields present"
    else
        echo -e "${YELLOW}⚠${NC} $MISSING_FIELDS recommended metadata fields missing"
    fi
fi
echo ""

# Check for image metadata
echo "5. Checking image metadata..."
if [ -f "metadata.json" ]; then
    if grep -q "\"image\"" metadata.json; then
        echo -e "${GREEN}✓${NC} Image field present in metadata"
        IMAGE_URL=$(grep "\"image\"" metadata.json | awk -F'"' '{print $4}')
        echo "   Image URL: $IMAGE_URL"
    else
        echo -e "${YELLOW}⚠${NC} No image field in metadata (recommended for Phantom display)"
    fi
fi
echo ""

# Check for external URL
echo "6. Checking external URL metadata..."
if [ -f "metadata.json" ]; then
    if grep -q "\"external_url\"" metadata.json; then
        echo -e "${GREEN}✓${NC} External URL field present"
        EXTERNAL_URL=$(grep "\"external_url\"" metadata.json | awk -F'"' '{print $4}')
        echo "   External URL: $EXTERNAL_URL"
    else
        echo -e "${YELLOW}⚠${NC} No external URL field (recommended for project website)"
    fi
fi
echo ""

# Check for logo file
echo "7. Checking for logo file..."
if [ -f "logo.png" ] || [ -f "logo.jpg" ] || [ -f "logo.svg" ]; then
    echo -e "${GREEN}✓${NC} Logo file found"
    ls -lh logo.* 2>/dev/null
else
    echo -e "${YELLOW}⚠${NC} No logo file found (recommended for Phantom display)"
fi
echo ""

# Verify metadata with Solana tools (if token mint exists)
echo "8. Verifying on-chain metadata (if token exists)..."
if [ -n "$TOKEN_MINT" ]; then
    if command -v spl-token &> /dev/null; then
        if spl-token display --mint "$TOKEN_MINT" &> /dev/null; then
            echo -e "${GREEN}✓${NC} Token exists on-chain"
            spl-token display --mint "$TOKEN_MINT"
        else
            echo -e "${YELLOW}⚠${NC} Token not yet on-chain (will be created during launch)"
        fi
    fi
fi
echo ""

# Check Metaplex CLI (optional)
echo "9. Checking Metaplex CLI (optional)..."
if command -v metaplex &> /dev/null; then
    echo -e "${GREEN}✓${NC} Metaplex CLI installed (can create on-chain metadata)"
else
    echo -e "${YELLOW}⚠${NC} Metaplex CLI not installed (optional for on-chain metadata)"
    echo "   Install: npm install -g @metaplex-foundation/sugar"
fi
echo ""

# Final summary
echo "=== METADATA CHECK SUMMARY ==="
if [ $METADATA_ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ READY${NC} - Metadata configuration is ready"
    echo ""
    echo "Metadata is optional but recommended for Phantom token display."
    echo "Basic token functionality works without metadata."
    exit 0
else
    echo -e "${RED}✗ BLOCKED${NC} - $METADATA_ERRORS metadata issues found"
    exit 1
fi