#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pure-first-route-preflight"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PREFLIGHT="tools/checks/pure_first_route_preflight.py"
PURE_FIRST_LIB="tools/checks/lib/pure_first_exe_guard.sh"
SSOT="docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md"
LOWERING_SSOT="docs/development/current/main/design/lowering-plan-json-v0-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-451-MIR-ROUTE-PREFLIGHT-001-LOWERING-PLAN-PREFLIGHT.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] checking lowering_plan preflight"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$PREFLIGHT" \
  "$PURE_FIRST_LIB" \
  "$SSOT" \
  "$LOWERING_SSOT" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'metadata.lowering_plan' "$LOWERING_SSOT" "LoweringPlan JSON SSOT must own metadata.lowering_plan"
guard_expect_in_file "$TAG" 'pure_first_route_preflight.py' "$SSOT" "pure-first SSOT must name the preflight tool"
guard_expect_in_file "$TAG" 'pure_first_route_preflight.py' "$CARD" "451 card must name the preflight tool"
guard_expect_in_file "$TAG" 'pure_first_route_preflight.py' "$PURE_FIRST_LIB" "pure-first EXE helper must run route preflight"
guard_expect_in_file "$TAG" 'tools/checks/pure_first_route_preflight_guard.sh' "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_route_preflight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

hit_json="$tmp_dir/hit.mir.json"
missing_json="$tmp_dir/missing.mir.json"
unsupported_json="$tmp_dir/unsupported.mir.json"
generated_json="$tmp_dir/generated.mir.json"

cat >"$hit_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {
              "op": "mir_call",
              "dst": 2,
              "mir_call": {
                "callee": {"type": "Extern", "name": "hako_mem_alloc"},
                "args": [1],
                "effects": ["IO"],
                "flags": {}
              }
            }
          ]
        }
      ],
      "metadata": {
        "lowering_plan": [
          {
            "site": "b0.i0",
            "block": 0,
            "instruction_index": 0,
            "source": "extern_call_routes",
            "source_route_id": "extern.hako_mem.alloc",
            "source_symbol": "hako_mem_alloc",
            "core_op": "HakoMemAlloc",
            "tier": "ColdRuntime",
            "emit_kind": "runtime_call",
            "symbol": "hako_mem_alloc",
            "proof": "extern_registry",
            "route_proof": "extern_registry",
            "route_kind": "extern.hako_mem.alloc",
            "perf_proof": false,
            "arity": 1,
            "key_value": 1,
            "result_value": 2,
            "return_shape": "native_ptr_nullable",
            "value_demand": "native_ptr_nullable",
            "effects": ["hako.mem.alloc"]
          }
        ]
      }
    }
  ]
}
JSON

cat >"$missing_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {
              "op": "mir_call",
              "dst": 2,
              "mir_call": {
                "callee": {"type": "Extern", "name": "hako_mem_alloc"},
                "args": [1],
                "effects": ["IO"],
                "flags": {}
              }
            }
          ]
        }
      ],
      "metadata": {"lowering_plan": []}
    }
  ]
}
JSON

cat >"$unsupported_json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "params": [],
      "blocks": [],
      "metadata": {
        "lowering_plan": [
          {
            "site": "b0.i0",
            "block": 0,
            "instruction_index": 0,
            "source": "global_call_routes",
            "source_route_id": "global.user",
            "callee_name": "User.call/0",
            "core_op": "UserGlobalCall",
            "tier": "Unsupported",
            "emit_kind": "unsupported",
            "symbol": null,
            "proof": "typed_global_call_generic_i64",
            "route_proof": "typed_global_call_generic_i64",
            "route_kind": "same_module_global",
            "perf_proof": false,
            "arity": 0,
            "target_exists": true,
            "arity_matches": true,
            "result_value": 1,
            "return_shape": null,
            "value_demand": null,
            "reason": "missing_multi_function_emitter",
            "effects": []
          }
        ]
      }
    }
  ]
}
JSON

"$PREFLIGHT" "$hit_json" >"$tmp_dir/hit.out" 2>"$tmp_dir/hit.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/hit.out" "supported route should pass"

if "$PREFLIGHT" "$missing_json" >"$tmp_dir/missing.out" 2>"$tmp_dir/missing.err"; then
  echo "[$TAG] ERROR: missing lowering plan should fail" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'reason=lowering_plan_missing' "$tmp_dir/missing.err" "missing route must classify lowering_plan_missing"
guard_expect_in_file "$TAG" 'callee=hako_mem_alloc' "$tmp_dir/missing.err" "missing route must include callee"

if "$PREFLIGHT" "$unsupported_json" >"$tmp_dir/unsupported.out" 2>"$tmp_dir/unsupported.err"; then
  echo "[$TAG] ERROR: unsupported lowering plan should fail" >&2
  exit 1
fi
guard_expect_in_file "$TAG" 'reason=unsupported_tier' "$tmp_dir/unsupported.err" "unsupported route must classify unsupported_tier"
guard_expect_in_file "$TAG" 'owner=global_call_routes' "$tmp_dir/unsupported.err" "unsupported route must include owner"

cargo build -q --bin hakorune
NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/debug/hakorune" --backend mir --emit-mir-json "$generated_json" \
  apps/hako-mem-extern-exe-proof/main.hako >/dev/null
"$PREFLIGHT" "$generated_json" >"$tmp_dir/generated.out" 2>"$tmp_dir/generated.err"
guard_expect_in_file "$TAG" '\[pure-first-route\]\[ok\]' "$tmp_dir/generated.out" "generated MIR route preflight should pass"

echo "[$TAG] ok"
