#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-string-corridor-name-vocabulary-parity-gate"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json"
HAKO_IMPL="$ROOT_DIR/lang/src/compiler/lib/string_corridor_name_vocabulary.hako"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_IMPL" "$HAKO_BIN"

TMP_DIR="$(mktemp -d /tmp/hakorune-string-corridor-name-parity.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR" >/dev/null 2>&1 || true
}
trap cleanup EXIT

APP="$TMP_DIR/string_corridor_name_vocabulary_parity.hako"
EXPECTED="$TMP_DIR/expected.txt"
ACTUAL="$TMP_DIR/actual.txt"
EXE="$TMP_DIR/string_corridor_name_vocabulary_parity.exe"
EMIT_LOG="$TMP_DIR/emit.log"

python3 - "$FIXTURE" "$APP" "$EXPECTED" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
app = Path(sys.argv[2])
expected = Path(sys.argv[3])

categories = [
    ("stringish_box", "is_stringish_box_name"),
    ("len_method", "is_len_method_name"),
    ("slice_method", "is_slice_method_name"),
    ("lowered_len_global", "is_lowered_len_global"),
    ("runtime_len_export", "is_runtime_len_export"),
    ("runtime_len_handle_export", "is_runtime_len_handle_export"),
    ("runtime_slice_export", "is_runtime_slice_export"),
    ("runtime_substring_export", "is_runtime_substring_export"),
    ("runtime_substring_len_export", "is_runtime_substring_len_export"),
    ("runtime_substring_concat3_export", "is_runtime_substring_concat3_export"),
    ("runtime_concat3_export", "is_runtime_concat3_export"),
]

rows = fixture["rows"]
expected_lines = []
calls = []
for row in rows:
    name = json.dumps(row["name"])
    for category, func in categories:
        value = "1" if row["expected"].get(category, False) else "0"
        expected_lines.append(value)
        calls.append(
            f"    print(StringCorridorNameVocabularyBox.{func}_flag({name}))"
        )

expected.write_text("\n".join(expected_lines) + "\n", encoding="utf-8")

source = "\n".join(
    [
        "using lang.compiler.lib.string_corridor_name_vocabulary as StringCorridorNameVocabularyBox",
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
    print("[string-corridor-name/parity] mismatch")
    max_len = max(len(expected), len(actual))
    for idx in range(max_len):
        exp = expected[idx] if idx < len(expected) else "<missing>"
        got = actual[idx] if idx < len(actual) else "<missing>"
        if exp != got:
            print(f"row={idx} expected={exp} actual={got}")
    raise SystemExit(1)
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-string-corridor-name-vocabulary-parity-gate-v0
owner=string_corridor_name_vocabulary_classifier
rust_oracle_fixture=mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json
hako_implementation=lang/src/compiler/lib/string_corridor_name_vocabulary.hako
parity_rows=18
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
string_corridor_fact_inference_migration=0
mir_instruction_traversal_migration=0
summary=ok
REPORT
