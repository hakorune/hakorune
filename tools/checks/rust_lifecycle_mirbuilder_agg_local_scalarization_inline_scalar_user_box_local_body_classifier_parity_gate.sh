#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-rust-oracle-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/lib/agg_local_scalarization_inline_scalar_user_box_local_body_classifier.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-agg-local-scalarization-inline-scalar-user-box-local-body-classifier.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR" >/dev/null 2>&1 || true
}
trap cleanup EXIT

APP="$TMP_DIR/agg_local_scalarization_inline_scalar_user_box_local_body_classifier_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/agg_local_scalarization_inline_scalar_user_box_local_body_classifier_parity.exe"
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
    "\n".join(row["expected"] for row in rows) + "\n",
    encoding="utf-8",
)

calls = []
for row in rows:
    calls.append(
        "    print(AggLocalScalarizationInlineScalarUserBoxLocalBodyClassifierBox."
        "is_inline_scalar_user_box_local_body("
        f"{json.dumps(row['surface'])}, {json.dumps(row['selected_entry'])}, {json.dumps(row['value_class'])}))"
    )

source = "\n".join(
    [
        "using lang.compiler.lib.agg_local_scalarization_inline_scalar_user_box_local_body_classifier as AggLocalScalarizationInlineScalarUserBoxLocalBodyClassifierBox",
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
    print("[agg-local-scalarization-inline-scalar-user-box-local-body-classifier/parity] mismatch")
    max_len = max(len(expected), len(actual))
    for idx in range(max_len):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp} actual={got}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-parity-gate-v0
owner=agg_local_scalarization_inline_scalar_user_box_local_body_classifier
rust_oracle_fixture=mirbuilder-agg-local-scalarization-inline-scalar-user-box-local-body-classifier-rust-oracle-v0.json
hako_implementation=lang/src/compiler/lib/agg_local_scalarization_inline_scalar_user_box_local_body_classifier.hako
parity_rows=6
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
mir_type_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
summary=ok
REPORT
