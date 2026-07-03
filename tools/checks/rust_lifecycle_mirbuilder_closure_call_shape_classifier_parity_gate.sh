#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-closure-call-shape-classifier-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-closure-call-shape-classifier-rust-oracle-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/lib/closure_call_shape_classifier.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-closure-call-shape-parity.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR" >/dev/null 2>&1 || true
}
trap cleanup EXIT

APP="$TMP_DIR/closure_call_shape_classifier_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/closure_call_shape_classifier_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

rows = fixture["rows"]
expected.write_text(
    "\n".join(
        f"{row['expected_shape']}|{row['expected_reject_code']}" for row in rows
    ) + "\n",
    encoding="utf-8",
)

calls = []
for idx, row in enumerate(rows):
    dst_present = json.dumps(row["dst_present"])
    arg_count = json.dumps(row["arg_count"])
    shape_var = f"shape{idx}"
    calls.extend(
        [
            "    local " + shape_var + " = ClosureCallShapeClassifierBox.classify_shape("
            + dst_present + ", " + arg_count + ")",
            "    print(" + shape_var + " + \"|\" + ClosureCallShapeClassifierBox.reject_code("
            + shape_var + "))",
        ]
    )

source = "\n".join(
    [
        "using lang.compiler.lib.closure_call_shape_classifier as ClosureCallShapeClassifierBox",
        "",
        "static box Main {",
        "  main() {",
        *calls,
        "    return 0",
        "  }",
        "}",
        "",
    ]
)
app.write_text(source, encoding="utf-8")
PY

bash "$HAKO_BIN" --backend mir --verify "$HAKO_IMPL" >/dev/null

if ! bash "$HAKO_BIN" --backend mir --emit-exe "$EXE" "$APP" >"$EMIT_LOG" 2>&1; then
  tail -n 120 "$EMIT_LOG" || true
  guard_fail "$TAG" "failed to emit parity executable"
fi

chmod +x "$EXE"
"$EXE" >"$ACTUAL.raw"

python3 - "$EXPECTED" "$ACTUAL.raw" "$ACTUAL" <<'PY'
import sys
from pathlib import Path

expected = Path(sys.argv[1]).read_text(encoding="utf-8").splitlines()
raw = Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
actual_path = Path(sys.argv[3])

actual = [
    line.strip()
    for line in raw
    if line.strip() and not line.startswith("Result:")
]
actual_path.write_text("\n".join(actual) + "\n", encoding="utf-8")

if actual != expected:
    print("[closure-call-shape/parity] mismatch")
    max_len = max(len(expected), len(actual))
    for idx in range(max_len):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp} actual={got}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-closure-call-shape-classifier-parity-gate-v0
owner=closure_call_shape_classifier
rust_oracle_fixture=mirbuilder-closure-call-shape-classifier-rust-oracle-v0.json
hako_implementation=lang/src/compiler/lib/closure_call_shape_classifier.hako
parity_rows=4
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
callsite_canonicalization_migration=0
new_closure_rewrite_migration=0
backend_fail_fast_boundary_migration=0
mir_instruction_mutation_migration=0
summary=ok
REPORT
