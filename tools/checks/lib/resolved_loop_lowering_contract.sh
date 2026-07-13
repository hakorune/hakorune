#!/usr/bin/env bash

# B0-L4 canonical Loop contract. Extend this reusable helper as S2-I2 land;
# do not add per-slice calls to the bounded top-level authority guard.

guard_resolved_loop_lowering_contract() {
  guard_resolved_loop_s1_contract_impl "$1" "$2"
}

guard_resolved_loop_s1_contract_impl() {
  local tag="$1"
  local root="$2"
  local module="$root/src/mir/resolved_semantics"
  local loop_region="$module/loop_region.rs"
  local product="$module/product.rs"
  local verifier="$module/verifier.rs"
  local authority_guard="$root/tools/checks/resolved_region_flow_authority_guard.sh"
  local helper="${BASH_SOURCE[0]}"

  guard_require_files "$tag" \
    "$loop_region" \
    "$module/loop_region_tests.rs" \
    "$product" \
    "$verifier" \
    "$module/mod.rs" \
    "$module/README.md"

  for anchor in ResolvedLoopRegionBundleV1 ResolvedLoopRegionIndexV1 \
    ResolvedLoopRegionLookupErrorV1 build_verified_loop_region_index_v1 \
    'region_record.kind() != RegionKindV1::Loop {' \
    'ScopeKindV1::LoopBody' 'SourcePathSegmentV1::LoopBodyRoot' \
    'exact_source_region_v1(data, site.node())'; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$loop_region" \
      "B0-L4-S1 exact Loop bundle contract drifted: $anchor"
  done
  for anchor in 'loop_regions: ResolvedLoopRegionIndexV1' \
    'let derived = verify_resolved_function(&self.data)?' \
    'loop_regions: derived.loop_regions'; do
    guard_expect_fixed_in_file "$tag" "$anchor" "$product" \
      "B0-L4-S1 verified-product index boundary drifted: $anchor"
  done
  guard_expect_fixed_in_file "$tag" 'build_verified_loop_region_index_v1(data)' \
    "$verifier" "seal verifier no longer constructs the exact Loop index"
  guard_expect_fixed_in_file "$tag" 'B0-L4-S1 exact Loop identity bundle' \
    "$module/README.md" "B0-L4-S1 boundary documentation missing"

  if rg -n 'ASTNode|LocatedStmtV1|LocatedBodyV1|LocatedBodySuffixV1|FunctionSourceViewV1|VerifiedSourceProjectionV1|ValueId|BasicBlockId|MirBuilder|resolved_region_flow|CorePlan|LoopRouteContext|BindingRef|PHI|carrier|Port|lower_' \
    "$loop_region"; then
    guard_fail "$tag" "B0-L4-S1 passive identity product crossed a future flow/lower boundary"
  fi
  if rg -n 'ForeignOwner|Span|pointer|name lookup' "$loop_region"; then
    guard_fail "$tag" "B0-L4-S1 query invented a non-self-relative identity authority"
  fi
  if rg -n 'loop_region_bundle|ResolvedLoopRegionBundleV1' \
    "$root/src/mir/resolved_region_flow" "$root/src/mir/compiler" "$root/src/mir/builder"; then
    guard_fail "$tag" "B0-L4-S1 must remain disconnected from RegionFlow/compiler/Builder"
  fi

  python3 - "$product" "$loop_region" <<'PY'
from pathlib import Path
import re
import sys

product = Path(sys.argv[1]).read_text()
loop_region = Path(sys.argv[2]).read_text()

def body(name: str) -> str:
    match = re.search(rf"struct {name}\s*\{{(?P<body>.*?)\n\}}", product, re.S)
    if match is None:
        raise SystemExit(f"missing struct {name}")
    return match.group("body")

for draft_name in ("ResolvedFunctionDataV1", "ResolvedFunctionDraftV1"):
    if "loop_regions" in body(draft_name):
        raise SystemExit(f"{draft_name} must remain free of the seal-derived Loop index")
verified = body("VerifiedResolvedFunctionV1")
if verified.count("loop_regions") != 1:
    raise SystemExit("VerifiedResolvedFunctionV1 must own exactly one private Loop index")
if "pub(super) loop_regions: ResolvedLoopRegionIndexV1" not in verified:
    raise SystemExit("verified Loop index visibility must remain private to resolved_semantics")

query = re.search(
    r"fn loop_region_bundle\s*\((?P<args>.*?)\)\s*->(?P<ret>.*?)\{(?P<body>.*?)\n    \}",
    loop_region,
    re.S,
)
if query is None:
    raise SystemExit("missing self-relative loop_region_bundle query")
if "owner" in query.group("args") or "ForeignOwner" in query.group(0):
    raise SystemExit("loop_region_bundle must remain self-relative")
if ".get(site)" not in query.group("body"):
    raise SystemExit("loop_region_bundle must use the private verified index")
if ".regions" in query.group("body") or ".scopes" in query.group("body"):
    raise SystemExit("loop_region_bundle must not rescan authoritative arenas")
PY

  local file lines
  for file in "$loop_region" "$module/loop_region_tests.rs" "$helper"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "B0-L4-S1 source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
  lines="$(wc -l < "$authority_guard" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$tag" "top-level authority guard reached the 800-line stop boundary: $authority_guard ($lines)"
  fi

  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_semantics::loop_region_tests

  echo "loop_region_s1_index_owner=verified-product-private"
  echo "loop_region_s1_draft_index_fields=0"
  echo "loop_region_s1_query=self-relative"
  echo "loop_region_s1_pair_cardinality=exactly-one"
  echo "loop_region_s1_orphan_records=0"
  echo "loop_region_s1_regionflow_callers=0"
  echo "loop_region_s1_builder_callers=0"
  echo "loop_region_s1_runtime_activation=0"
}
