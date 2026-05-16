#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-random-capability-preflight"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PREFLIGHT="tools/checks/pure_first_route_preflight.py"
CARD="docs/development/current/main/phases/phase-293x/293x-533-RANDOM-CAP-002-RANDOM-UNSUPPORTED-ROUTE-PREFLIGHT.md"
DESIGN="docs/development/current/main/design/random-capability-preflight-ssot.md"
FAILFAST_DESIGN="docs/development/current/main/design/random-capability-failfast-ssot.md"
PURE_FIRST_SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
EXTERN_PLAN="src/mir/extern_call_route_plan.rs"
KERNEL_EXPORTS="crates/nyash_kernel/src/exports"
RUNTIME_SUBSTRATE="lang/src/runtime/substrate"
INC_DIR="lang/c-abi/shims"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking RANDOM-CAP-002 unsupported random preflight"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$PREFLIGHT" \
  "$CARD" \
  "$DESIGN" \
  "$FAILFAST_DESIGN" \
  "$PURE_FIRST_SSOT" \
  "$EXTERN_PLAN" \
  "$INDEX" \
  "$0"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "RANDOM-CAP-002 card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "random preflight SSOT must be accepted"
guard_expect_in_file "$TAG" '--reject-unsupported-random' "$DESIGN" "SSOT must name explicit preflight option"
guard_expect_in_file "$TAG" '--reject-unsupported-random' "$PREFLIGHT" "preflight tool must expose explicit random rejection option"
guard_expect_in_file "$TAG" 'random_capability_route_unsupported' "$PREFLIGHT" "preflight must emit stable random unsupported reason"
guard_expect_in_file "$TAG" 'random_capability_route_unsupported' "$PURE_FIRST_SSOT" "pure-first SSOT must document reason"
guard_expect_in_file "$TAG" 'metadata.capability_plans' "$DESIGN" "SSOT must fix capability contract"
guard_expect_in_file "$TAG" 'hako.random' "$DESIGN" "SSOT must fix random capability id"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list RANDOM-CAP-002 guard"

if rg -n 'hako_random|hako_entropy|random_source|entropy_source|/dev/urandom|getrandom' \
  "$EXTERN_PLAN" "$KERNEL_EXPORTS" "$RUNTIME_SUBSTRATE" "$INC_DIR" \
  >/tmp/"$TAG".random_route 2>&1; then
  echo "[$TAG] ERROR: random/entropy route leaked into runtime/backend code" >&2
  cat /tmp/"$TAG".random_route >&2
  rm -f /tmp/"$TAG".random_route
  exit 1
fi
rm -f /tmp/"$TAG".random_route

tmp_dir="$(mktemp -d /tmp/hakorune_random_cap_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

random_json="$tmp_dir/random.mir.json"
mem_json="$tmp_dir/mem.mir.json"
unreachable_json="$tmp_dir/unreachable-random.mir.json"

cat >"$random_json" <<'JSON'
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
            "allow": ["hako.random"],
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

cat >"$mem_json" <<'JSON'
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
            "allow": ["hako.mem"],
            "verified": false,
            "source": "rune_profile"
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
      "name": "Unused.secure/0",
      "params": [],
      "blocks": [],
      "metadata": {
        "capability_plans": [
          {
            "function": "Unused.secure/0",
            "allow": ["hako.random"],
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

"$PREFLIGHT" "$random_json" >"$tmp_dir/default.out" 2>"$tmp_dir/default.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/default.out" "metadata-only random must pass default preflight"

if "$PREFLIGHT" --reject-unsupported-random "$random_json" >"$tmp_dir/random.out" 2>"$tmp_dir/random.err"; then
  echo "[$TAG] ERROR: explicit random execution preflight should fail without a route" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'reason=random_capability_route_unsupported' "$tmp_dir/random.err" "random route preflight must classify unsupported route"
guard_expect_in_file "$TAG" 'owner=capability_plans' "$tmp_dir/random.err" "random route preflight must name capability owner"
guard_expect_in_file "$TAG" 'contract=metadata.capability_plans\[hako.random\]' "$tmp_dir/random.err" "random route preflight must name metadata contract"

"$PREFLIGHT" --reject-unsupported-random "$mem_json" >"$tmp_dir/mem.out" 2>"$tmp_dir/mem.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/mem.out" "non-random capabilities must remain unaffected"

"$PREFLIGHT" --reject-unsupported-random "$unreachable_json" >"$tmp_dir/unreachable.out" 2>"$tmp_dir/unreachable.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/unreachable.out" "unreachable random metadata must not fail pure-first route"

echo "[$TAG] ok"
