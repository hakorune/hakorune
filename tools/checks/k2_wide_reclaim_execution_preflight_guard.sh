#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-reclaim-execution-preflight"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PREFLIGHT="tools/checks/pure_first_route_preflight.py"
CARD="docs/development/current/main/phases/phase-293x/293x-539-MIMAP-052B-RECLAIM-EXECUTION-INTENT-MARKER-PREFLIGHT.md"
DESIGN="docs/development/current/main/design/reclaim-execution-preflight-ssot.md"
USES_DESIGN="docs/development/current/main/design/declared-uses-capability-plan-mapping-ssot.md"
PURE_FIRST_SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
PLAN_OWNER="src/mir/effect_capability_plan.rs"
TESTS="src/tests/mir_effect_capability_plan.rs"
EXTERN_PLAN="src/mir/extern_call_route_plan.rs"
INC_DIR="lang/c-abi/shims"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking MIMAP-052B reclaim execution intent marker preflight"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$PREFLIGHT" \
  "$CARD" \
  "$DESIGN" \
  "$USES_DESIGN" \
  "$PURE_FIRST_SSOT" \
  "$PLAN_OWNER" \
  "$TESTS" \
  "$EXTERN_PLAN" \
  "$INDEX" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-052B card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "reclaim preflight SSOT must be accepted"
guard_expect_in_file "$TAG" 'uses alloc_reclaim' "$DESIGN" "SSOT must name source marker"
guard_expect_in_file "$TAG" 'hako.alloc.reclaim' "$DESIGN" "SSOT must name MIR capability marker"
guard_expect_in_file "$TAG" 'hako.alloc.reclaim' "$USES_DESIGN" "declared uses SSOT must list reclaim marker"
guard_expect_in_file "$TAG" '--reject-unsupported-reclaim-execution' "$DESIGN" "SSOT must name explicit preflight option"
guard_expect_in_file "$TAG" '--reject-unsupported-reclaim-execution' "$PREFLIGHT" "preflight tool must expose explicit reclaim rejection option"
guard_expect_in_file "$TAG" 'reclaim_execution_route_unsupported' "$PREFLIGHT" "preflight must emit stable reclaim unsupported reason"
guard_expect_in_file "$TAG" 'reclaim_execution_route_unsupported' "$PURE_FIRST_SSOT" "pure-first SSOT must document reclaim reason"
guard_expect_in_file "$TAG" '"alloc_reclaim" => Some\("hako\.alloc\.reclaim"\)' "$PLAN_OWNER" "declared uses mapping must own reclaim marker"
guard_expect_in_file "$TAG" 'source_declared_uses_emit_reclaim_execution_capability_marker' "$PLAN_OWNER" "unit test must fix source mapping"
guard_expect_in_file "$TAG" 'mir_transports_alloc_reclaim_declared_uses_as_capability_plan_id' "$TESTS" "integration test must fix MIR transport"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-052B guard"

if rg -n 'hako\.alloc\.reclaim|alloc_reclaim|reclaim_execution_route_unsupported' \
  "$EXTERN_PLAN" "$INC_DIR" >/tmp/"$TAG".route_leak 2>&1; then
  echo "[$TAG] ERROR: reclaim execution marker leaked into runtime/backend route tables" >&2
  cat /tmp/"$TAG".route_leak >&2
  rm -f /tmp/"$TAG".route_leak
  exit 1
fi
rm -f /tmp/"$TAG".route_leak

tmp_dir="$(mktemp -d /tmp/hakorune_reclaim_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

reclaim_json="$tmp_dir/reclaim.mir.json"
atomic_json="$tmp_dir/atomic.mir.json"
unreachable_json="$tmp_dir/unreachable-reclaim.mir.json"

cat >"$reclaim_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [],
      "metadata": {
        "capability_plans": [
          {
            "function": "main",
            "allow": ["hako.alloc.reclaim", "hako.atomic", "hako.osvm"],
            "verified": false,
            "source": "source_uses"
          }
        ],
        "lowering_plan": []
      }
    }
  ]
}
JSON

cat >"$atomic_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [],
      "metadata": {
        "capability_plans": [
          {
            "function": "main",
            "allow": ["hako.atomic", "hako.osvm"],
            "verified": false,
            "source": "source_uses"
          }
        ],
        "lowering_plan": []
      }
    }
  ]
}
JSON

cat >"$unreachable_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [],
      "metadata": {
        "capability_plans": [],
        "lowering_plan": []
      }
    },
    {
      "name": "Unused.reclaim/0",
      "params": [],
      "blocks": [],
      "metadata": {
        "capability_plans": [
          {
            "function": "Unused.reclaim/0",
            "allow": ["hako.alloc.reclaim"],
            "verified": false,
            "source": "source_uses"
          }
        ],
        "lowering_plan": []
      }
    }
  ]
}
JSON

"$PREFLIGHT" "$reclaim_json" >"$tmp_dir/default.out" 2>"$tmp_dir/default.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/default.out" "metadata-only reclaim marker must pass default preflight"

if "$PREFLIGHT" --reject-unsupported-reclaim-execution "$reclaim_json" >"$tmp_dir/reclaim.out" 2>"$tmp_dir/reclaim.err"; then
  echo "[$TAG] ERROR: explicit reclaim execution preflight should fail without an execution row" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'reason=reclaim_execution_route_unsupported' "$tmp_dir/reclaim.err" "reclaim preflight must classify unsupported execution"
guard_expect_in_file "$TAG" 'owner=capability_plans' "$tmp_dir/reclaim.err" "reclaim preflight must name capability owner"
guard_expect_in_file "$TAG" 'contract=metadata.capability_plans\[hako.alloc.reclaim\]' "$tmp_dir/reclaim.err" "reclaim preflight must name metadata contract"

"$PREFLIGHT" --reject-unsupported-reclaim-execution "$atomic_json" >"$tmp_dir/atomic.out" 2>"$tmp_dir/atomic.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/atomic.out" "generic atomic/osvm capabilities must not imply reclaim execution"

"$PREFLIGHT" --reject-unsupported-reclaim-execution "$unreachable_json" >"$tmp_dir/unreachable.out" 2>"$tmp_dir/unreachable.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/unreachable.out" "unreachable reclaim metadata must not fail pure-first route"

echo "[$TAG] ok"
