# Passive B0-L3b-S1 exact If identity-bundle contract.

guard_resolved_if_s1_contract() {
  local tag="$1"
  local root="$2"
  local module="$root/src/mir/resolved_semantics"
  local if_region="$module/if_region.rs"
  local product="$module/product.rs"
  local verifier="$module/verifier.rs"
  local authority_guard="$root/tools/checks/resolved_region_flow_authority_guard.sh"

  guard_require_files "$tag" \
    "$if_region" \
    "$module/if_region_tests.rs" \
    "$product" \
    "$verifier" \
    "$module/mod.rs" \
    "$module/README.md"

  for anchor in ResolvedIfRegionBundleV1 ResolvedIfRegionIndexV1 \
    ResolvedIfRegionLookupErrorV1 build_verified_if_region_index_v1 \
    'control_record.kind() != RegionKindV1::If {' 'RegionKindV1::IfThen' 'RegionKindV1::IfElse' \
    'ScopeKindV1::IfThen' 'ScopeKindV1::IfElse' \
    'SourcePathSegmentV1::IfThenBody' 'SourcePathSegmentV1::IfElseBody'; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$if_region" \
      "B0-L3b-S1 exact If bundle contract drifted: $anchor"
  done
  for anchor in 'if_regions: ResolvedIfRegionIndexV1' \
    'let if_regions = verify_resolved_function(&self.data)?' 'if_regions,'; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$product" \
      "B0-L3b-S1 verified-product index boundary drifted: $anchor"
  done
  guard_expect_fixed_in_file "$tag" 'build_verified_if_region_index_v1(data)' \
    "$verifier" "seal verifier no longer constructs the exact If index"
  guard_expect_fixed_in_file "$tag" 'B0-L3b-S1 exact If identity bundle' \
    "$module/README.md" "B0-L3b-S1 boundary documentation missing"

  if rg -n 'ASTNode|LocatedStmtV1|LocatedBodyV1|FunctionSourceViewV1|VerifiedSourceProjectionV1|ValueId|BasicBlockId|MirBuilder|resolved_region_flow|ResolvedFallthroughPortV1|VerifiedResolvedFunctionFlowV1|PostConditionEntry|BranchExit|falls_through|define_phi_final|lower_if_form' "$if_region"; then
    guard_fail "$tag" "B0-L3b-S1 passive identity product crossed a future flow/lower boundary"
  fi
  if rg -n 'ForeignOwner|Span|pointer|name lookup' "$if_region"; then
    guard_fail "$tag" "B0-L3b-S1 query invented a non-self-relative identity authority"
  fi

  python3 - "$product" "$if_region" <<'PY'
from pathlib import Path
import re
import sys

product = Path(sys.argv[1]).read_text()
if_region = Path(sys.argv[2]).read_text()

def body(name: str) -> str:
    match = re.search(rf"struct {name}\s*\{{(?P<body>.*?)\n\}}", product, re.S)
    if match is None:
        raise SystemExit(f"missing struct {name}")
    return match.group("body")

for draft_name in ("ResolvedFunctionDataV1", "ResolvedFunctionDraftV1"):
    if "if_regions" in body(draft_name):
        raise SystemExit(f"{draft_name} must remain free of the seal-derived If index")
if body("VerifiedResolvedFunctionV1").count("if_regions") != 1:
    raise SystemExit("VerifiedResolvedFunctionV1 must own exactly one private If index")
if "pub(super) if_regions: ResolvedIfRegionIndexV1" not in body("VerifiedResolvedFunctionV1"):
    raise SystemExit("verified If index visibility must remain private to resolved_semantics")

query = re.search(
    r"fn if_region_bundle\s*\((?P<args>.*?)\)\s*->(?P<ret>.*?)\{(?P<body>.*?)\n    \}",
    if_region,
    re.S,
)
if query is None:
    raise SystemExit("missing self-relative if_region_bundle query")
if "owner" in query.group("args") or "ForeignOwner" in query.group(0):
    raise SystemExit("if_region_bundle must remain self-relative")
if ".get(site)" not in query.group("body"):
    raise SystemExit("if_region_bundle must use the private verified index")
if ".regions" in query.group("body") or ".scopes" in query.group("body"):
    raise SystemExit("if_region_bundle must not rescan authoritative arenas")
PY

  local production_calls=""
  local caller_rc
  if production_calls="$(rg -n '\.if_region_bundle[[:space:]]*\(' "$root/src" \
    --glob '*.rs' --glob '!*_tests.rs' --glob '!tests.rs')"; then
    guard_fail "$tag" "B0-L3b-S1 query gained an early production caller: $production_calls"
  else
    caller_rc=$?
    if [[ "$caller_rc" != "1" ]]; then
      guard_fail "$tag" "B0-L3b-S1 production caller scan failed: rc=$caller_rc"
    fi
  fi

  local authority_lines
  authority_lines="$(wc -l < "$authority_guard" | tr -d '[:space:]')"
  if (( authority_lines >= 800 )); then
    guard_fail "$tag" "top resolved-region-flow authority guard reached 800 lines: $authority_lines"
  fi

  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_semantics::if_region_tests

  echo "if_region_s1_index_owner=verified-product-private"
  echo "if_region_s1_draft_index_fields=0"
  echo "if_region_s1_query=self-relative"
  echo "if_region_s1_control_cardinality=exactly-one"
  echo "if_region_s1_then_pair=required-exactly-one"
  echo "if_region_s1_else_pair=zero-or-one-arena-topology"
  echo "if_region_s1_orphan_records=0"
  echo "if_region_s1_query_production_callers=0"
  echo "if_region_s1_source_else_totality=deferred-S2"
  echo "if_region_s1_flow_connection=0"
  echo "if_region_s1_lower_connection=0"
  echo "if_region_s1_runtime_activation=0"
  echo "if_region_s1_selected_next_slice=B0-L3b-S2"
}
