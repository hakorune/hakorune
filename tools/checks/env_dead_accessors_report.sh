#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(".").resolve()
env_dir = root / "src/config/env"
if not env_dir.exists():
    print("[env-audit] src/config/env not found", file=sys.stderr)
    sys.exit(1)

def run_count(cmd: str) -> int:
    out = subprocess.check_output(cmd, shell=True, text=True, cwd=root).strip()
    return int(out) if out else 0

def run_lines(cmd: str) -> list[str]:
    out = subprocess.check_output(cmd, shell=True, text=True, cwd=root).strip()
    return [ln for ln in out.splitlines() if ln]

rows: list[tuple[str, str, list[str], int, int, int]] = []

for file_path in sorted(env_dir.glob("*.rs")):
    rel = file_path.relative_to(root).as_posix()
    module = file_path.stem
    text = file_path.read_text()
    fn_names = re.findall(r"^pub fn ([a-zA-Z0-9_]+)\s*\(", text, flags=re.M)
    for fn_name in fn_names:
        hits = run_lines(
            f"rg -n --glob '*.rs' '\\b{fn_name}\\s*\\(' src tests 2>/dev/null || true"
        )
        external_hits = [ln for ln in hits if not ln.startswith(f"{rel}:")]
        if external_hits:
            continue

        body_match = re.search(
            rf"pub fn {fn_name}\b.*?(?=\n(?:pub fn |#[cfg]|$))",
            text,
            flags=re.S,
        )
        body = body_match.group(0) if body_match else ""
        keys = sorted(set(re.findall(r'"((?:NYASH|HAKO|JOINIR)_[A-Z0-9_]+)"', body)))
        if not keys:
            continue

        src_hits = 0
        tools_hits = 0
        docs_hits = 0
        for key in keys:
            src_hits += run_count(
                f"rg -n --fixed-strings '{key}' src -g '!src/config/env/**' -g '!src/tests/**' 2>/dev/null | wc -l"
            )
            tools_hits += run_count(
                f"rg -n --fixed-strings '{key}' tools 2>/dev/null | wc -l"
            )
            docs_hits += run_count(
                f"rg -n --fixed-strings '{key}' docs 2>/dev/null | wc -l"
            )
        rows.append((module, fn_name, keys, src_hits, tools_hits, docs_hits))

rows.sort(key=lambda x: (x[3] + x[4], x[0], x[1]))

print("status,module,function,keys,src_hits,tools_hits,docs_hits")
for module, fn_name, keys, src_hits, tools_hits, docs_hits in rows:
    status = "dead" if (src_hits + tools_hits) == 0 else "doc-only"
    key_csv = ";".join(keys)
    print(f"{status},{module},{fn_name},{key_csv},{src_hits},{tools_hits},{docs_hits}")
PY
