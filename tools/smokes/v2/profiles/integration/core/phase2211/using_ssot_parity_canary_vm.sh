#!/usr/bin/env bash
# Phase 22.1 — Using SSOT parity canary
# Ensures that enabling HAKO_USING_SSOT yields identical resolution/output.
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

TEST_DIR="/tmp/using_ssot_parity_$$"
mkdir -p "$TEST_DIR" && cd "$TEST_DIR"

cat > nyash.toml << 'EOF'
[using.my_pkg]
path = "lib/my_pkg/"
main = "entry.hako"

[using]
paths = ["lib"]
EOF

mkdir -p lib/my_pkg
cat > lib/my_pkg/entry.hako << 'EOF'
static box MyPkg {
  value() { return "ok-ssot" }
}
EOF

cat > app.hako << 'EOF'
using my_pkg
static box Main {
  main() { print(MyPkg.value()); return 0 }
}
EOF

# Baseline
out0=$(HAKO_USING_SSOT=0 run_nyash_vm app.hako 2>&1)
# SSOT gate ON (MVP routes to same resolver and logs a tag)
out1=$(HAKO_USING_SSOT=1 run_nyash_vm app.hako 2>&1)

# Strip resolver noise; ensure payload identical
filt() { echo "$1" | sed 's/\[using\/.*/<trace>/g'; }
if [ "$(filt "$out0")" != "$(filt "$out1")" ]; then
  echo "[FAIL] using_ssot_parity: outputs differ" >&2
  echo "--- baseline ---" >&2; echo "$out0" >&2
  echo "---   ssot   ---" >&2; echo "$out1" >&2
  exit 1
fi

echo "[PASS] using_ssot_parity_canary_vm"
exit 0
