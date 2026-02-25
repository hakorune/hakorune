#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2
preflight_plugins >/dev/null || exit 2

TEST_DIR="/tmp/ssot_rel_unique_$$"
mkdir -p "$TEST_DIR" && cd "$TEST_DIR"

cat > nyash.toml << 'EOF'
[using]
paths = ["libA"]
EOF

mkdir -p libA
cat > libA/mypkg.hako << 'EOF'
static box Pkg { value() { return "one" } }
EOF

cat > app.hako << 'EOF'
using mypkg
static box Main { main() { print(Pkg.value()); return 0 } }
EOF

out=$(HAKO_USING_SSOT=1 HAKO_USING_SSOT_RELATIVE=1 run_nyash_vm app.hako 2>&1)
if echo "$out" | grep -q '^one$'; then
  echo "[PASS] ssot_relative_unique_canary_vm"
  exit 0
fi
echo "[FAIL] ssot_relative_unique_canary_vm" >&2
echo "$out" >&2
exit 1

