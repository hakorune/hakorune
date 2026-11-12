#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-owner-count"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
BASELINE="$ROOT_DIR/tools/checks/mir_owner_count_baseline.tsv"
guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" git
guard_require_files "$TAG" "$BASELINE"
# Share the existing parser crate and its dependencies. Keep one foreground
# Cargo, with the same four-job limit as the compiler's own build policy.
cd "$ROOT_DIR"
APPROVED_BASE="${1:-${MIR_OWNER_APPROVED_BASE:-$(git merge-base HEAD main)}}"
APPROVED_BASELINE="$(git show "$APPROVED_BASE:tools/checks/mir_owner_count_baseline.tsv")"
RETIRED="$ROOT_DIR/tools/checks/mir_owner_retired_symbols.txt"
guard_require_files "$TAG" "$RETIRED"
COUNTS="$(CARGO_BUILD_JOBS=4 cargo run --quiet --manifest-path "$ROOT_DIR/tools/checks/rust_source_topology/Cargo.toml" --bin mir_owner_census -- "$ROOT_DIR")"
python3 - "$BASELINE" "$COUNTS" "$APPROVED_BASELINE" "$RETIRED" "$ROOT_DIR/src" <<'PY'
import pathlib, sys
def parse_baseline(text):
    values = {}
    for line in text.splitlines():
        if not line.strip() or line.lstrip().startswith('#'):
            continue
        key, value = line.split()
        if key in values or not value.isdecimal():
            raise SystemExit(f'[mir-owner-count] invalid or duplicate baseline row: {line}')
        values[key] = int(value)
    if not values:
        raise SystemExit('[mir-owner-count] empty baseline')
    return values
baseline = parse_baseline(pathlib.Path(sys.argv[1]).read_text())
approved = parse_baseline(sys.argv[3])
if baseline.keys() != approved.keys():
    raise SystemExit('[mir-owner-count] approved metric keys differ')
for key, value in baseline.items():
    if value > approved[key]:
        raise SystemExit(f'[mir-owner-count] {key}: {approved[key]} -> {value}; 基準を上げるにはユーザーの決定が要る')
import re
retired = pathlib.Path(sys.argv[4]).read_text().splitlines()
retired = [name for name in retired if name and not name.startswith('#')]
if not retired or any(not re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*', name) for name in retired):
    raise SystemExit('[mir-owner-count] invalid or empty retired symbol list')
pattern = re.compile(r'\b(?:' + '|'.join(map(re.escape, retired)) + r')\b')
for source in pathlib.Path(sys.argv[5]).rglob('*'):
    if not source.is_file():
        continue
    match = pattern.search(source.read_text(errors='replace'))
    if match:
        raise SystemExit(f'[mir-owner-count] retired symbol {match[0]} remains in {source}')
actual = {}
for line in sys.argv[2].splitlines():
    key, value, pattern = line.split('\t', 2)
    if key in actual:
        raise SystemExit(f'[mir-owner-count] duplicate actual key: {key}')
    actual[key] = int(value)
    print(f'[mir-owner-count] {key}: {value} (baseline {baseline.get(key, "MISSING")}) — {pattern}')
if baseline.keys() != actual.keys():
    raise SystemExit(f'[mir-owner-count] metric keys differ: baseline-only={sorted(baseline.keys()-actual.keys())}, actual-only={sorted(actual.keys()-baseline.keys())}')
failed = False
for key, value in actual.items():
    if value > baseline[key]:
        print(f'[mir-owner-count] ERROR {key} increased: {baseline[key]} -> {value}; raising the baseline requires a user decision.', file=sys.stderr)
        failed = True
    elif value < baseline[key]:
        print(f'[mir-owner-count] ERROR {key} decreased: {baseline[key]} -> {value}; lower the baseline in this same change.', file=sys.stderr)
        failed = True
if failed:
    raise SystemExit(1)
print('[mir-owner-count] PASS: counts match the non-increasing baseline')
PY
