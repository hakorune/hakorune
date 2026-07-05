#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-heavy-exe-readiness-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/lib/ffi_contract.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-loop-recipe-dto-parity-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3120-HAKO-AOT-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-HEAVY-EXE-READINESS-001.md"
COMMON_INC="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_common.inc"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$COMMON_INC" "$HAKO_BIN"

guard_expect_fixed_in_file "$TAG" 'return "mem2reg";' "$COMMON_INC" "O0 opt path must stay mem2reg-only"
guard_expect_fixed_in_file "$TAG" 'always-inline,default<O3>' "$COMMON_INC" "O3 opt path must remain explicit only for opt level 3"
guard_expect_fixed_in_file "$TAG" '3109 scanner result-map contract guard = green' "$CARD" "3120 must require 3109 scanner contract"
guard_expect_fixed_in_file "$TAG" 'void_signature_object_return_widening = 0' "$CARD" "3120 must not widen void object returns"

ffi_contract_ensure_fresh "$ROOT_DIR" >/dev/null

TMP_DIR="$(mktemp -d /tmp/hakorune-layer4-loop-recipe-dto-heavy.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/layer4_loop_recipe_dto_heavy_probe.hako"
EXE="$TMP_DIR/layer4_loop_recipe_dto_heavy_probe.exe"
EXPECTED="$TMP_DIR/expected.txt"
BUILD_LOG="$TMP_DIR/build.log"
RUN_OUT="$TMP_DIR/run.out"
RUN_FILTERED="$TMP_DIR/run.filtered"
RUN_ERR="$TMP_DIR/run.err"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "MirBuilderProgramJsonLayer4LoopRecipeDtoParityV1":
    raise SystemExit("bad fixture kind")
rows = fixture.get("rows") or []
if len(rows) < 4:
    raise SystemExit("expected at least four Layer4 DTO rows")

lines = [
    "using lang.compiler.mirbuilder.program_json_loop_recipe_dto_snapshot as ProgramJsonLoopRecipeDtoSnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected = []
for row in rows:
    row_id = row["row_id"]
    prefix = f"dto:{row_id}:"
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    lines.append(
        "    print("
        + json.dumps(prefix)
        + " + ProgramJsonLoopRecipeDtoSnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected.append(prefix + row["rust_oracle_expected_summary"])
lines.extend(["    return 0", "  }", "}", ""])

Path(sys.argv[2]).write_text("\n".join(lines), encoding="utf-8")
Path(sys.argv[3]).write_text("\n".join(expected) + "\n", encoding="utf-8")
PY

if ! timeout --kill-after=2s 120s env \
    NYASH_LLVM_OPT_LEVEL=0 \
    HAKO_LLVM_OPT_LEVEL=0 \
    bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "heavy emit-exe probe failed or timed out"
fi

if ! "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"; then
  cat "$RUN_ERR" >&2 || true
  guard_fail "$TAG" "heavy executable failed at runtime"
fi
grep -v '^Result: 0$' "$RUN_OUT" >"$RUN_FILTERED" || true

if diff -u "$EXPECTED" "$RUN_FILTERED" >/dev/null; then
  runtime_parity_green=1
  exact_first_blocker=none
else
  runtime_parity_green=0
  exact_first_blocker="$(python3 - "$RUN_FILTERED" <<'PY'
import sys
from pathlib import Path

lines = [line for line in Path(sys.argv[1]).read_text(encoding="utf-8").splitlines() if line]
if lines and all("snapshot_kind=LoopRecipeDtoSnapshotV1;err=1;reason=parse_error" in line for line in lines):
    print("phase_state_parse_runtime_parse_error")
else:
    print("unexpected_runtime_parity_mismatch")
PY
)"
  if [[ "$exact_first_blocker" != "phase_state_parse_runtime_parse_error" ]]; then
    echo "[${TAG}] expected:" >&2
    cat "$EXPECTED" >&2
    echo "[${TAG}] actual:" >&2
    cat "$RUN_FILTERED" >&2
    guard_fail "$TAG" "$exact_first_blocker"
  fi
fi

cat <<REPORT
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-heavy-exe-readiness-gate-v0
token=HAKO-AOT-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-HEAVY-EXE-READINESS-001
heavy_emit_exe_probe=green
ffi_o0_opt_pass=mem2reg
runtime_parity_green=$runtime_parity_green
exact_first_blocker=$exact_first_blocker
void_signature_object_return_widening=0
mixed_runtime_i64_or_handle_for_scanner_out_map=0
source_selfhost_claim=0
summary=ok
REPORT
