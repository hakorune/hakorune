#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env >/dev/null || exit 2
preflight_plugins >/dev/null || exit 2

TEST_DIR="/tmp/ssot_rel_amb_strict_$$"
mkdir -p "$TEST_DIR" && cd "$TEST_DIR"

cat > nyash.toml << 'EOF'
[using]
paths = ["libA", "libB"]
EOF

mkdir -p libA libB
cat > libA/mypkg.hako << 'EOF'
static box Pkg { value() { return "A" } }
EOF
cat > libB/mypkg.hako << 'EOF'
static box Pkg { value() { return "B" } }
EOF

cat > app.hako << 'EOF'
using mypkg
static box Main { main() { print(Pkg.value()); return 0 } }
EOF

# Strictモードで曖昧候補→レガシーに委譲→エラー期待（rc != 0）
set +e
HAKO_USING_SSOT=1 HAKO_USING_SSOT_RELATIVE=1 NYASH_USING_STRICT=1 run_nyash_vm app.hako >/tmp/ssot_rel_amb_strict.out 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[PASS] ssot_relative_ambiguous_strict_canary_vm"
  exit 0
fi
echo "[FAIL] ssot_relative_ambiguous_strict_canary_vm (rc=$rc)" >&2
sed -n '1,120p' /tmp/ssot_rel_amb_strict.out >&2
exit 1

