#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2
preflight_plugins >/dev/null || exit 2

TEST_DIR="/tmp/using_ssot_hako_parity_$$"
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

out0=$(HAKO_USING_SSOT=0 run_nyash_vm app.hako 2>&1)
out1=$(HAKO_USING_SSOT=1 HAKO_USING_SSOT_HAKO=1 run_nyash_vm app.hako 2>&1)

filt() { echo "$1" | sed 's/\[using\/.*/<trace>/g'; }
if [ "$(filt "$out0")" != "$(filt "$out1")" ]; then
  echo "[FAIL] using_ssot_hako_parity" >&2
  echo "--- baseline ---" >&2; echo "$out0" >&2
  echo "---   ssot+hako ---" >&2; echo "$out1" >&2
  exit 1
fi
echo "[PASS] using_ssot_hako_parity_canary_vm"
exit 0

