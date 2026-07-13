# B0-L3b exact If identity and verified pre-Builder flow contracts.

guard_resolved_if_lowering_contract() {
  guard_resolved_if_s1_contract_impl "$1" "$2"
  guard_resolved_if_s2_contract_impl "$1" "$2"
}

guard_resolved_if_s1_contract_impl() {
  local tag="$1"
  local root="$2"
  local module="$root/src/mir/resolved_semantics"
  local if_region="$module/if_region.rs"
  local product="$module/product.rs"
  local verifier="$module/verifier.rs"

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

  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_semantics::if_region_tests

  echo "if_region_s1_index_owner=verified-product-private"
  echo "if_region_s1_draft_index_fields=0"
  echo "if_region_s1_query=self-relative"
  echo "if_region_s1_control_cardinality=exactly-one"
  echo "if_region_s1_then_pair=required-exactly-one"
  echo "if_region_s1_else_pair=zero-or-one-arena-topology"
  echo "if_region_s1_orphan_records=0"
}

guard_resolved_if_s2_contract_impl() {
  local tag="$1"
  local root="$2"
  local flow="$root/src/mir/resolved_region_flow"
  local flow_test="$flow/if_flow_tests.rs"
  local authority_guard="$root/tools/checks/resolved_region_flow_authority_guard.sh"
  local helper="${BASH_SOURCE[0]}"

  if [[ ! -d "$flow" ]]; then
    guard_fail "$tag" "B0-L3b-S2 resolved_region_flow directory missing"
  fi
  guard_require_files "$tag" \
    "$flow/README.md" \
    "$flow/analyzer.rs" \
    "$flow/coverage.rs" \
    "$flow/if_flow.rs" \
    "$flow_test" \
    "$flow/mod.rs" \
    "$flow/ports.rs" \
    "$flow/verifier.rs" \
    "$root/src/mir/mod.rs"

  local expected_manifest actual_manifest
  expected_manifest="$(printf '%s\n' \
    README.md analyzer.rs coverage.rs if_flow.rs if_flow_tests.rs mod.rs ports.rs verifier.rs)"
  actual_manifest="$(find "$flow" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort)"
  if [[ "$actual_manifest" != "$expected_manifest" ]]; then
    printf '%s\n' "$actual_manifest" >&2
    guard_fail "$tag" "B0-L3b-S2 source manifest drifted; classify every entry"
  fi
  guard_expect_fixed_in_file "$tag" 'pub(crate) mod resolved_region_flow;' \
    "$root/src/mir/mod.rs" "B0-L3b-S2 MIR module boundary missing"

  local -a production_files
  mapfile -t production_files < <(
    find "$flow" -maxdepth 1 -type f -name '*.rs' ! -name '*_tests.rs' -print | LC_ALL=C sort
  )
  if rg -n 'ValueId|BasicBlockId|MirBuilder|Planner|CorePlan|JoinIR|crate::mir::builder|crate::mir::join_ir|variable_map|LexicalScopeGuard|lower_if_form|lower_if_form_with_condition_value|emit_conditional_edgecfg|build_expression|build_statement|ASTNode::Program|define_phi_final|publish_join_value' \
    "${production_files[@]}"; then
    guard_fail "$tag" "B0-L3b-S2 flow crossed a Builder/Planner/materialization boundary"
  fi
  if rg -n 'compiler::(capability|lowering_input|module_session)|MirCompiler|ResolvedModuleLoweringInputV1|CanonicalLoweringPreflightV1|CanonicalModuleLoweringSessionV1' \
    "${production_files[@]}"; then
    guard_fail "$tag" "B0-L3b-S2 flow imported compiler orchestration instead of source leaves"
  fi
  if rg -n 'falls_through|ptr::eq|as_ptr|HashMap<String|BTreeMap<String|name lookup' \
    "${production_files[@]}"; then
    guard_fail "$tag" "B0-L3b-S2 flow invented bool fallthrough or representation identity"
  fi

  for spec in \
    'ports.rs:ResolvedFallthroughPortV1' \
    'ports.rs:ResolvedElseFallthroughV1' \
    'ports.rs:ResolvedIfJoinBindingV1' \
    'ports.rs:ResolvedIfPortValueSourceV1' \
    'ports.rs:PostConditionEntry' \
    'ports.rs:BranchExit' \
    'if_flow.rs:VerifiedResolvedIfFlowV1' \
    'if_flow.rs:VerifiedResolvedFunctionFlowV1' \
    'if_flow.rs:Box<[VerifiedResolvedIfFlowV1]>' \
    'coverage.rs:VerifiedIfFlowCoverageV1' \
    'analyzer.rs:ResolvedFunctionLoweringInputV1' \
    'verifier.rs:.if_region_bundle('; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$flow/$file" \
      "B0-L3b-S2 contract drifted: $file:$anchor"
  done

  python3 - "$flow/if_flow.rs" "$flow/ports.rs" <<'PY'
from pathlib import Path
import re
import sys

flow = Path(sys.argv[1]).read_text()
ports = Path(sys.argv[2]).read_text()

def item(text: str, kind: str, name: str) -> tuple[str, str]:
    match = re.search(rf"\b{kind}\s+{name}(?P<header>[^{{]*)\{{", text)
    if match is None:
        raise SystemExit(f"missing {kind} {name}")
    start = match.end() - 1
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return match.group("header"), text[start + 1:index]
    raise SystemExit(f"unterminated {kind} {name}")

function_header, function_body = item(flow, "struct", "VerifiedResolvedFunctionFlowV1")
if "<" in function_header or "&" in function_body:
    raise SystemExit("VerifiedResolvedFunctionFlowV1 must remain lifetime-free and owned")
if "Box<[VerifiedResolvedIfFlowV1]>" not in function_body:
    raise SystemExit("function flow must publish one source-preorder boxed slice")
if "BTreeMap" in function_body or "HashMap" in function_body:
    raise SystemExit("function flow must not publish a second map-order authority")

if_header, if_body = item(flow, "struct", "VerifiedResolvedIfFlowV1")
if "<" in if_header or "&" in if_body:
    raise SystemExit("VerifiedResolvedIfFlowV1 must remain lifetime-free and owned")
for field in (
    "site", "regions", "condition_effects", "then_port", "else_port", "join", "coverage"
):
    if re.search(rf"\b{field}\s*:", if_body) is None:
        raise SystemExit(f"VerifiedResolvedIfFlowV1 missing owned field: {field}")

_, port_body = item(ports, "struct", "ResolvedFallthroughPortV1")
if re.search(r"\bmay_rebind_outer\s*:\s*Box<\[BindingRefV1\]>", port_body) is None:
    raise SystemExit("fallthrough port must carry a typed BindingRef slice")
if re.search(r"\bbool\b|\bfalls_through\b|&", port_body):
    raise SystemExit("fallthrough port must remain typed, owned, and bool-free")

_, else_body = item(ports, "enum", "ResolvedElseFallthroughV1")
for variant in ("ImplicitIdentity", "Explicit(ResolvedFallthroughPortV1)"):
    if variant not in re.sub(r"\s+", "", else_body):
        raise SystemExit(f"missing typed else variant: {variant}")

_, source_body = item(ports, "enum", "ResolvedIfPortValueSourceV1")
for variant in ("PostConditionEntry", "BranchExit"):
    if variant not in source_body:
        raise SystemExit(f"missing exact join source variant: {variant}")
if re.search(r"(?<!PostCondition)\bEntry\b", source_body):
    raise SystemExit("ambiguous Entry join source is forbidden")

_, join_binding_body = item(ports, "struct", "ResolvedIfJoinBindingV1")
for field in ("binding", "then_source", "else_source"):
    if re.search(rf"\b{field}\s*:", join_binding_body) is None:
        raise SystemExit(f"join binding missing exact source field: {field}")
_, join_body = item(ports, "struct", "ResolvedIfJoinContractV1")
if re.search(r"\brows\s*:\s*Box<\[ResolvedIfJoinBindingV1\]>", join_body) is None:
    raise SystemExit("join contract must own the ordered join-row authority")
PY

  local query_calls="" caller_rc query_count
  if query_calls="$(rg -n '\.if_region_bundle[[:space:]]*\(' "$root/src" \
    --glob '*.rs' --glob '!*_tests.rs' --glob '!tests.rs')"; then
    query_count="$(printf '%s\n' "$query_calls" | wc -l | tr -d '[:space:]')"
    if [[ "$query_count" != "1" || "$query_calls" != "$flow/"* ]]; then
      guard_fail "$tag" "B0-L3b-S2 semantic query must have exactly one RegionFlow caller: $query_calls"
    fi
  else
    caller_rc=$?
    if [[ "$caller_rc" != "1" ]]; then
      guard_fail "$tag" "B0-L3b-S2 semantic query caller scan failed: rc=$caller_rc"
    fi
    guard_fail "$tag" "B0-L3b-S2 semantic query has no RegionFlow caller"
  fi

  local flow_consumers=""
  if flow_consumers="$(rg -l 'VerifiedResolvedFunctionFlowV1|analyze_resolved_function_flow_v1' \
    "$root/src" --glob '*.rs' --glob '!*_tests.rs' --glob '!tests.rs')"; then
    while IFS= read -r consumer; do
      [[ -z "$consumer" ]] && continue
      case "$consumer" in
        "$flow"/*) ;;
        *) guard_fail "$tag" "B0-L3b-S2 flow gained an early production consumer: $consumer" ;;
      esac
    done <<< "$flow_consumers"
  else
    caller_rc=$?
    if [[ "$caller_rc" != "1" ]]; then
      guard_fail "$tag" "B0-L3b-S2 flow consumer scan failed: rc=$caller_rc"
    fi
  fi

  local file lines
  for file in "$authority_guard" "$helper" "${production_files[@]}" "$flow_test"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "B0-L3b guard source reached the 800-line stop boundary: $file ($lines)"
    fi
  done

  cargo test -q --manifest-path "$root/Cargo.toml" --lib mir::resolved_region_flow

  echo "if_region_s2_manifest=closed"
  echo "if_region_s2_input=owner-closed"
  echo "if_region_s2_product=lifetime-free"
  echo "if_region_s2_publication_order=source-preorder"
  echo "if_region_s2_fallthrough_ports=typed-no-bool"
  echo "if_region_s2_else_mode=implicit-identity-or-explicit"
  echo "if_region_s2_join_sources=post-condition-entry-or-branch-exit"
  echo "if_region_s2_condition_effects=separate"
  echo "if_region_s2_branch_local_join_rows=0"
  echo "if_region_s2_nested_composition=postorder-child-summary"
  echo "if_region_s2_assignment_coverage=exact-once"
  echo "if_region_s2_bundle_flow_bijection=verified"
  echo "if_region_s2_semantic_query_production_callers=1-regionflow"
  echo "if_region_s2_flow_production_callers=0"
  echo "if_region_s2_builder_connection=0"
  echo "if_region_s2_lower_connection=0"
  echo "if_region_s2_runtime_activation=0"
  echo "if_region_s2_selected_next_slice=B0-L3b-I1a"
}
