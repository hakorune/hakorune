#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-global-target-b1-static-method-s0"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-global-target-b1-static-method-s0-2026-08-28.toml"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
CARRIER="$ROOT_DIR/crates/hakorune_mir_defs/src/global_target.rs"
CALL_DEFS="$ROOT_DIR/crates/hakorune_mir_defs/src/call_unified.rs"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for file in "$CARD" "$STATE" "$MANIFEST" "$CARRIER" "$CALL_DEFS"; do
  [[ -f "$file" ]] || fail "missing owner $file"
done

python3 - "$ROOT_DIR" "$CARD" "$STATE" "$MANIFEST" "$CARRIER" "$CALL_DEFS" <<'PY'
from pathlib import Path
import re
import sys
import tomllib

root, card_path, state_path, manifest_path, carrier_path, call_defs_path = map(Path, sys.argv[1:])

def load(path):
    with path.open("rb") as stream:
        return tomllib.load(stream)

card = load(card_path)
state = load(state_path)
manifest = load(manifest_path)
carrier = carrier_path.read_text()
call_defs = call_defs_path.read_text()

row_id = "mir-call-global-target-b1-static-method-s0"
expected_allowed = {
    "crates/hakorune_mir_defs/src/lib.rs",
    "crates/hakorune_mir_defs/src/global_target.rs",
    "tools/checks/b1_global_target_static_method_s0_guard.sh",
    "tools/checks/guard_rows.toml",
    "docs/development/current/main/investigations/mir-call-global-target-b1-static-method-s0-2026-08-28.toml",
    "docs/development/current/main/CURRENT_STATE.toml",
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
}

status = card.get("status")
if status not in {"fast_open", "landed", "landed_bounded_child_row"}:
    raise SystemExit(f"B1 S0 card has unsupported status: {status!r}")
if status == "fast_open" and card.get("implementation_permission") is not True:
    raise SystemExit("open B1 S0 card does not have scoped implementation permission")
if status in {"landed", "landed_bounded_child_row"}:
    if card.get("implementation_permission") is not False:
        raise SystemExit("landed B1 S0 card still has implementation permission")
    if not card.get("landed_commit"):
        raise SystemExit("landed B1 S0 card is missing landed_commit")
    if card.get("completed_execution_row") != "MIR-CALL-GLOBAL-TARGET-B1-STATIC-METHOD-S0":
        raise SystemExit("landed B1 S0 card does not record completion")
if card.get("guard_phase") != "b1_carrier_s0":
    raise SystemExit("B1 S0 guard phase drifted")
if card.get("execution_row") != "MIR-CALL-GLOBAL-TARGET-B1-STATIC-METHOD-S0":
    raise SystemExit("B1 S0 execution row drifted")
allowed = set(card.get("allowed_files", {}).get("paths", []))
if allowed != expected_allowed:
    raise SystemExit(f"B1 S0 allowed file boundary drifted: {sorted(allowed)}")

if status == "fast_open":
    if state.get("work_mode") != "fast":
        raise SystemExit("CURRENT_STATE work_mode is not fast")
    if state.get("current_execution_row") != card["execution_row"]:
        raise SystemExit("CURRENT_STATE current row does not select B1 S0")
    if state.get("next_execution_card") != card["execution_row"]:
        raise SystemExit("CURRENT_STATE next execution card does not select B1 S0")
    if state.get("latest_card_path") != str(card_path.relative_to(root)):
        raise SystemExit("CURRENT_STATE latest card path does not select B1 S0")

rows = manifest.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard_rows.toml rows table is missing")
matches = [row for row in rows if row.get("id") == row_id]
if len(matches) != 1:
    raise SystemExit(f"expected exactly one registry row for {row_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    raise SystemExit("B1 S0 guard profiles drifted")
if row.get("cmd") != ["bash", "tools/checks/b1_global_target_static_method_s0_guard.sh"]:
    raise SystemExit("B1 S0 guard command drifted")

if sum(1 for _ in carrier_path.open()) >= 760:
    raise SystemExit("carrier source reached the 760-line split threshold")
for token in (
    "CanonicalGlobalTargetV1",
    "CanonicalGlobalTargetV1::Builtin",
    "CanonicalGlobalTargetV1::SameModule",
    "CanonicalSameModuleGlobalTargetV1::FreeFunction",
    "CanonicalSameModuleGlobalTargetV1::StaticBoxMethod",
    "pub fn new_static_box_method",
    "pub fn new_free_function",
    "pub const fn builtin_print",
):
    if token not in carrier:
        raise SystemExit(f"carrier shape/constructor token missing: {token}")

for pattern in (
    r"impl\s+From\s*<",
    r"From\s*<\s*(?:String|&str)",
    r"\bparse\s*\(",
    r"\bto_string\s*\(",
    r"as_mir_name",
    r"impl\s+.*Display",
    r"\bDefault\b",
    r"\b(?:Unknown|Legacy)\b",
    r"serde",
    r"\bCallee\b",
    r"\bCallTarget\b",
    r"Verified[A-Za-z]*|Prepared[A-Za-z]*",
    r"#\s*\[\s*path\s*=",
):
    if re.search(pattern, carrier):
        raise SystemExit(f"forbidden carrier API/token found: {pattern}")

# The carrier-only row must not create a disconnected production issuer.
constructor_pattern = re.compile(
    r"CanonicalGlobalTargetV1::(?:new_static_box_method|new_free_function|builtin_print|Builtin|SameModule)"
)
hits = []
for path in root.rglob("*.rs"):
    if path == carrier_path or "tests" in path.parts or path.name.endswith("_tests.rs"):
        continue
    for line_no, line in enumerate(path.read_text().splitlines(), 1):
        if constructor_pattern.search(line):
            hits.append(f"{path.relative_to(root)}:{line_no}")
if hits:
    raise SystemExit("production carrier constructors/literals appeared in S0: " + ", ".join(hits))

# S0 must leave the transitional Call representation untouched.
if "Global(String)" not in call_defs:
    raise SystemExit("Callee::Global(String) was changed during carrier-only S0")
if "Global(CanonicalGlobalTargetV1)" in call_defs:
    raise SystemExit("canonical Callee payload changed during carrier-only S0")

print(f"[{row_id}] carrier-only shape, API, and zero-production-caller checks ok")
PY

echo "[$TAG] ok"
