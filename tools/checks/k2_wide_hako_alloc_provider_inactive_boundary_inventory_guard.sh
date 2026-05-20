#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-provider-inactive-boundary-inventory"
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
    echo "[$TAG] ERROR: MIMAP-352A defers L3/L4 evidence to a provider boundary closeout or provider-facing row" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-provider-inactive-boundary-inventory-proof/main.hako"
APP_README="apps/hako-alloc-provider-inactive-boundary-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-provider-inactive-boundary-inventory-proof/test.sh"
CARD_350A="docs/development/current/main/phases/phase-293x/293x-965-MIMAP-350A-WORKER-TLS-PILOT.md"
CARD_351A="docs/development/current/main/phases/phase-293x/293x-967-MIMAP-351A-POST-WORKER-TLS-PILOT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-968-MIMAP-352A-PROVIDER-INACTIVE-BOUNDARY-INVENTORY.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-969-MIMAP-353A-POST-PROVIDER-INACTIVE-BOUNDARY-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-provider-inactive-boundary-inventory-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/provider_inactive_boundary_inventory_box.hako"
WORKER_OWNER="lang/src/hako_alloc/memory/worker_tls_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

printf '[%s] checking MIMAP-352A provider inactive boundary inventory\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_350A" "$CARD_351A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$WORKER_OWNER" "$SELF_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_350A" "MIMAP-350A worker/TLS pilot must be landed before provider inactive boundary inventory"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_351A" "MIMAP-351A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-352A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-353A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-352A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-352A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-352A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-352A"
guard_expect_in_file "$TAG" 'row_kind = "inventory"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-352A must be an inventory row"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-352A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-352A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.provider_inactive_boundary_inventory_box' "$MODULE" "module must export provider inactive boundary owner"
guard_expect_in_file "$TAG" 'provider_inactive_boundary_inventory_box.hako' "$MEMORY_README" "memory README must name provider inactive boundary owner"
guard_expect_in_file "$TAG" 'record HakoAllocProviderInactiveBoundaryInventoryReportFields' "$OWNER" "provider inactive boundary owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeProviderInactiveBoundaryInventoryReport' "$OWNER" "provider inactive boundary owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordProviderInactiveBoundary' "$OWNER" "provider inactive boundary owner must expose inventory route"
guard_expect_in_file "$TAG" 'HakoAllocWorkerTlsPilotReport' "$OWNER" "provider inactive boundary owner must consume worker/TLS report"
guard_expect_in_file "$TAG" 'provider_activation_supported: 0' "$OWNER" "provider activation must remain unsupported"
guard_expect_in_file "$TAG" 'provider_activation_inactive: 1' "$OWNER" "provider activation must remain inactive"
guard_expect_in_file "$TAG" 'host_replacement_inactive: 1' "$OWNER" "host replacement must remain inactive"
guard_expect_in_file "$TAG" 'hooks_inactive: 1' "$OWNER" "hooks must remain inactive"
guard_expect_in_file "$TAG" 'backend_matcher_inactive: 1' "$OWNER" "backend matchers must remain inactive"
guard_expect_in_file "$TAG" 'would_activate_provider: 0' "$OWNER" "provider activation must not execute"
guard_expect_in_file "$TAG" 'would_replace_host_allocator: 0' "$OWNER" "host allocator replacement must not execute"
guard_expect_in_file "$TAG" 'would_install_hook: 0' "$OWNER" "hook installation must not execute"
guard_expect_in_file "$TAG" 'would_add_backend_matcher: 0' "$OWNER" "backend matcher addition must not execute"

if rg -n 'providerActivate|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|selectProvider|activateProvider|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-352A owner/app must keep provider/replacement/hook/backend/source-concurrency seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'provider-inactive-boundary-inventory-proof|ProviderInactiveBoundaryInventory|providerInactiveBoundaryInventory' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-352A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap352_provider_inactive.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap352.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-provider-inactive-boundary-inventory-proof' "$vm_log"
rg -F -q 'provider=1,0,1,99019005099,1,0,1,1,1,1' "$vm_log"
rg -F -q 'worker=1,1,0,7,99019005077,99019005066,99019005055,99019005044' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'closed=0,0,0,1,1,1,1,0,0,0,0,0' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

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
    "HakoAllocProviderInactiveBoundaryInventory.makeProviderInactiveBoundaryInventoryReport/1",
    "HakoAllocProviderInactiveBoundaryInventory.recordProviderInactiveBoundary/2",
    "HakoAllocProviderInactiveBoundaryInventory.reject/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocProviderInactiveBoundaryInventoryReport")
if report is None:
    raise SystemExit("missing provider inactive boundary report typed object plan")
target = "HakoAllocProviderInactiveBoundaryInventoryReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing provider inactive boundary ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "provider_activation_inactive",
    "host_replacement_inactive",
    "hooks_inactive",
    "backend_matcher_inactive",
    "would_activate_provider",
    "would_replace_host_allocator",
    "would_install_hook",
    "would_add_backend_matcher",
    "invalid_boundary_token_reject_count",
):
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

for fn_name, fn in functions.items():
    for callee in iter_calls(fn):
        label = callee_label(callee)
        if any(part in label for part in ("ProviderActivation", "GlobalAllocator", "HookInstaller", "BackendMatcherInstaller")):
            raise SystemExit(f"forbidden provider/backend call in {fn_name}: {label}")
print("[mimap352a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
