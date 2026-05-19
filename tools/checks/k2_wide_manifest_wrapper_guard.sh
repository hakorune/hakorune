#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-manifest-wrapper-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MANIFEST="tools/checks/guard_rows.toml"
DESIGN="docs/development/current/main/design/guard-manifest-migration-ssot.md"
FIRST_WRAPPER_CARD="docs/development/current/main/phases/phase-293x/293x-577-GUARD-MANIFEST-003-SEGMENT-CLOSEOUT-THIN-WRAPPERS.md"
CLOSEOUT_CARD="docs/development/current/main/phases/phase-293x/293x-584-GUARD-MANIFEST-010-CLOSEOUT-OR-RETURN-SELECTION.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MANIFEST" "$DESIGN" "$FIRST_WRAPPER_CARD" "$CLOSEOUT_CARD"
guard_require_exec_files "$TAG" "$0"

guard_expect_in_file "$TAG" "tools/checks/impl" "$DESIGN" "migration SSOT must name implementation command location"
guard_expect_in_file "$TAG" "hako-alloc-closeout" "$DESIGN" "migration SSOT must name closeout profile"
guard_expect_in_file "$TAG" "k2_wide_manifest_wrapper_guard.sh" \
  "$FIRST_WRAPPER_CARD" "GM003 card must name this guard"
guard_expect_in_file "$TAG" "hako-alloc-closeout" \
  "$CLOSEOUT_CARD" "GM010 card must name profile-derived closeout guard"

python3 - "$ROOT_DIR" "$MANIFEST" <<'PY'
import os
import pathlib
import sys
import tomllib

root = pathlib.Path(sys.argv[1]).resolve()
manifest_path = root / sys.argv[2]

CLOSEOUT_PROFILE = "hako-alloc-closeout"
IMPL_PREFIX = "tools/checks/impl/"
WRAPPER_PREFIX = "tools/checks/"
PUBLIC_GLOB = "k2_wide_hako_alloc_*closeout_guard.sh"

def load_rows(path: pathlib.Path, stack: tuple[pathlib.Path, ...] = ()) -> list[dict]:
    if path in stack:
        cycle = " -> ".join(str(item.relative_to(root)) for item in (*stack, path))
        raise SystemExit(f"guard_rows.toml include cycle: {cycle}")

    data = tomllib.loads(path.read_text(encoding="utf-8"))
    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        raise SystemExit(f"{path.relative_to(root)} includes must be a list of non-empty strings")

    rows: list[dict] = []
    for include in includes:
        include_path = root / include
        if not include_path.is_file():
            raise SystemExit(f"missing included manifest: {include}")
        rows.extend(load_rows(include_path, (*stack, path)))

    local_rows = data.get("rows")
    if not isinstance(local_rows, list):
        raise SystemExit(f"{path.relative_to(root)} must contain [[rows]] entries")
    for row in local_rows:
        if not isinstance(row, dict):
            raise SystemExit(f"{path.relative_to(root)} row entry is not a table")
        rows.append(row)
    return rows

rows = load_rows(manifest_path)

expected = {}
seen_wrappers = set()
seen_impls = set()
errors: list[str] = []

for row in rows:
    if isinstance(row, dict) and isinstance(row.get("id"), str):
        profiles = row.get("profiles")
        if isinstance(profiles, list) and CLOSEOUT_PROFILE in profiles:
            row_id = row["id"]
            cmd = row.get("cmd")
            if not (
                isinstance(cmd, list)
                and len(cmd) == 2
                and cmd[0] == "bash"
                and isinstance(cmd[1], str)
                and cmd[1].startswith(IMPL_PREFIX)
                and cmd[1].endswith("_closeout_guard.sh")
            ):
                errors.append(
                    f"{row_id}: closeout row cmd must be "
                    f"['bash', 'tools/checks/impl/*_closeout_guard.sh'], got {cmd!r}"
                )
                continue

            impl_path = cmd[1]
            wrapper_path = WRAPPER_PREFIX + pathlib.PurePosixPath(impl_path).name
            if wrapper_path in seen_wrappers:
                errors.append(f"{row_id}: duplicate public wrapper path: {wrapper_path}")
            if impl_path in seen_impls:
                errors.append(f"{row_id}: duplicate implementation path: {impl_path}")
            seen_wrappers.add(wrapper_path)
            seen_impls.add(impl_path)
            expected[row_id] = {
                "wrapper": wrapper_path,
                "impl": impl_path,
            }

if not expected:
    errors.append(f"manifest profile has no rows: {CLOSEOUT_PROFILE}")

public_wrappers = {
    str(path.relative_to(root))
    for path in (root / "tools/checks").glob(PUBLIC_GLOB)
}
expected_wrappers = {spec["wrapper"] for spec in expected.values()}
for wrapper_path in sorted(public_wrappers - expected_wrappers):
    errors.append(f"public hako_alloc closeout wrapper is not manifest-backed: {wrapper_path}")
for wrapper_path in sorted(expected_wrappers - public_wrappers):
    errors.append(f"manifest closeout row wrapper missing: {wrapper_path}")

for row_id, spec in sorted(expected.items()):
    wrapper = root / spec["wrapper"]
    impl = root / spec["impl"]
    if not wrapper.is_file():
        errors.append(f"{row_id}: wrapper missing: {spec['wrapper']}")
        continue
    if not impl.is_file():
        errors.append(f"{row_id}: implementation command missing: {spec['impl']}")
        continue
    if not os.access(wrapper, os.X_OK):
        errors.append(f"{row_id}: wrapper is not executable: {spec['wrapper']}")
    if not os.access(impl, os.X_OK):
        errors.append(f"{row_id}: implementation command is not executable: {spec['impl']}")

    text = wrapper.read_text(encoding="utf-8")
    if (
        f"run_row_guard.sh --only {row_id}" not in text
        and f"run_row_guard.sh\" --only {row_id}" not in text
    ):
        errors.append(f"{row_id}: wrapper must delegate to run_row_guard.sh --only {row_id}")
    forbidden = [
        "guard_common.sh",
        "pure_first_exe_guard.sh",
        "guard_expect_in_file",
        "guard_require_files",
        "mktemp",
        "rg -n",
        "python3 -",
    ]
    for marker in forbidden:
        if marker in text:
            errors.append(f"{row_id}: wrapper regrew embedded guard body marker: {marker}")

if errors:
    for error in errors:
        print(f"[k2-wide-manifest-wrapper-guard] ERROR: {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"[k2-wide-manifest-wrapper-guard] profile={CLOSEOUT_PROFILE} checked={len(expected)}")
PY

echo "[$TAG] ok"
