#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-196-wsm-p10-min3-loop-extern-writer-section-lock-ssot.md"
SMOKE="tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min3_loop_extern_writer_section_lock_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"
WRITER="src/backend/wasm/binary_writer.rs"

if [ ! -f "$DOC" ]; then
  echo "[wsm-p10-min3-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for needle in "WSM-P10-min3" "Type -> Import -> Function -> Export -> Code" "env.console_log" "WSM-P10-min4"; do
  if ! rg -q "$needle" "$DOC"; then
    echo "[wsm-p10-min3-guard] missing keyword in lock doc: $needle" >&2
    exit 1
  fi
done

for needle in "build_loop_extern_call_skeleton_module" "SECTION_IMPORT" "OP_LOOP" "OP_BR_IF" "OP_CALL"; do
  if ! rg -q "$needle" "$WRITER"; then
    echo "[wsm-p10-min3-guard] writer contract missing: $needle" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_wsm_p10_loop_extern_writer_section_guard.sh" "$DEV_GATE"; then
  echo "[wsm-p10-min3-guard] dev_gate missing p10 min3 guard step" >&2
  exit 1
fi

if [ ! -x "$SMOKE" ]; then
  echo "[wsm-p10-min3-guard] missing or not executable: $SMOKE" >&2
  exit 1
fi

bash "$SMOKE"
echo "[wsm-p10-min3-guard] ok"
