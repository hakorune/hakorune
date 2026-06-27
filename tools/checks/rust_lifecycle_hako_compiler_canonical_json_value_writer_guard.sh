#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-hako-compiler-canonical-json-value-writer-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LIB_DIR="$ROOT_DIR/lang/src/compiler/lib"
TEXT_BUILDER="$LIB_DIR/text_builder.hako"
PROJECTION_VALUE="$LIB_DIR/projection_value.hako"
CANONICAL_JSON="$LIB_DIR/canonical_json.hako"
README="$LIB_DIR/README.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TEXT_BUILDER" "$PROJECTION_VALUE" "$CANONICAL_JSON" "$README" "$ROOT_DIR/tools/bin/hako"

verify_hako() {
  local file="$1"
  bash "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$file"
}

verify_hako "$TEXT_BUILDER"
verify_hako "$PROJECTION_VALUE"
verify_hako "$CANONICAL_JSON"

python3 - <<'PY'
from pathlib import Path

root = Path("lang/src/compiler/lib")

checks = {
    root / "text_builder.hako": [
        "static box TextBuilder",
        "box TextBuilderBox",
        "append_json_string(text)",
        "finish()",
    ],
    root / "projection_value.hako": [
        "static box CompilerProjectionValueBox",
        "create_null()",
        "create_bool(val)",
        "create_i64(val)",
        "create_string(val)",
        "create_array()",
        "create_object()",
        "array_push(value, item)",
        "object_set(value, key, item)",
        "object_get(value, key)",
        "object_has(value, key)",
    ],
    root / "canonical_json.hako": [
        "static box CanonicalJsonWriterBox",
        "canonicalize(value)",
        "write_into(builder, value)",
        "TextOrder.compare_rust_string_v1",
        "CompilerProjectionValueBox.kind",
        "CompilerProjectionValueBox.value",
        "_append_array(builder, arr)",
        "_append_object(builder, obj)",
    ],
}

for path, needles in checks.items():
    text = path.read_text(encoding="utf-8")
    for needle in needles:
        if needle not in text:
            raise SystemExit(f"missing canonical-json surface text: {path} :: {needle}")

readme = (root / "README.md").read_text(encoding="utf-8")
for needle in [
    "Hako Compiler Libraries",
    "canonical_json.hako",
    "projection_value.hako",
    "text_builder.hako",
    "ordinary `.hako` library home",
]:
    if needle not in readme:
        raise SystemExit(f"missing compiler-lib README text: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-hako-compiler-canonical-json-value-writer-v0
library_home=lang/src/compiler/lib/
canonical_json_writer=green
text_builder_surface=green
projection_value_surface=green
ordinary_hako_library=1
abi_surface=0
host_surface=0
syntax_surface=0
hako_buf_backing=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
