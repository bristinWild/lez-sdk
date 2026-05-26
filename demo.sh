#!/usr/bin/env bash
# lez-sdk Counter Demo
# Demonstrates the counter program running on LEZ with RISC0_DEV_MODE=0
# Run with: RISC0_DEV_MODE=0 bash demo.sh

set -euo pipefail

echo "================================================================"
echo " lez-sdk: Counter Program — On-Chain Demo"
echo " RISC0_DEV_MODE=${RISC0_DEV_MODE:-not set}"
echo "================================================================"
echo ""

LEZ_DIR="$HOME/rebase-lez/logos-execution-zone"
DEMO_DIR="$HOME/rebase-lez/lp0013-demo"
SPEL_CLI="$HOME/rebase-lez/spel/target/release/spel"
COUNTER_BIN="$LEZ_DIR/target/riscv32im-risc0-zkvm-elf/docker/counter.bin"
WALLET_DIR="$DEMO_DIR/.scaffold/wallet"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
IDL="$SCRIPT_DIR/counter.idl.json"

# ── 1. Check localnet ────────────────────────────────────────────────
echo "[1/5] Checking localnet..."
cd "$DEMO_DIR"
if lgs localnet status 2>/dev/null | grep -q "ready: true"; then
    echo "      Localnet already running."
else
    lgs localnet start
    echo "      Localnet started."
fi

# ── 2. Fund wallet ───────────────────────────────────────────────────
echo "[2/5] Funding wallet..."
lgs wallet topup
echo "      Wallet funded."

# ── 3. Deploy counter program ────────────────────────────────────────
echo "[3/5] Deploying counter program..."
DEPLOY_RESULT=$(lgs deploy --program-path "$COUNTER_BIN" --json 2>&1)
PROGRAM_ID=$(echo "$DEPLOY_RESULT" | python3 -c "import sys,json; print(json.load(sys.stdin)['program_id'])")
echo "      Program ID: $PROGRAM_ID"

# ── 4. Create counter account ────────────────────────────────────────
echo "[4/5] Creating counter account..."
COUNTER_RESULT=$(lgs wallet -- account new public 2>&1)
COUNTER_ID=$(echo "$COUNTER_RESULT" | grep -oE '[0-9a-f]{64}' | head -1)
echo "      Counter account: $COUNTER_ID"

# ── 5. Submit transactions ───────────────────────────────────────────
echo "[5/5] Submitting transactions..."

echo "      [1/2] Increment by 42..."
NSSA_WALLET_HOME_DIR="$WALLET_DIR" \
"$SPEL_CLI" \
  --idl "$IDL" \
  --program "$COUNTER_BIN" \
  -- increment \
  --counter "$COUNTER_ID" \
  --amount 42

echo "      [2/2] Reset..."
NSSA_WALLET_HOME_DIR="$WALLET_DIR" \
"$SPEL_CLI" \
  --idl "$IDL" \
  --program "$COUNTER_BIN" \
  -- reset \
  --counter "$COUNTER_ID"

echo ""
echo "================================================================"
echo " Demo complete — counter program running on LEZ with RISC0!"
echo "================================================================"
