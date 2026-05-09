#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-fast-path-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/allocator-fast-path-exe-proof/main.hako"
APP_README="apps/allocator-fast-path-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-065-M13-ALLOCATOR-FAST-PATH-EXE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
RUNE_PROFILE_SSOT="docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md"
REGISTRY="docs/reference/mir/rune-profile-registry.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"
METADATA_FACTS="docs/reference/mir/metadata-facts-ssot.md"

echo "[$TAG] running M13 allocator fast-path EXE guard"

for file in "$APP" "$APP_README" "$CARD" "$TASKBOARD" "$RUNE_PROFILE_SSOT" "$REGISTRY" "$SUBSTRATE_DOC" "$METADATA_FACTS"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q mir_optimizer_consumes_verified_profile_allocator_fast_required_inline -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m13_fast_path.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m13.mir.json"
exe_out="$tmp_dir/m13.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {f.get("name"): f for f in data.get("functions", [])}
callee = functions.get("AllocFastProof.size_to_bin/1")
main = functions.get("main")
if callee is None:
    raise SystemExit("missing AllocFastProof.size_to_bin/1")
if main is None:
    raise SystemExit("missing main")

metadata = callee.get("metadata", {})
inline_plans = metadata.get("inline_plans", [])
if not any(
    plan.get("request") == "required"
    and plan.get("verified") is True
    and plan.get("source") == "rune_profile:allocator.fast"
    for plan in inline_plans
):
    raise SystemExit("missing verified allocator.fast required InlinePlan")

effect_plans = metadata.get("effect_plans", [])
if not any(
    plan.get("source") == "rune_profile"
    and plan.get("requires") == ["no_alloc", "no_safepoint"]
    for plan in effect_plans
):
    raise SystemExit("missing allocator.fast EffectPlan requirements")

capability_plans = metadata.get("capability_plans", [])
if not any(
    plan.get("source") == "rune_profile"
    and plan.get("allow") == ["hako.mem", "hako.ptr", "hako.tls"]
    for plan in capability_plans
):
    raise SystemExit("missing allocator.fast CapabilityPlan allowance")

target = "AllocFastProof.size_to_bin/1"
for route_key in ("global_call_routes", "lowering_plan"):
    for route in main.get("metadata", {}).get(route_key, []) or []:
        if route.get("symbol") == target or route.get("target_symbol") == target:
            raise SystemExit(f"main still has {route_key} for {target}")

for block in main.get("blocks", []):
    for inst in block.get("instructions", []):
        call = inst.get("mir_call") or {}
        callee_obj = call.get("callee") or {}
        if callee_obj.get("type") == "Global" and callee_obj.get("name") == target:
            raise SystemExit(f"main still calls {target}")

print("[m13-mir-json] ok")
PY

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
NYASH_LLVM_ROUTE_TRACE=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
timeout 120 tools/selfhost/selfhost_build.sh \
  --in "$APP" \
  --mir "$mir_json" \
  --exe "$exe_out" >"$build_log" 2>&1

pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

if rg -F -q 'AllocFastProof.size_to_bin/1' "$build_log"; then
  echo "[$TAG] ERROR: allocator fast helper must be inlined before backend route trace" >&2
  sed -n '1,160p' "$build_log" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"
rg -F -q 'allocator-fast-path-exe-proof' "$run_log"
rg -F -q 'bins=3,4' "$run_log"
rg -F -q 'summary=ok' "$run_log"
rg -F -q 'Result: 0' "$run_log"
if rg -F -q 'summary=fail' "$run_log"; then
  echo "[$TAG] ERROR: fixture reported summary=fail" >&2
  cat "$run_log" >&2
  exit 1
fi

rg -F -q '@rune Profile(allocator.fast)' "$APP"
rg -F -q 'M13 scalar EXE proof' "$APP_README"
rg -F -q 'M13 allocator fast-path EXE proof` is live-narrow' "$CARD"
rg -F -q '| `M13 allocator fast-path EXE proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M13 allocator fast-path EXE proof [live-narrow]' "$RUNE_PROFILE_SSOT"
rg -F -q 'Status: M13 live-narrow allocator fast-path EXE proof.' "$REGISTRY"
rg -F -q 'M13 Allocator Fast-Path EXE Proof' "$SUBSTRATE_DOC"
rg -F -q 'M13 may consume verified `request=required`' "$METADATA_FACTS"
rg -F -q 'k2_wide_allocator_fast_path_exe_guard.sh' docs/tools/check-scripts-index.md

if rg -F -q 'Profile(' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume Profile syntax" >&2
  exit 1
fi

if rg -F -q 'allocator.fast' lang/c-abi/shims lang/src/shared/backend -g '*.inc' -g '*.hako' -g '*.rs'; then
  echo "[$TAG] ERROR: backend/.inc must not branch on profile names" >&2
  exit 1
fi

echo "[$TAG] ok"
