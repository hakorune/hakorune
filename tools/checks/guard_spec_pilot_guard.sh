#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="guard-spec-pilot-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUNNER="tools/checks/guard_spec_runner.py"
SPEC="tools/checks/specs/hako_alloc_osvm_fast_path_route_closeout.toml"
IMPL="tools/checks/impl/k2_wide_hako_alloc_osvm_fast_path_route_closeout_guard.sh"
PUBLIC="tools/checks/k2_wide_hako_alloc_osvm_fast_path_route_closeout_guard.sh"
DESIGN="docs/development/current/main/design/guard-manifest-migration-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-648-GUARD-MANIFEST-013-DECLARATIVE-GUARD-SPEC-PILOT.md"
MANIFEST="tools/checks/guard_rows.toml"
INDEX="docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$RUNNER" "$SPEC" "$IMPL" "$PUBLIC" "$DESIGN" "$CARD" "$MANIFEST" "$INDEX"
guard_require_exec_files "$TAG" "$RUNNER" "$IMPL" "$PUBLIC" "$0"

guard_expect_in_file "$TAG" "Declarative Guard Spec Pilot" "$DESIGN" \
  "guard manifest SSOT must describe declarative guard spec pilot"
guard_expect_in_file "$TAG" "guard_spec_runner.py" "$DESIGN" \
  "guard manifest SSOT must name spec runner"
guard_expect_in_file "$TAG" "hako_alloc_osvm_fast_path_route_closeout.toml" "$DESIGN" \
  "guard manifest SSOT must name pilot spec"
guard_expect_in_file "$TAG" "guard-spec-pilot" "$MANIFEST" \
  "guard rows manifest must register the spec pilot guard"
guard_expect_in_file "$TAG" "guard_spec_pilot_guard.sh" "$INDEX" \
  "check index must list the spec pilot guard"
guard_expect_in_file "$TAG" "guard_spec_runner.py" "$IMPL" \
  "pilot implementation wrapper must call guard spec runner"
guard_expect_in_file "$TAG" "run_row_guard.sh\" --only hako-alloc-osvm-fast-path-route-closeout" "$PUBLIC" \
  "public wrapper must remain manifest-backed"

if rg -n 'guard_common.sh|guard_expect_in_file|guard_require_files|mktemp|rg -n|python3 -' "$IMPL" >/tmp/"$TAG".impl_leak 2>&1; then
  cat /tmp/"$TAG".impl_leak >&2
  rm -f /tmp/"$TAG".impl_leak
  guard_fail "$TAG" "pilot implementation wrapper regrew shell guard body"
fi
rm -f /tmp/"$TAG".impl_leak

python3 -m py_compile "$RUNNER"
python3 "$RUNNER" --root "$ROOT_DIR" --spec "$SPEC"
bash "$PUBLIC"

echo "[$TAG] ok"
