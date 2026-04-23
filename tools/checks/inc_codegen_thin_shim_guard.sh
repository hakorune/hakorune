#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="inc-codegen-thin-shim-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ALLOWLIST="$ROOT_DIR/tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv"
VIEW_ALLOWLIST="$ROOT_DIR/tools/checks/inc_codegen_thin_shim_view_allowlist.tsv"
SHIMS_DIR="$ROOT_DIR/lang/c-abi/shims"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ALLOWLIST" "$VIEW_ALLOWLIST"

echo "[$TAG] checking .inc analysis-debt no-growth baseline"

python3 - "$ROOT_DIR" "$ALLOWLIST" "$VIEW_ALLOWLIST" "$SHIMS_DIR" <<'PY'
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
allowlist_path = pathlib.Path(sys.argv[2])
view_allowlist_path = pathlib.Path(sys.argv[3])
shims_dir = pathlib.Path(sys.argv[4])

patterns = [
    re.compile(r"analyze_[A-Za-z0-9_]*candidate"),
    re.compile(r"hako_llvmc_match_[A-Za-z0-9_]*seed"),
    re.compile(r"window_candidate"),
    re.compile(r'yyjson_obj_get\([^\n]*"(?:blocks|instructions|op)"'),
]
view_begin = re.compile(r"inc-codegen-view-owner:\s*([A-Za-z0-9_./-]+)\s+begin")
view_end = re.compile(r"inc-codegen-view-owner:\s*([A-Za-z0-9_./-]+)\s+end")


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

view_baseline: dict[tuple[str, str], int] = {}
for raw in view_allowlist_path.read_text().splitlines():
    line = raw.strip()
    if not line or line.startswith("#"):
        continue
    parts = line.split()
    if len(parts) != 3:
        print(f"[inc-codegen-thin-shim-guard] ERROR: bad view allowlist row: {raw}", file=sys.stderr)
        sys.exit(2)
    view_baseline[(parts[0], parts[1])] = int(parts[2])

actual: dict[str, int] = {}
view_actual: dict[tuple[str, str], int] = {}
failed = False
for path in sorted(shims_dir.glob("*.inc")):
    count = 0
    active_view_label: str | None = None
    path_rel = rel(path)
    for line in path.read_text(errors="ignore").splitlines():
        begin = view_begin.search(line)
        end = view_end.search(line)
        if begin:
            if active_view_label is not None:
                print(
                    f"[inc-codegen-thin-shim-guard] ERROR: nested view-owner region in {path_rel}",
                    file=sys.stderr,
                )
                failed = True
            active_view_label = begin.group(1)
            if (path_rel, active_view_label) not in view_baseline:
                print(
                    f"[inc-codegen-thin-shim-guard] ERROR: unallowlisted view-owner region: {path_rel} {active_view_label}",
                    file=sys.stderr,
                )
                failed = True
            continue
        if end:
            label = end.group(1)
            if active_view_label != label:
                print(
                    f"[inc-codegen-thin-shim-guard] ERROR: mismatched view-owner end in {path_rel}: {label}",
                    file=sys.stderr,
                )
                failed = True
            active_view_label = None
            continue
        if any(pattern.search(line) for pattern in patterns):
            if active_view_label is not None:
                key = (path_rel, active_view_label)
                view_actual[key] = view_actual.get(key, 0) + 1
            else:
                count += 1
    if active_view_label is not None:
        print(
            f"[inc-codegen-thin-shim-guard] ERROR: unterminated view-owner region in {path_rel}: {active_view_label}",
            file=sys.stderr,
        )
        failed = True
    if count:
        actual[path_rel] = count

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

for key, count in sorted(view_actual.items()):
    limit = view_baseline.get(key)
    path, label = key
    if limit is None:
        print(
            f"[inc-codegen-thin-shim-guard] ERROR: new view-owner debt region: {path} {label} ({count} view lines)",
            file=sys.stderr,
        )
        failed = True
    elif count > limit:
        print(
            f"[inc-codegen-thin-shim-guard] ERROR: view-owner shape reads grew: {path} {label} {count}>{limit}",
            file=sys.stderr,
        )
        failed = True

removed = sorted(set(baseline) - set(actual))
reduced = sorted(path for path, limit in baseline.items() if path in actual and actual[path] < limit)
view_removed = sorted(set(view_baseline) - set(view_actual))
view_reduced = sorted(key for key, limit in view_baseline.items() if key in view_actual and view_actual[key] < limit)

if removed:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(removed)} baseline files no longer have debt; allowlist prune recommended"
    )
if reduced:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(reduced)} baseline files reduced debt; allowlist prune recommended"
    )
if view_removed:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(view_removed)} view-owner regions no longer have shape reads; view allowlist prune recommended"
    )
if view_reduced:
    print(
        f"[inc-codegen-thin-shim-guard] NOTE: {len(view_reduced)} view-owner regions reduced shape reads; view allowlist prune recommended"
    )

if failed:
    sys.exit(1)

print(
    f"[inc-codegen-thin-shim-guard] ok files={len(actual)} debt_lines={sum(actual.values())} "
    f"view_owner_files={len({path for path, _ in view_actual})} view_owner_lines={sum(view_actual.values())}"
)
PY
