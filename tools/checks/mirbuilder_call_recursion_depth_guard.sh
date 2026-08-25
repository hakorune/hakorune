#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

python3 - <<'PY'
from pathlib import Path

checks = {
    Path("src/mir/builder/calls/build.rs"): (
        "fn build_method_call_from_input_with_route_v1",
        "fn build_method_call_impl_with_route_v1",
        "self.recursion_depth -= 1;\n            return Err(error);",
        "return Err(format!(",
    ),
    Path("src/mir/builder/calls/unified_emitter.rs"): (
        "fn emit_unified_call_outcome_with_lookup_and_map_replay",
        "// Check environment variable for unified call usage",
        "builder.recursion_depth -= 1;\n            return Err(error);",
        "return Err(UnifiedCallAttemptErrorV1::Emission(format!(",
    ),
}

for path, (start, end, restore, legacy_return) in checks.items():
    text = path.read_text()
    lines = text.splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"[mirbuilder-call-recursion-depth] hard stop: {path} has {len(lines)} lines")
    start_at = text.index(start)
    window = text[start_at : text.index(end, start_at)]
    if restore not in window:
        raise SystemExit(f"[mirbuilder-call-recursion-depth] missing overflow restore in {path}")
    if legacy_return in window:
        raise SystemExit(f"[mirbuilder-call-recursion-depth] legacy direct overflow return remains in {path}")

print("[mirbuilder-call-recursion-depth] two overflow owners restore before returned Err; source limits pass")
PY
