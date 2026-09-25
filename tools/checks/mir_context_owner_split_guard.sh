#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-context-owner-split-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

METHOD_INDEX="$ROOT_DIR/src/mir/builder/builder_method_index.rs"
COMP_CTX="$ROOT_DIR/src/mir/builder/compilation_context.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$METHOD_INDEX" "$COMP_CTX"

echo "[$TAG] checking method-tail-index owner seam (MIR-CONTEXT-OWNER-SPLIT0)"

python3 - "$ROOT_DIR" "$METHOD_INDEX" "$COMP_CTX" <<'PY'
import pathlib
import re
import sys

tag = "mir-context-owner-split-guard"
root = pathlib.Path(sys.argv[1]).resolve()
method_index = pathlib.Path(sys.argv[2]).resolve()
comp_ctx = pathlib.Path(sys.argv[3]).resolve()


def fail(message: str) -> None:
    print(f"[{tag}] ERROR: {message}", file=sys.stderr)
    sys.exit(1)


index_text = method_index.read_text()
ctx_text = comp_ctx.read_text()

# 1. The catalog type is owned by builder_method_index.
if not re.search(r"pub\(crate\) struct MethodTailIndexV1", index_text):
    fail("MethodTailIndexV1 is not defined in builder_method_index.rs")

# 2. CompilationContext holds exactly one catalog field of that type.
field_hits = re.findall(
    r"pub method_tail_index: super::builder_method_index::MethodTailIndexV1",
    ctx_text,
)
if len(field_hits) != 1:
    fail(
        "CompilationContext must hold exactly one "
        "method_tail_index: MethodTailIndexV1 field "
        f"(found {len(field_hits)})"
    )

# 3. The retired scalar pair must not reappear under src/.
retired = []
for path in (root / "src").rglob("*.rs"):
    text = path.read_text()
    if "method_tail_index_source_len" in text:
        retired.append(str(path.relative_to(root)))
if retired:
    fail(
        "retired method_tail_index_source_len field reintroduced: "
        + ", ".join(sorted(retired))
    )

# 4. No second tail-index HashMap lives on CompilationContext.
if re.search(r"method_tail_index:\s*HashMap", ctx_text):
    fail("CompilationContext still stores the raw HashMap tail index")

print(f"[{tag}] ok")
PY
