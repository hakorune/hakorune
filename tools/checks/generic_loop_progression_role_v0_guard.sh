#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="generic-loop-progression-role-v0"
BASELINE_TEST="progression_role_baseline_tests"
CONTRACT_PIN="tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh"
LOG="/tmp/${TAG}.contract-pin.log"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" timeout
guard_require_files "$TAG" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/extract/progression_role_baseline_tests.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/generic_loop/facts/extract/test_support.rs" \
  "$ROOT/$CONTRACT_PIN"

cd "$ROOT"
cargo test -q "$BASELINE_TEST" --lib

set +e
timeout 180s bash "$CONTRACT_PIN" >"$LOG" 2>&1
pin_rc=$?
set -e

if [ "$pin_rc" -eq 0 ]; then
  guard_fail "$TAG" "A0 expected the clean-HEAD contract pin to expose the known progression-role blocker"
fi
if [ "$pin_rc" -eq 124 ]; then
  tail -n 120 "$LOG" >&2
  guard_fail "$TAG" "contract pin exceeded 180 seconds"
fi

rg -q "ParserDelegateExposesBox\._parse_delegate/3|FuncScannerBox\._scan_methods/4" "$LOG" \
  || {
    tail -n 120 "$LOG" >&2
    guard_fail "$TAG" "known real-source cursor owner was not observed"
  }
rg -q "loop var used after in-body step|no valid loop_var candidates found" "$LOG" \
  || {
    tail -n 120 "$LOG" >&2
    guard_fail "$TAG" "known progression-role rejection was not observed"
  }

python3 - "$ROOT" <<'PY'
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
paths = [
    root / "src/mir/builder/control_flow/plan/generic_loop/facts/extract/progression_role_baseline_tests.rs",
    root / "src/mir/builder/control_flow/plan/generic_loop/facts/extract/test_support.rs",
]
for path in paths:
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} has {lines}")

fixture = paths[0].read_text(encoding="utf-8")
for needle in (
    "delegate_style_cursor_records_no_accepted_progression_role",
    "scanner_style_cursor_records_no_condition_anchored_candidate",
    "body-only state cursor must not become an unrestricted candidate",
):
    if needle not in fixture:
        raise SystemExit(f"missing A0 progression-role fixture: {needle}")

print("a0_structural_fixtures=green")
print("clean_head_contract_pin=known_red")
print("parser_source_change=0")
print("summary=ok")
PY

echo "[$TAG] ok"
