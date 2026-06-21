#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from __future__ import annotations

import re
import subprocess

TARGET_PREFIXES = (
    "src/mir/ordered_map_origin_plan.rs",
    "src/mir/semantic_refresh.rs",
    "src/mir/user_box_method_route_plan.rs",
    "tools/rust_lifecycle/",
)

SUSPECT_TOKENS = [
    re.compile(r"\b(special[- ]case|workaround|temporary|hardcode|hack|for now|TODO|FIXME)\b", re.IGNORECASE),
]

ORDERED_MAP_HINTS = [
    re.compile(r"\bkey\s*==\s*['\"][^'\"]+['\"]"),
    re.compile(r"\bkey\s*!=\s*['\"][^'\"]+['\"]"),
    re.compile(r"seed_(string|text)_key"),
    re.compile(r'MirType::Box\("StringBox"\.to_string\(\)\)'),
]


def fail(reason: str) -> None:
    raise SystemExit(f"hardcode_guard=fail reason={reason}")


def staged_files() -> list[str]:
    result = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMRT"],
        check=True,
        capture_output=True,
        text=True,
    )
    return [path for path in result.stdout.splitlines() if path.startswith(TARGET_PREFIXES)]


def staged_diff(path: str) -> list[str]:
    result = subprocess.run(
        ["git", "diff", "--cached", "--unified=0", "--no-color", "--", path],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.splitlines()


paths = staged_files()
if not paths:
    print("hardcode_guard=skip no_sensitive_staged_files=1")
    print("summary=ok")
    raise SystemExit(0)

for path in paths:
    for line in staged_diff(path):
        if not line.startswith("+") or line.startswith("+++"):
            continue
        added = line[1:]
        for pattern in SUSPECT_TOKENS:
            if pattern.search(added):
                fail(f"suspect_token path={path} line={added}")
        if path == "src/mir/ordered_map_origin_plan.rs":
            for pattern in ORDERED_MAP_HINTS:
                if pattern.search(added):
                    fail(f"ordered_map_special_case path={path} line={added}")

print("output_contract=rust-lifecycle-no-silent-hardcode-v0")
print("sensitive_files=" + ",".join(paths))
print("summary=ok")
PY
