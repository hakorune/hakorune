#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-reclaim-tls-readonly-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

OWNER="lang/src/hako_alloc/memory/segment_arena_reclaim_tls_readonly_integration_box.hako"
MATRIX="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako"
SUPPORT_GATE="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako"
PREREQ="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako"
WORKER="lang/src/hako_alloc/memory/worker_tls_pilot_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APP="apps/hako-alloc-segment-arena-reclaim-tls-readonly-integration-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-arena-reclaim-tls-readonly-integration-proof/test.sh"
APP_README="apps/hako-alloc-segment-arena-reclaim-tls-readonly-integration-proof/README.md"
CARD="docs/development/current/main/phases/phase-296x/296x-646-HAKO-MIMALLOC-SEGMENT-ARENA-RECLAIM-TLS-READONLY-INTEGRATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
WORKSTREAM="docs/development/current/main/workstreams/mimalloc-current.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_INVENTORY="tools/checks/manifests/proof_apps/hako_alloc_inventory.toml"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_reclaim_tls_readonly_integration_guard.sh"

echo "[$TAG] checking segment-arena reclaim/tls read-only integration"

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$OWNER" \
  "$MATRIX" \
  "$SUPPORT_GATE" \
  "$PREREQ" \
  "$WORKER" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$TASKBOARD" \
  "$WORKSTREAM" \
  "$INDEX" \
  "$PROOF_INVENTORY" \
  "$0"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "integration card must remain current while the row is open"
guard_expect_in_file "$TAG" 'segment_arena_reclaim_tls_unification' "$CARD" "card must mention the segment-arena reclaim/TLS seam"
guard_expect_in_file "$TAG" 'matrix + support gate + pointer prerequisite + worker/TLS' "$TASKBOARD" "taskboard must mention the read-only integration seam"
guard_expect_in_file "$TAG" 'segment_arena_reclaim_tls_unification' "$WORKSTREAM" "workstream must mention the read-only integration seam"
guard_expect_in_file "$TAG" 'memory.segment_arena_reclaim_tls_readonly_integration_box = "memory/segment_arena_reclaim_tls_readonly_integration_box.hako"' "$MODULE" "hako_alloc module must export the readonly integration owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentArenaReclaimTlsReadonlyIntegration' "$OWNER" "readonly integration owner must exist"
guard_expect_in_file "$TAG" 'recordReadonlyIntegration' "$OWNER" "readonly integration owner must expose recordReadonlyIntegration"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box' "$OWNER" "readonly integration owner must import matrix surface"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box' "$OWNER" "readonly integration owner must import support gate surface"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box' "$OWNER" "readonly integration owner must import pointer prerequisite surface"
guard_expect_in_file "$TAG" 'worker_tls_pilot_box' "$OWNER" "readonly integration owner must import worker/TLS surface"
guard_expect_in_file "$TAG" 'HakoAllocSegmentArenaReclaimTlsReadonlyIntegration' "$ROOT_README" "root README must document the readonly integration owner"
guard_expect_in_file "$TAG" 'segment_arena_reclaim_tls_readonly_integration_box.hako' "$MEMORY_README" "memory README must document the readonly integration module"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.segment_arena_reclaim_tls_readonly_integration_box as HakoAllocSegmentArenaReclaimTlsReadonlyIntegration' "$APP" "proof app must import the readonly integration owner"
guard_expect_in_file "$TAG" 'check "segment arena reclaim tls readonly integration"' "$APP" "proof app must use labelled check block"
guard_expect_in_file "$TAG" 'tools/checks/k2_wide_hako_alloc_segment_arena_reclaim_tls_readonly_integration_guard.sh' "$INDEX" "check script index must list the readonly integration guard"
guard_expect_in_file "$TAG" 'id = "M218"' "$PROOF_INVENTORY" "proof app inventory must list M218"

if rg -n 'providerActivate|replace_host_allocator|replace_process_allocator|install_hook|global_allocator|winner_claim|worker_local|task_scope|spawn|thread::' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: readonly integration owner/app leaked provider/replacement/thread behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'segment-arena-reclaim-tls-readonly-integration|HakoAllocSegmentArenaReclaimTlsReadonlyIntegration|segment_arena_reclaim_tls_readonly_integration' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: readonly integration app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_segment_arena_reclaim_tls_readonly_integration.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/out"
err="$tmp_dir/err"

if [[ -n "${HAKORUNE_BIN:-}" ]]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend vm "$APP" >"$out" 2>"$err"

rg -F -q 'hako-alloc-segment-arena-reclaim-tls-readonly-integration-proof' "$out"
rg -F -q 'integration=1,0,1,1,1,1,1,1' "$out"
rg -F -q 'counts=6,1,5,1,1,1,1,1,9' "$out"
rg -F -q 'reasons=1,4,5,8,9' "$out"
rg -F -q 'closed=0,0,0,0,0' "$out"
rg -F -q 'check=1' "$out"
rg -F -q 'summary=ok' "$out"

cat "$out"
echo "[$TAG] ok"
