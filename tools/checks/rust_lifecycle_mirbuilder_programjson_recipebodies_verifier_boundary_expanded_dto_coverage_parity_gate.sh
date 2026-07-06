#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
DECISION_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_post_verifier_boundary_decision_guard.sh"
IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipebodies_verifier_boundary_snapshot.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$FIXTURE" "$DECISION_GUARD" "$IMPL" "$HAKO_BIN"

DECISION_OUT="$("$DECISION_GUARD")"
if ! grep -q '^selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001$' <<<"$DECISION_OUT"; then
  guard_fail "$TAG" "post-verifier decision does not select expanded DTO coverage"
fi

TMP_DIR="$(mktemp -d /tmp/hakorune-recipebodies-verifier-expanded.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipebodies_verifier_expanded_probe.hako"
MIR_JSON="$TMP_DIR/recipebodies_verifier_expanded_probe.mir.json"
EXE="$TMP_DIR/recipebodies_verifier_expanded_probe.exe"
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
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesVerifierBoundaryExpandedDtoCoverageParityV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001":
    raise SystemExit("bad fixture token")
if fixture.get("acceptance", {}).get("expanded_dto_coverage_rows") != 2:
    raise SystemExit("expanded row count drift")

claims = fixture.get("claims") or {}
if claims.get("expanded_dto_coverage_parity_green") != 1:
    raise SystemExit("expanded coverage claim missing")
for key, value in claims.items():
    if key == "expanded_dto_coverage_parity_green":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

rows = fixture.get("rows") or []
if len(rows) != 2:
    raise SystemExit("expanded DTO coverage requires exactly 2 rows")

lines = [
    "using lang.compiler.mirbuilder.program_json_recipebodies_verifier_boundary_snapshot as ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox",
    "",
    "static box Main {",
    "  main() {",
]
expected_lines = []
for row in rows:
    row_id = row["row_id"]
    summary = row["rust_oracle_expected_summary"]
    for token in [
        "snapshot_kind=ProgramJsonRecipeBodiesVerifierBoundarySnapshotV1;",
        ";err=0",
        ";arena_ready=1",
        ";verifier_boundary_used=1",
        ";body_count=4",
        ";def_count=1",
        ";update_count=2",
    ]:
        if token not in summary:
            raise SystemExit(f"missing expected summary token {token}: {row_id}")
    program_json = json.dumps(row["program_json"], separators=(",", ":"), ensure_ascii=False)
    lines.append(
        "    print(\"expanded:"
        + row_id
        + ":\" + ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox.build_summary("
        + json.dumps(program_json)
        + "))"
    )
    expected_lines.append("expanded:" + row_id + ":" + summary + "\n")
lines.extend(["    return 0", "  }", "}", ""])
app.write_text("\n".join(lines), encoding="utf-8")
expected.write_text("".join(expected_lines), encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$IMPL" >/dev/null

if ! timeout 90s bash "$HAKO_BIN" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" || true
  guard_fail "$TAG" "failed to emit MIR JSON for expanded DTO coverage probe"
fi

python3 - "$MIR_JSON" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
functions = data.get("functions") or []
main = next((fn for fn in functions if fn.get("name") == "main"), None)
snapshot = next((fn for fn in functions if fn.get("name") == "ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox.build_summary/1"), None)
if main is None or snapshot is None:
    raise SystemExit("required functions missing")

def route_rows(fn):
    metadata = fn.get("metadata") or {}
    rows = []
    for key in ("global_call_routes", "lowering_plan"):
        value = metadata.get(key) or []
        if isinstance(value, list):
            rows.extend(row for row in value if isinstance(row, dict))
    return rows

main_routes = [row for row in route_rows(main) if row.get("symbol") == "ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox.build_summary/1"]
if len(main_routes) < 2:
    raise SystemExit("main must call verifier-boundary snapshot for both rows")
bad = [row for row in route_rows(snapshot) if row.get("tier") == "Unsupported" or row.get("reason")]
if bad:
    raise SystemExit(f"verifier-boundary snapshot has unsupported routes: {bad[:5]}")
PY

if ! timeout --kill-after=2s 120s env \
    NYASH_LLVM_OPT_LEVEL=0 \
    HAKO_LLVM_OPT_LEVEL=0 \
    bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$BUILD_LOG" 2>&1; then
  tail -n 160 "$BUILD_LOG" >&2 || true
  guard_fail "$TAG" "emit-exe probe failed or timed out"
fi
if ! "$EXE" >"$RUN_OUT" 2>"$RUN_ERR"; then
  cat "$RUN_ERR" >&2 || true
  guard_fail "$TAG" "executable failed at runtime"
fi
grep -v '^Result: 0$' "$RUN_OUT" >"$RUN_FILTERED" || true
if ! diff -u "$EXPECTED" "$RUN_FILTERED" >/dev/null; then
  echo "[${TAG}] expected:" >&2
  cat "$EXPECTED" >&2
  echo "[${TAG}] actual:" >&2
  cat "$RUN_FILTERED" >&2
  guard_fail "$TAG" "runtime parity mismatch"
fi

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-gate-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001
owner=ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox
row_count=2
expanded_dto_coverage_rows=2
existing_snapshot_owner_reused=1
runtime_parity_green=1
runtime_recipe_bodies_publication=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
