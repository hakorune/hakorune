#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-312-LOCALTYPE-001-LOCAL-TYPE-ANNOTATION-METADATA-CAPSULE.md'
ssot='docs/development/current/main/design/local-type-annotation-metadata-capsule-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[localtype-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-312 LOCALTYPE-001 local type annotation metadata capsule"
require_text "$ssot" "Local Type Annotation Metadata Capsule SSOT"
require_text docs/reference/language/EBNF.md "local_type_opt := (':' TYPE_REF)?"
require_text src/ast/mod.rs "declared_type_names: Vec<Option<String>>"
require_text src/parser/statements/variables.rs "local type annotation"
require_text src/macro/ast_json/joinir_compat.rs "declared_type_names"
require_text src/stage1/program_json_v0/lowering.rs '"declared_type": declared_type_name'
require_text docs/tools/check-scripts-index.md "k2_wide_localtype_metadata_capsule_guard.sh"

python3 - <<'PY'
from pathlib import Path
missing = []
for root in (Path("src"), Path("tests")):
    for path in root.rglob("*.rs"):
        text = path.read_text()
        i = 0
        needle = "ASTNode::Local {"
        while True:
            start = text.find(needle, i)
            if start == -1:
                break
            brace = text.find("{", start)
            depth = 0
            j = brace
            while j < len(text):
                if text[j] == "{":
                    depth += 1
                elif text[j] == "}":
                    depth -= 1
                    if depth == 0:
                        j += 1
                        break
                j += 1
            block = text[start:j]
            if "initial_values:" in block and "declared_type_names" not in block:
                missing.append(f"{path}:{text.count(chr(10), 0, start) + 1}")
            i = j
if missing:
    print("[localtype-capsule] Local constructors missing declared_type_names:", file=__import__("sys").stderr)
    for item in missing:
        print(item, file=__import__("sys").stderr)
    raise SystemExit(1)
PY

cargo test -q source_to_program_json_v0_transports_local_type_annotation_metadata --lib
cargo test -q source_to_program_json_v0_rejects_multi_local_after_type_annotation --lib

echo "[localtype-capsule] OK"
