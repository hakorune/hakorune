#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-191-TYPED-OBJECT-RUNTIME-SINGLE-THREAD-FAST-LANE.md"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"

grep -q '^output_contract=typed-object-runtime-single-thread-fast-lane-v0$' "$DOC"
grep -q '^default_backend=SafeMutexStore$' "$DOC"
grep -q '^selected_backend=SingleThreadExactStore$' "$DOC"
grep -q '^selection_env=HAKO_TYPED_OBJECT_STORE$' "$DOC"
grep -q '^exported_abi_unchanged=1$' "$DOC"
grep -q '^invalid_backend_fail_fast=ok$' "$DOC"
grep -q '^winner_claim=0$' "$DOC"
grep -q '^replacement_active=0$' "$DOC"
grep -q '^hook_installed=0$' "$DOC"
grep -q '^global_allocator=0$' "$DOC"
grep -q '^summary=ok$' "$DOC"
grep -F -q 'HAKO_TYPED_OBJECT_STORE={safe_mutex\|single_thread_exact}' "$ENV_DOC"

cargo test -p nyash_kernel typed_object --lib >/tmp/hakorune_row191_typed_object_unit.log
cargo build --release -p nyash_kernel >/tmp/hakorune_row191_nyash_kernel_release_build.log

HAKO_TYPED_OBJECT_STORE=single_thread_exact \
  RUN_TIMEOUT_SECS=180 \
  bash "$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh" \
  >/tmp/hakorune_row191_single_thread_exact_smoke.log

set +e
HAKO_TYPED_OBJECT_STORE=invalid \
  RUN_TIMEOUT_SECS=180 \
  bash "$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh" \
  >/tmp/hakorune_row191_invalid_backend_smoke.log 2>&1
invalid_rc=$?
set -e
if [ "$invalid_rc" -eq 0 ]; then
  echo "[row191] invalid typed-object backend unexpectedly succeeded" >&2
  exit 1
fi
grep -q '\[freeze:contract\]\[typed-object-store/backend\]' \
  /tmp/hakorune_row191_invalid_backend_smoke.log

echo "output_contract=typed-object-runtime-single-thread-fast-lane-v0"
echo "default_backend=SafeMutexStore"
echo "selected_backend=SingleThreadExactStore"
echo "selection_env=HAKO_TYPED_OBJECT_STORE"
echo "exported_abi_unchanged=1"
echo "safe_mutex_unit_tests=ok"
echo "single_thread_exact_smoke=ok"
echo "invalid_backend_fail_fast=ok"
echo "semantic_summary=ok"
echo "winner_claim=0"
echo "replacement_active=0"
echo "hook_installed=0"
echo "global_allocator=0"
echo "summary=ok"
