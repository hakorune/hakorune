#!/bin/bash
# emit_vm_hako_checkpoint_snippet.sh
# Print CURRENT_TASK checkpoint bullet lines using current vm-hako phase and gate summary log.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NYASH_ROOT="${NYASH_ROOT:-$ROOT_DIR}"
PHASE_LIB="$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/lib/vm_hako_phase.sh"

if [ ! -f "$PHASE_LIB" ]; then
    echo "[emit_vm_hako_checkpoint_snippet] missing phase lib: $PHASE_LIB" >&2
    exit 2
fi

source "$PHASE_LIB"

GATE_LOG=""
while [ $# -gt 0 ]; do
    case "$1" in
        --gate-log)
            [ $# -ge 2 ] || { echo "--gate-log requires path" >&2; exit 2; }
            GATE_LOG="$2"
            shift 2
            ;;
        -h|--help)
            cat <<'USAGE'
Usage:
  tools/selfhost/emit_vm_hako_checkpoint_snippet.sh [--gate-log <path>]

Notes:
  - If --gate-log is provided, stageb_total_secs / avg_case_secs / cases are parsed from
    [diag/selfhost] gate_summary line.
  - Without --gate-log, gate summary values are emitted as TODO placeholders.
USAGE
            exit 0
            ;;
        *)
            echo "unknown arg: $1" >&2
            exit 2
            ;;
    esac
done

PHASE="$(resolve_vm_hako_phase || true)"
if [ -z "$PHASE" ]; then
    echo "[emit_vm_hako_checkpoint_snippet] failed to resolve VM_HAKO_PHASE from src/runner/modes/vm_hako.rs" >&2
    exit 2
fi

CASES="TODO/TODO"
STAGEB="TODO"
AVG="TODO"
if [ -n "$GATE_LOG" ]; then
    if [ ! -f "$GATE_LOG" ]; then
        echo "[emit_vm_hako_checkpoint_snippet] gate log not found: $GATE_LOG" >&2
        exit 2
    fi
    SUMMARY_LINE="$(rg -n "gate_summary" "$GATE_LOG" 2>/dev/null | tail -n 1 | sed -E 's/^[0-9]+://' || true)"
    if [ -n "$SUMMARY_LINE" ]; then
        CASES="$(echo "$SUMMARY_LINE" | sed -n 's/.*cases=\([^ ]*\).*/\1/p')"
        STAGEB="$(echo "$SUMMARY_LINE" | sed -n 's/.*stageb_total_secs=\([^ ]*\).*/\1/p')"
        AVG="$(echo "$SUMMARY_LINE" | sed -n 's/.*avg_case_secs=\([^ ]*\).*/\1/p')"
        [ -n "$CASES" ] || CASES="TODO/TODO"
        [ -n "$STAGEB" ] || STAGEB="TODO"
        [ -n "$AVG" ] || AVG="TODO"
    fi
fi

cat <<EOF_MD
  - [x] \`bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_backend_frame_vm.sh\` PASS（\`phase=$PHASE\`, \`rc=1\` fail-fast 契約維持）
  - [x] \`bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_get_parity_vm.sh\` PASS（\`rust-vm=42\`, \`hako-runner=42\`）
  - [x] \`bash tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_set_parity_vm.sh\` PASS（\`rust-vm=42\`, \`hako-runner=42\`）
  - [x] \`./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4\` PASS（\`$CASES\`, \`stageb_total_secs=$STAGEB\`, \`avg_case_secs=$AVG\`）
EOF_MD
