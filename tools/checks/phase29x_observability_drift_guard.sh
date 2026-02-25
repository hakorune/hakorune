#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CATEGORIES="$ROOT_DIR/tools/checks/phase29x_observability_categories.txt"
TRACKER="$ROOT_DIR/src/runtime/leak_tracker.rs"
SUMMARY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-88-observability-drift-guard-ssot.md"
GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh"

cd "$ROOT_DIR"

echo "[observability-drift-guard] checking X61 observability drift wiring"

if ! command -v rg >/dev/null 2>&1; then
  echo "[observability-drift-guard] ERROR: rg is required" >&2
  exit 2
fi

for required in "$CATEGORIES" "$TRACKER" "$SUMMARY_SMOKE" "$DOC"; do
  if [[ ! -f "$required" ]]; then
    echo "[observability-drift-guard] ERROR: required file missing: $required"
    exit 1
  fi
done

if [[ ! -x "$GATE" ]]; then
  echo "[observability-drift-guard] ERROR: gate missing or not executable: $GATE"
  exit 1
fi

if ! rg -q '^Decision: accepted$' "$DOC"; then
  echo "[observability-drift-guard] ERROR: X61 SSOT decision drift (expected Decision: accepted)"
  exit 1
fi

EXPECTED=(handles locals temps heap_fields singletons)
mapfile -t ACTUAL < <(grep -v '^[[:space:]]*#' "$CATEGORIES" | sed '/^[[:space:]]*$/d')

if [[ "${#ACTUAL[@]}" -ne 5 ]]; then
  echo "[observability-drift-guard] ERROR: category inventory count drift (expected=5 got=${#ACTUAL[@]})"
  exit 1
fi

UNIQUE_COUNT="$(printf '%s\n' "${ACTUAL[@]}" | sort -u | wc -l | tr -d ' ')"
if [[ "$UNIQUE_COUNT" -ne 5 ]]; then
  echo "[observability-drift-guard] ERROR: category inventory contains duplicate entries"
  exit 1
fi

for i in "${!EXPECTED[@]}"; do
  if [[ "${ACTUAL[$i]}" != "${EXPECTED[$i]}" ]]; then
    echo "[observability-drift-guard] ERROR: category order drift at index $i (expected='${EXPECTED[$i]}' got='${ACTUAL[$i]}')"
    exit 1
  fi
done

for cat in "${EXPECTED[@]}"; do
  if ! rg -Fq "[lifecycle/leak]   ${cat}:" "$TRACKER"; then
    echo "[observability-drift-guard] ERROR: leak_tracker output missing category line: $cat"
    exit 1
  fi
  if ! rg -Fq "$cat" "$DOC"; then
    echo "[observability-drift-guard] ERROR: SSOT missing category: $cat"
    exit 1
  fi
done

if ! rg -q 'phase29x_observability_categories.txt' "$SUMMARY_SMOKE"; then
  echo "[observability-drift-guard] ERROR: summary smoke missing category inventory binding"
  exit 1
fi
if ! rg -q 'phase29x_rc_phase2_queue_lock_vm.sh' "$GATE"; then
  echo "[observability-drift-guard] ERROR: gate missing X60 precondition step"
  exit 1
fi
for smoke in \
  phase29x_observability_temps_vm.sh \
  phase29x_observability_heap_fields_vm.sh \
  phase29x_observability_singletons_vm.sh \
  phase29x_observability_summary_vm.sh
do
  if ! rg -q "$smoke" "$GATE"; then
    echo "[observability-drift-guard] ERROR: gate missing observability replay step: $smoke"
    exit 1
  fi
done

echo "[observability-drift-guard] ok"
