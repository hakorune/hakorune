#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-747-MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-746-MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_call_compare_operand_forwarding_measurement_guard.sh"

[[ -f "$CARD" ]] || { echo "[mir-call-compare-measure] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mir-call-compare-measure] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mir-call-compare-measure] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mir-call-compare-measure] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mir-call-compare-measure] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mir-call-compare-measure] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-measurement-v0" \
  "source_evidence=296x-746" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "llvm_string_const_declare_deduped=1" \
  "llvm_duplicate_declare_error_fixed=1" \
  "hako_runner_summary=ok" \
  "c_runner_summary=ok" \
  "pair_adapter_summary=ok" \
  "hako_body_timing_available=1" \
  "c_body_timing_available=1" \
  "hako_body_timing_repeat_kind=workload-body-env-now-ms-v0" \
  "c_body_timing_repeat_kind=workload-body-monotonic-v0" \
  "in_process_operation_repeat=8192" \
  "allocation_count=524288" \
  "free_count=524288" \
  "hako_body_elapsed_ns=365000000" \
  "c_body_elapsed_ns=3727908" \
  "body_elapsed_ratio=97.910" \
  "winner_claim=0" \
  "next_optimization_allowed=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001:" "$CARD" || {
  echo "[mir-call-compare-measure] next owner refresh is not documented" >&2
  exit 1
}

echo "[mir-call-compare-measure] ok"
