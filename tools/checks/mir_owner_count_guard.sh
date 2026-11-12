#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-owner-count"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
BASELINE="$ROOT_DIR/tools/checks/mir_owner_count_baseline.tsv"
guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$BASELINE"
# Share the existing parser crate and its dependencies. Keep one foreground
# Cargo, with the same four-job limit as the compiler's own build policy.
cd "$ROOT_DIR"
COUNTS="$(CARGO_BUILD_JOBS=4 cargo run --quiet --manifest-path "$ROOT_DIR/tools/checks/rust_source_topology/Cargo.toml" --bin mir_owner_census -- "$ROOT_DIR")"
python3 - "$BASELINE" "$COUNTS" <<'PY'
import pathlib, sys
baseline = {}
for line in pathlib.Path(sys.argv[1]).read_text().splitlines():
    if not line.strip() or line.lstrip().startswith('#'):
        continue
    key, value = line.split()
    if key in baseline:
        raise SystemExit(f'[mir-owner-count] duplicate baseline key: {key}')
    baseline[key] = int(value)
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
