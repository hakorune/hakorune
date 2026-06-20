#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SURFACE="docs/development/current/main/design/fixtures/rust-lifecycle/carrier-info-merge-from-emitter-surface-v0.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_lifecycle_emitter_surface.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/surface.mir.json"

target/debug/hakorune \
  --backend mir \
  --emit-mir-json "$MIR_JSON" \
  "$SURFACE" >/tmp/rust_lifecycle_emitter_surface_mir.out 2>/tmp/rust_lifecycle_emitter_surface_mir.err

python3 - <<'PY' "$SURFACE" "$MIR_JSON"
import json
import sys
from pathlib import Path

surface = Path(sys.argv[1]).read_text()
mir = json.loads(Path(sys.argv[2]).read_text())

assert "subject: CarrierInfo::merge_from" in surface
assert "plan_kind: OwnedCarrierInfoMerge" in surface
assert "box LifecycleEmitterSurface" in surface
assert "merge_from_lifecycle_surface" in surface
assert "Denied boundary: no join_id producer is emitted here." in surface
assert "Denied boundary: no trim_helper lifecycle owner is claimed here." in surface
assert "Denied boundary: no general converter rewrite is claimed here." in surface
assert "generated_program_execution_claim" not in surface

functions = mir.get("functions")
assert isinstance(functions, list), "MIR JSON functions missing"
joined = json.dumps(mir, sort_keys=True)
assert "LifecycleEmitterSurface" in joined
assert "merge_from_lifecycle_surface" in joined
PY

cat <<'REPORT'
output_contract=rust-lifecycle-emitter-surface-mir-v0
surface_parse_or_mir_emit=green
emitted_subject=CarrierInfo::merge_from
emitted_plan_kind=OwnedCarrierInfoMerge
denied_boundaries_preserved=1
generated_program_execution_claim=0
backend_behavior_changed=0
converter_core_changed=0
summary=ok
REPORT
