#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_core_contracts_vm.sh"

ABI_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_handle_abi_borrowed_owned_vm.sh"
RC_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_rc_insertion_entry_vm.sh"
OBS_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh"

source "$(dirname "$0")/lib/guard_common.sh"

TAG="phase29y-core-contracts-guard"

cd "$ROOT_DIR"
echo "[$TAG] checking phase29y core contracts gate wiring"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$DOC" "$GATE"
guard_require_exec_files "$TAG" "$GATE"

guard_expect_in_file "$TAG" 'phase29y_core_contracts_vm.sh' "$DOC" "doc missing core contracts gate reference"
guard_expect_in_file "$TAG" 'phase29y_core_contracts_guard.sh' "$DOC" "doc missing core contracts guard reference"

for dep in "$ABI_GATE" "$RC_GATE" "$OBS_GATE"; do
  guard_require_exec_files "$TAG" "$ROOT_DIR/$dep"
  guard_expect_in_file "$TAG" "$dep" "$DOC" "doc missing dependency gate reference: $dep"
  guard_expect_in_file "$TAG" "$dep" "$GATE" "integrated gate missing dependency step: $dep"
done

guard_expect_in_file "$TAG" 'phase29y_core_contracts_guard.sh' "$GATE" "gate missing guard precondition step"
guard_expect_in_file "$TAG" 'NYASH_GC_METRICS=0' "$ROOT_DIR/$OBS_GATE" "observability smoke missing metrics-OFF baseline pin"
guard_expect_in_file "$TAG" 'NYASH_GC_METRICS=1' "$ROOT_DIR/$OBS_GATE" "observability smoke missing metrics-ON diagnostic pin"
guard_expect_in_file "$TAG" 'gc/optional:mode' "$ROOT_DIR/$OBS_GATE" "observability smoke missing optional GC stable tag assertion"

echo "[$TAG] ok"
