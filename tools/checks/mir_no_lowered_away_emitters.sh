#!/usr/bin/env bash
# Ensure lowered-away MIR instructions are not directly emitted from production code.
#
# Contract:
# - Direct constructors for lowered-away variants must not appear in production emit paths.
# - Test fixtures may construct them intentionally (allowlisted below).
#
# This check intentionally targets constructor-style patterns (add_instruction / set_terminator /
# direct assignment) to avoid false positives from pattern matches in verifier/normalizer code.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DISALLOWED_VARIANTS="ArrayGet|ArraySet|RefGet|RefSet"

# Known fixture files that intentionally construct lowered-away ops for contract tests.
ALLOWLIST_RE='^(src/mir/optimizer.rs|src/mir/instruction/tests.rs|src/backend/mir_interpreter/exec/diagnostics.rs|src/mir/contracts/backend_core_ops.rs):'

collect_hits() {
  local pattern="$1"
  rg -n "$pattern" src -S || true
}

HITS="$(
  {
    collect_hits "add_instruction\\(MirInstruction::(${DISALLOWED_VARIANTS})\\s*\\{"
    collect_hits "set_terminator\\(MirInstruction::(${DISALLOWED_VARIANTS})\\s*\\{"
    collect_hits "=\\s*MirInstruction::(${DISALLOWED_VARIANTS})\\s*\\{"
  } | grep -Ev "$ALLOWLIST_RE" || true
)"

if [ -n "$HITS" ]; then
  echo "[mir-lowered-away-emitters] FAIL: found disallowed lowered-away constructors in production paths" >&2
  echo "$HITS" >&2
  exit 1
fi

echo "[mir-lowered-away-emitters] OK"
