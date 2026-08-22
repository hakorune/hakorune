#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-canonical-corridor-guard"
LLVM="$ROOT_DIR/src/runner/product/llvm/mod.rs"
OPTIMIZER="$ROOT_DIR/src/mir/optimizer/core.rs"
SCHEDULE="$ROOT_DIR/src/mir/passes/callsite_canonicalize/schedule.rs"
REJECT="$ROOT_DIR/src/mir/contracts/backend_core_ops/allowlists.rs"
JSON="$ROOT_DIR/src/runner/json_v0_bridge/core.rs"
EXEC="$ROOT_DIR/src/runner/modes/common_util/exec.rs"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require() {
  local file="$1"
  local token="$2"
  rg -F -q -- "$token" "$file" || fail "missing '$token' in ${file#$ROOT_DIR/}"
}

for file in "$LLVM" "$OPTIMIZER" "$SCHEDULE" "$REJECT" "$JSON" "$EXEC"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

require "$OPTIMIZER" "CallsiteCanonicalizeScheduleSite::MirOptimizerLateCallAndInline"
require "$SCHEDULE" "MirOptimizerLateCallAndInline"
require "$REJECT" "Some(\"call-missing-callee\")"
require "$LLVM" "fn reject_selected_dynamic_legacy_callsites"
require "$LLVM" "legacy_callsite_reject_code"
require "$LLVM" "boundary_executor::BoundaryExecutorBox::try_execute_selected_dynamic"
require "$JSON" "CallsiteCanonicalizeScheduleSite::ProgramJsonV0Bridge"
require "$EXEC" "project_module_to_legacy_calls"

python3 - "$LLVM" "$ROOT_DIR" <<'PY'
from pathlib import Path
import sys

llvm = Path(sys.argv[1]).read_text()
root = Path(sys.argv[2])

start = llvm.index("let mut module = if selected_dynamic")
reject = llvm.index("if let Err(error) = reject_selected_dynamic_legacy_callsites", start)
backend = llvm.index("match execute_via_harness_or_fallback", reject)
window = llvm[start:backend]

if "into_verified_module" not in window:
    raise SystemExit("selected corridor does not verify the module")
if "project_module_to_legacy_calls" in window:
    raise SystemExit("compatibility legacy projection crossed selected corridor")
if not (window.index("into_verified_module") < window.index("reject_selected_dynamic_legacy_callsites")):
    raise SystemExit("selected verification/rejection order drifted")

definition_start = llvm.index("fn reject_selected_dynamic_legacy_callsites")
definition_end = llvm.index("struct LlvmExecutionOutcome", definition_start)
definition = llvm[definition_start:definition_end]
for token in ("block.instructions", "block.terminator", "legacy_callsite_reject_code"):
    if token not in definition:
        raise SystemExit(f"selected legacy scanner lost {token}")

for relative in (
    "src/mir/optimizer/core.rs",
    "src/mir/passes/callsite_canonicalize/schedule.rs",
    "src/mir/contracts/backend_core_ops/allowlists.rs",
    "src/runner/product/llvm/mod.rs",
    "tools/checks/mir_call_canonical_corridor_guard.sh",
):
    lines = (root / relative).read_text().splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"800-line hard stop reached: {relative} ({len(lines)})")

print("[mir-call-canonical-corridor-guard] ok")
PY
