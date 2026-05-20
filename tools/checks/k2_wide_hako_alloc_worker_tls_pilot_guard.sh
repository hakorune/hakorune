#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-worker-tls-pilot"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L2"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-350A defers L3/L4 evidence to a closeout or provider-facing route change" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-worker-tls-pilot-proof/main.hako"
APP_README="apps/hako-alloc-worker-tls-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-worker-tls-pilot-proof/test.sh"
CARD_349A="docs/development/current/main/phases/phase-293x/293x-964-MIMAP-349A-OSVM-PAGE-SOURCE-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-965-MIMAP-350A-WORKER-TLS-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-worker-tls-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/worker_tls_pilot_box.hako"
CACHE_OWNER="lang/src/hako_alloc/memory/worker_tls_cache_box.hako"
PAGE_SOURCE_OWNER="lang/src/hako_alloc/memory/osvm_page_source_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh"

printf '[%s] checking MIMAP-350A worker/TLS pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_349A" "$CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$CACHE_OWNER" "$PAGE_SOURCE_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_349A" "MIMAP-349A OSVM/page-source pilot must be landed before worker/TLS pilot"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-350A card must be current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-350A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-350A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-350A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-350A"
guard_expect_in_file "$TAG" 'row_kind = "first-real-seam"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-350A must be marked as first-real-seam"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-350A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.worker_tls_pilot_box' "$MODULE" "module must export worker/TLS pilot owner"
guard_expect_in_file "$TAG" 'worker_tls_pilot_box.hako' "$MEMORY_README" "memory README must name worker/TLS pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocWorkerTlsPilotReportFields' "$OWNER" "worker/TLS owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeWorkerTlsPilotReport' "$OWNER" "worker/TLS owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordWorkerTlsFact' "$OWNER" "worker/TLS owner must expose fact route"
guard_expect_in_file "$TAG" 'HakoAllocOSVMPageSourcePilotReport' "$OWNER" "worker/TLS owner must consume OSVM/page-source report"
guard_expect_in_file "$TAG" 'HakoAllocWorkerTlsCache' "$OWNER" "worker/TLS owner must compose existing cache seam"
guard_expect_in_file "$TAG" 'cache.storeSlot' "$OWNER" "worker/TLS owner must write through cache seam"
guard_expect_in_file "$TAG" 'cache.loadSlot' "$OWNER" "worker/TLS owner must read through cache seam"
guard_expect_in_file "$TAG" 'cache.clearSlot' "$OWNER" "worker/TLS owner must clear through cache seam"
guard_expect_in_file "$TAG" 'would_use_worker_tls: worker_tls_present' "$OWNER" "worker/TLS seam must be explicit"
guard_expect_in_file "$TAG" 'would_run_thread: 0' "$OWNER" "worker/TLS pilot must not schedule workers"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "worker/TLS report must mirror backing bytes as usize"

if rg -n 'pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator|replace_process_allocator' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-350A owner/app must keep source concurrency/provider/replacement seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'worker-tls-pilot-proof|HakoAllocWorkerTlsPilot|workerTlsPilot' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-350A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap350_worker_tls.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap350.mir.json"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
python3 tools/checks/pure_first_route_preflight.py "$mir_json"
python3 - "$mir_json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocWorkerTlsPilot.makeWorkerTlsPilotReport/1",
    "HakoAllocWorkerTlsPilot.recordWorkerTlsFact/3",
    "HakoAllocWorkerTlsPilot.closedExecutionBlockerCount/1",
    "HakoAllocWorkerTlsCache.storeSlot/2",
    "HakoAllocWorkerTlsCache.loadSlot/1",
    "HakoAllocWorkerTlsCache.clearSlot/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocWorkerTlsPilotReport")
if report is None:
    raise SystemExit("missing worker/TLS pilot report typed object plan")
target = "HakoAllocWorkerTlsPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing worker/TLS pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("worker_id", "worker_id_valid", "tls_slot", "tls_slot_valid", "would_use_worker_tls", "would_run_thread"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def callee_label(callee):
    return ".".join(part for part in (callee.get("box_name"), callee.get("name")) if part)

def require_call(fn_name, fragment):
    labels = [callee_label(callee) for callee in iter_calls(functions[fn_name])]
    if not any(fragment in label for label in labels):
        raise SystemExit(f"missing call {fragment} in {fn_name}: {labels}")

require_call("HakoAllocWorkerTlsPilot.recordWorkerTlsFact/3", "HakoAllocWorkerTlsCache.storeSlot")
require_call("HakoAllocWorkerTlsPilot.recordWorkerTlsFact/3", "HakoAllocWorkerTlsCache.loadSlot")
require_call("HakoAllocWorkerTlsPilot.recordWorkerTlsFact/3", "HakoAllocWorkerTlsCache.clearSlot")
require_call("HakoAllocWorkerTlsPilot.recordWorkerTlsFact/3", "HakoAllocWorkerTlsPilot.closedExecutionBlockerCount")
require_call("HakoAllocWorkerTlsCache.storeSlot/2", "TlsCoreBox.cache_slot_set_i64")
require_call("HakoAllocWorkerTlsCache.loadSlot/1", "TlsCoreBox.cache_slot_get_i64")
require_call("HakoAllocWorkerTlsCache.storeSlot/2", "HakoAllocWorkerIdentity.currentWorkerId")
require_call("HakoAllocWorkerTlsCache.loadSlot/1", "HakoAllocWorkerIdentity.currentWorkerId")

for fn_name, fn in functions.items():
    for callee in iter_calls(fn):
        label = callee_label(callee)
        if any(part in label for part in ("Provider", "Channel", "TaskGroup", "RemoteFree")):
            raise SystemExit(f"forbidden call in {fn_name}: {label}")
print("[mimap350a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
