#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-phase-state-aot-call-blocker-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-phase-state-aot-call-blocker-v0.json"
PHASE_STATE_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PHASE_STATE_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-programjson-phase-state-aot-blocker.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/phase_state_parse_probe.hako"
EXE="$TMP_DIR/phase_state_parse_probe.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])

if fixture.get("kind") != "MirBuilderProgramJsonLayer4PhaseStateAotCallBlockerV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-PHASE-STATE-AOT-CALL-BLOCKER-001":
    raise SystemExit("bad fixture token")

blocker = fixture.get("blocker") or {}
if blocker.get("callee_symbol") != "ProgramJsonV0PhaseStateBox.parse/2":
    raise SystemExit("bad blocker callee")
if blocker.get("reason") != "missing_multi_function_emitter":
    raise SystemExit("bad blocker reason")

decision = fixture.get("decision") or {}
if decision.get("selected_next_card") != "HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001":
    raise SystemExit("bad selected next card")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

source = "\n".join([
    "using selfhost.shared.common.box_helpers as BoxHelpers",
    "using lang.compiler.mirbuilder.program_json_v0_phase_state_box as ProgramJsonV0PhaseStateBox",
    "",
    "static box Main {",
    "  main() {",
    "    local out = ProgramJsonV0PhaseStateBox.parse(",
    "      \"{\\\"version\\\":0,\\\"kind\\\":\\\"Program\\\",\\\"type\\\":\\\"Program\\\",\\\"body\\\":[]}\",",
    "      \"[test]\"",
    "    )",
    "    print(\"err=\" + (\"\" + BoxHelpers.map_get(out, \"err\")))",
    "    return 0",
    "  }",
    "}",
    "",
])
app.write_text(source, encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$PHASE_STATE_IMPL" >/dev/null

set +e
NYASH_LLVM_ROUTE_TRACE=1 bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  guard_fail "$TAG" "PhaseState parse AOT probe unexpectedly emitted; update task-order and re-enable layer4 parity"
fi

grep -q "callee_symbol=ProgramJsonV0PhaseStateBox.parse/2" "$EMIT_LOG" \
  || guard_fail "$TAG" "missing expected callee_symbol in AOT blocker log"
grep -q "reason=missing_multi_function_emitter" "$EMIT_LOG" \
  || guard_fail "$TAG" "missing expected missing_multi_function_emitter reason"
grep -q "first_op=mir_call" "$EMIT_LOG" \
  || guard_fail "$TAG" "missing expected mir_call first_op"

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-phase-state-aot-call-blocker-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-PHASE-STATE-AOT-CALL-BLOCKER-001
blocked_callee=ProgramJsonV0PhaseStateBox.parse/2
owner_hint=backend_lowering
reason=missing_multi_function_emitter
first_op=mir_call
selected_next_card=HAKO-AOT-PROGRAMJSON-PHASE-STATE-PARSE-AOT-CALL-READINESS-001
resume_after_green=MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
layer4_recipe_dto_parity_green=0
phase_state_aot_call_fixed=0
source_selfhost_claim=0
summary=ok
REPORT
