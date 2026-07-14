#!/usr/bin/env bash

# D′ B0-L4-S2′ generic exact-source coverage. This helper owns the new
# carrier-free module manifest so the bounded public authority guard stays flat.

guard_resolved_control_flow_contract() {
  local tag="$1"
  local root="$2"
  local flow="$root/src/mir/resolved_control_flow"
  local compiler="$root/src/mir/compiler"
  local coverage="$flow/source_coverage.rs"
  local helper="${BASH_SOURCE[0]}"
  local authority_guard="$root/tools/checks/resolved_region_flow_authority_guard.sh"

  guard_require_files "$tag" \
    "$flow/README.md" \
    "$flow/mod.rs" \
    "$coverage" \
    "$flow/source_coverage_tests.rs" \
    "$compiler/README.md" \
    "$compiler/located.rs" \
    "$compiler/source_projection.rs" \
    "$compiler/source_view.rs" \
    "$compiler/source_view_tests.rs" \
    "$root/src/mir/mod.rs"

  local expected_manifest actual_manifest
  expected_manifest="$(printf '%s\n' README.md mod.rs source_coverage.rs source_coverage_tests.rs)"
  actual_manifest="$(find "$flow" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort)"
  if [[ "$actual_manifest" != "$expected_manifest" ]]; then
    printf '%s\n' "$actual_manifest" >&2
    guard_fail "$tag" "D′ S2′ resolved_control_flow manifest drifted; classify every entry"
  fi

  for spec in \
    'located.rs:ConsumedSourceRangeV1' \
    'located.rs:NonZeroU32' \
    'source_view.rs:suffix_first_stmt' \
    'source_view.rs:consumed_prefix' \
    'source_view.rs:advance_body_suffix' \
    'source_view.rs:u32::try_from' \
    'source_view.rs:checked_add'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$compiler/$file" \
      "D′ S2′ source transport contract drifted: $file:$anchor"
  done
  for spec in \
    'source_coverage.rs:CoveredSourceSiteV1' \
    'source_coverage.rs:VerifiedLocatedSourceCoverageV1' \
    'source_coverage.rs:SourceCoverageVerificationErrorV1' \
    'source_coverage.rs:verify_located_source_coverage_v1' \
    'source_coverage.rs:ForeignOuterOwner' \
    'source_coverage.rs:ForeignCoveredOwner' \
    'source_coverage.rs:EmptyPreorder' \
    'source_coverage.rs:DuplicateSite' \
    'README.md:B0-L4-S2′ generic located source coverage'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$flow/$file" \
      "D′ S2′ coverage contract drifted: $file:$anchor"
  done
  guard_expect_fixed_in_file "$tag" 'pub(crate) mod resolved_control_flow;' \
    "$root/src/mir/mod.rs" "D′ S2′ MIR module boundary missing"
  guard_expect_fixed_in_file "$tag" 'coverage schema in `resolved_control_flow` verifies' \
    "$compiler/README.md" "D′ S2′ compiler transport boundary documentation missing"

  if rg -n 'ASTNode|\bSpan\b|ValueId|BasicBlockId|MirBuilder|Planner|CorePlan|LoopRouteContext|variable_map|may_rebind|carrier|\bPHI\b|lower_if_form' \
    "$coverage"; then
    guard_fail "$tag" "D′ S2′ coverage crossed a syntax/materialization/effect boundary"
  fi
  if rg -n 'as u32' "$compiler/source_view.rs"; then
    guard_fail "$tag" "D′ S2′ source navigation reintroduced unchecked usize-to-u32 conversion"
  fi
  if rg -n 'verify_located_source_coverage_v1\(' "$flow" \
    --glob '!source_coverage.rs' --glob '!source_coverage_tests.rs'; then
    guard_fail "$tag" "D′ S2′ coverage verifier gained a production consumer"
  fi
  if rg -n 'ConsumedSourceRangeV1|VerifiedLocatedSourceCoverageV1|CoveredSourceSiteV1' \
    "$root/src/mir/builder" "$root/src/mir/join_ir" "$root/src/mir/resolved_region_flow"; then
    guard_fail "$tag" "D′ S2′ coverage leaked into existing production lowering"
  fi

  python3 - "$compiler/located.rs" "$coverage" "$flow/mod.rs" <<'PY'
from pathlib import Path
import re
import sys

located = Path(sys.argv[1]).read_text()
coverage = Path(sys.argv[2]).read_text()
module = Path(sys.argv[3]).read_text()

def struct_body(text: str, name: str) -> str:
    match = re.search(rf"struct {name}[^{{]*\{{(?P<body>.*?)\n\}}", text, re.S)
    if match is None:
        raise SystemExit(f"missing struct {name}")
    return match.group("body")

range_body = struct_body(located, "ConsumedSourceRangeV1")
for exact in ("body: SourceBodySiteV1", "start: u32", "count: NonZeroU32"):
    if exact not in range_body:
        raise SystemExit(f"ConsumedSourceRangeV1 missing private exact field: {exact}")
if re.search(r"\bpub(?:\([^)]*\))?\s+(body|start|count):", range_body):
    raise SystemExit("ConsumedSourceRangeV1 fields must remain private")

verified = struct_body(coverage, "VerifiedLocatedSourceCoverageV1")
if re.search(r"\bpub(?:\([^)]*\))?\s+(outer|preorder):", verified):
    raise SystemExit("verified coverage fields must remain private")
header = coverage.split("pub(super) struct VerifiedLocatedSourceCoverageV1", 1)[0]
derive = header.rsplit("#[derive(", 1)[-1].split(")]", 1)[0]
if "Clone" in derive:
    raise SystemExit("verified coverage product must not implement Clone")
if re.search(r"fn\s+into_parts\b", coverage):
    raise SystemExit("verified coverage must not separate range from preorder")
if re.search(r"pub(?:\(crate\))?\s+fn verify_located_source_coverage_v1", coverage):
    raise SystemExit("generic coverage verifier must remain module-private")
for carrier in ("LocatedBodyV1", "LocatedStmtV1", "LocatedExprV1"):
    if carrier not in coverage:
        raise SystemExit(f"owner-branded coverage constructor missing {carrier}")
if "pub(crate) use" in module or "pub use" in module:
    raise SystemExit("disconnected coverage module must not re-export its product")
PY

  local file lines
  for file in \
    "$compiler/located.rs" \
    "$compiler/source_projection.rs" \
    "$compiler/source_view.rs" \
    "$compiler/source_view_tests.rs" \
    "$coverage" \
    "$flow/source_coverage_tests.rs" \
    "$helper"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "D′ S2′ source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
  lines="$(wc -l < "$authority_guard" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$tag" "top-level authority guard reached the 800-line stop boundary: $authority_guard ($lines)"
  fi

  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::compiler::source_view_tests
  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_control_flow::source_coverage_tests

  echo "resolved_control_flow_s2prime_range=owner-body-start-nonzero-count"
  echo "resolved_control_flow_s2prime_site_identity=located-carriers-only"
  echo "resolved_control_flow_s2prime_verified_clone=0"
  echo "resolved_control_flow_s2prime_effect_rows=0"
  echo "resolved_control_flow_s2prime_production_consumers=0"
  echo "resolved_control_flow_s2prime_runtime_activation=0"
}
