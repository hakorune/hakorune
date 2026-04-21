#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="inc-codegen-thin-shim-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ALLOWLIST="$ROOT_DIR/tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv"
SHIMS_DIR="$ROOT_DIR/lang/c-abi/shims"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ALLOWLIST"

echo "[$TAG] checking .inc analysis-debt no-growth baseline"

python3 - "$ROOT_DIR" "$ALLOWLIST" "$SHIMS_DIR" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
allowlist_path = pathlib.Path(sys.argv[2])
shims_dir = pathlib.Path(sys.argv[3])

patterns = [
    re.compile(r"analyze_[A-Za-z0-9_]*candidate"),
    re.compile(r"hako_llvmc_match_[A-Za-z0-9_]*seed"),
    re.compile(r"window_candidate"),
    re.compile(r'yyjson_obj_get\([^\n]*"(?:blocks|instructions|op)"'),
]


def rel(path: pathlib.Path) -> str:
    return path.resolve().relative_to(root.resolve()).as_posix()


baseline: dict[str, int] = {}
for raw in allowlist_path.read_text().splitlines():
    line = raw.strip()
    if not line or line.startswith("#"):
        continue
    parts = line.split()
    if len(parts) != 2:
        print(f"[inc-codegen-thin-shim-guard] ERROR: bad allowlist row: {raw}", file=sys.stderr)
        sys.exit(2)
    baseline[parts[0]] = int(parts[1])

actual: dict[str, int] = {}
for path in sorted(shims_dir.glob("*.inc")):
    count = 0
    for line in path.read_text(errors="ignore").splitlines():
        if any(pattern.search(line) for pattern in patterns):
            count += 1
    if count:
        actual[rel(path)] = count

failed = False
for path, count in sorted(actual.items()):
    limit = baseline.get(path)
    if limit is None:
        print(
            f"[inc-codegen-thin-shim-guard] ERROR: new .inc analysis-debt file: {path} ({count} debt lines)",
            file=sys.stderr,
        )
        failed = True
    elif count > limit:
        print(
            f"[inc-codegen-thin-shim-guard] ERROR: .inc analysis debt grew: {path} {count}>{limit}",
            file=sys.stderr,
        )
        failed = True

removed = sorted(set(baseline) - set(actual))
reduced = sorted(path for path, limit in baseline.items() if path in actual and actual[path] < limit)

if removed:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(removed)} baseline files no longer have debt; allowlist prune recommended"
    )
if reduced:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(reduced)} baseline files reduced debt; allowlist prune recommended"
    )

if failed:
    sys.exit(1)

print(
    f"[inc-codegen-thin-shim-guard] ok files={len(actual)} debt_lines={sum(actual.values())}"
)
PY
