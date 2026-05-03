#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="stage0-shape-inventory-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MODEL="$ROOT_DIR/src/mir/global_call_route_plan/model.rs"
DOC="$ROOT_DIR/docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MODEL" "$DOC"

python3 - "$MODEL" "$DOC" <<'PY'
import re
import sys
from pathlib import Path

model_path = Path(sys.argv[1])
doc_path = Path(sys.argv[2])
model = model_path.read_text(encoding="utf-8")
doc = doc_path.read_text(encoding="utf-8")

match = re.search(r"pub enum GlobalCallTargetShape\s*\{(.*?)\n\}", model, re.S)
if not match:
    print("[stage0-shape-inventory-guard] missing GlobalCallTargetShape enum", file=sys.stderr)
    raise SystemExit(1)

variants = []
for raw in match.group(1).splitlines():
    line = raw.strip().rstrip(",")
    if not line or line.startswith("#") or line.startswith("//"):
        continue
    line = line.split("//", 1)[0].strip()
    if line:
        variants.append(line)

missing = [variant for variant in variants if f"`{variant}`" not in doc]
if missing:
    print(
        "[stage0-shape-inventory-guard] undocumented GlobalCallTargetShape variants:",
        file=sys.stderr,
    )
    for variant in missing:
        print(f"  - {variant}", file=sys.stderr)
    print(f"[stage0-shape-inventory-guard] update {doc_path}", file=sys.stderr)
    raise SystemExit(1)

print(f"[stage0-shape-inventory-guard] ok variants={len(variants)}")
PY
