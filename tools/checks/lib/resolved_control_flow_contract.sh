#!/usr/bin/env bash

# D′ pre-Builder control-only products. This helper owns the bounded module
# manifest so exact source coverage and completion transport cannot drift.

guard_resolved_control_flow_contract() {
  local tag="$1"
  local root="$2"
  local flow="$root/src/mir/resolved_control_flow"
  local compiler="$root/src/mir/compiler"
  local coverage="$flow/source_coverage.rs"
  local completion="$flow/function_control.rs"
  local cleanup="$flow/cleanup.rs"
  local if_control="$flow/if_control.rs"
  local if_control_tests="$flow/if_control_tests.rs"
  local lowering="$root/src/mir/builder/resolved_lowering"
  local draft_seal="$lowering/draft_seal.rs"
  local session_terminal="$root/src/mir/builder/calls/function_session/terminal.rs"
  local helper="${BASH_SOURCE[0]}"
  local authority_guard="$root/tools/checks/resolved_region_flow_authority_guard.sh"

  guard_require_files "$tag" \
    "$flow/README.md" \
    "$cleanup" \
    "$completion" \
    "$flow/function_control_tests.rs" \
    "$if_control" \
    "$if_control_tests" \
    "$flow/mod.rs" \
    "$coverage" \
    "$flow/source_coverage_tests.rs" \
    "$compiler/README.md" \
    "$compiler/located.rs" \
    "$compiler/source_projection.rs" \
    "$compiler/source_view.rs" \
    "$compiler/source_view_tests.rs" \
    "$draft_seal" \
    "$session_terminal" \
    "$root/src/mir/mod.rs"

  local expected_manifest actual_manifest
  expected_manifest="$(printf '%s\n' README.md cleanup.rs function_control.rs function_control_tests.rs if_control.rs if_control_tests.rs mod.rs source_coverage.rs source_coverage_tests.rs)"
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
  local coverage_consumers=""
  coverage_consumers="$(rg -l 'verify_located_source_coverage_v1\(' "$flow" \
    --glob '!source_coverage.rs' --glob '!source_coverage_tests.rs' \
    --glob '!if_control_tests.rs' || true)"
  if [[ "$coverage_consumers" != "$if_control" ]]; then
    guard_fail "$tag" "D′ SSA-S3 generic coverage must have exactly one internal family consumer: $coverage_consumers"
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
if "VerifiedLocatedSourceCoverageV1" in module:
    raise SystemExit("disconnected coverage product must not be re-exported")
PY

  for spec in \
    'if_control.rs:VerifiedResolvedFunctionIfControlV1' \
    'if_control.rs:VerifiedLocatedIfControlV1' \
    'if_control.rs:ResolvedIfFallthroughPortV1' \
    'if_control.rs:ResolvedIfElsePortV1' \
    'if_control.rs:ImplicitIdentity' \
    'if_control.rs:VerifiedLocatedSourceCoverageV1' \
    'if_control.rs:IfControlCoverageUseV1' \
    'if_control.rs:CoveragePartitionOverlap' \
    'if_control.rs:UnsupportedStatement' \
    'README.md:SSA-S3 carrier-free If control'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$flow/$file" \
      "D′ SSA-S3 If control contract drifted: $file:$anchor"
  done
  if rg -n 'BindingRefV1|may_rebind|ResolvedIfJoin|ValueId|BasicBlockId|MirBuilder|variable_map|lower_if_form|resolved_region_flow|CorePlan|LoopRouteContext|HashMap<String|BTreeMap<String' \
    "$if_control"; then
    guard_fail "$tag" "D′ SSA-S3 If control crossed the binding-effect/materialization boundary"
  fi
  if rg -n 'analyze_resolved_if_control_v1' "$root/src" \
    --glob '*.rs' --glob '!if_control.rs' --glob '!if_control_tests.rs'; then
    guard_fail "$tag" "D′ SSA-S3 disconnected analyzer gained a production caller"
  fi

  python3 - "$if_control" "$flow/mod.rs" <<'PY'
from pathlib import Path
import re
import sys

control = Path(sys.argv[1]).read_text()
module = Path(sys.argv[2]).read_text()

def struct_body(name: str) -> str:
    match = re.search(rf"struct {name}[^{{]*\{{(?P<body>.*?)\n\}}", control, re.S)
    if match is None:
        raise SystemExit(f"missing struct {name}")
    return match.group("body")

for name in ("VerifiedLocatedIfControlV1", "VerifiedResolvedFunctionIfControlV1"):
    body = struct_body(name)
    if re.search(r"\bpub(?:\([^)]*\))?\s+\w+\s*:", body):
        raise SystemExit(f"{name} fields must remain private")
    prefix = control.split(f"struct {name}", 1)[0]
    derive = prefix.rsplit("#[derive(", 1)[-1].split(")]", 1)[0]
    if "Clone" in derive:
        raise SystemExit(f"{name} must not implement Clone")

row = struct_body("VerifiedLocatedIfControlV1")
for field in ("site", "regions", "then_port", "else_port", "coverage"):
    if re.search(rf"\b{field}\s*:", row) is None:
        raise SystemExit(f"If control row missing co-sealed field: {field}")
function = struct_body("VerifiedResolvedFunctionIfControlV1")
for field in ("owner", "rows", "coverage_partition"):
    if re.search(rf"\b{field}\s*:", function) is None:
        raise SystemExit(f"function If product missing sealed field: {field}")
if re.search(r"fn\s+into_parts\b", control):
    raise SystemExit("If control product must not separate topology from coverage")
if "pub(crate) use if_control" in module or "pub use if_control" in module:
    raise SystemExit("disconnected If control product must not be re-exported")
PY

  for spec in \
    'cleanup.rs:ResolvedCleanupObligationsV1' \
    'cleanup.rs:explicit_empty' \
    'function_control.rs:VerifiedFunctionCompletionV1' \
    'function_control.rs:VerifiedTerminalReturnV1' \
    'function_control.rs:VerifiedImplicitVoidCompletionV1' \
    'function_control.rs:unreachable_suffix_count' \
    'function_control.rs:SourceBodySiteV1' \
    'function_control.rs:ResolvedControlTransferV1::Return' \
    'function_control.rs:SealedFunctionExitContractV1' \
    'function_control.rs:DeclaredFunctionResultContractV1' \
    'function_control.rs:SealedFunctionExitDispositionV1' \
    'function_control.rs:FunctionExitCoverageV1' \
    'function_control.rs:ReturnExitRelationV1' \
    'function_control.rs:function_exit_contract' \
    'function_control.rs:ReturnClassificationInvariant' \
    'README.md:SSA-E0 function completion'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$flow/$file" \
      "D′ SSA-E0 completion contract drifted: $file:$anchor"
  done
  for spec in \
    'completion_consumption.rs:ReadyFunctionCompletionV1' \
    'completion_consumption.rs:claim_explicit_return' \
    'completion_consumption.rs:implicit_body_end()' \
    'completion_consumption.rs:implicit_body_mismatch' \
    'completion_consumption.rs:legacy_return_state_active' \
    'completion_consumption.rs:finalize_ready_function_completion' \
    'lowerer.rs:canonical_completion/body_length_overflow' \
    'mod.rs:finalize_ready_function_completion(builder, ready)'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$lowering/$file" \
      "D′ SSA-E0 completion consumption drifted: $file:$anchor"
  done

  # F1 DRAFT-SEAL0-S0 is still disconnected from the legacy finalizer.  This
  # guard fixes the new owner vocabulary and its non-mutating projection now;
  # direct Return-writer retirement is a later integration row.
  for spec in \
    'draft_seal.rs:ReadyFunctionDraftSealV1' \
    'draft_seal.rs:OpenFunctionDraftSealV1' \
    'draft_seal.rs:PreparedFunctionDraftSealV1' \
    'draft_seal.rs:PreparedFunctionDraftSealPlanV1' \
    'draft_seal.rs:CompletedFunctionDraftV1' \
    'draft_seal.rs:RejectedFunctionDraftSealV1' \
    'draft_seal.rs:FunctionDraftSealProjectionV1' \
    'draft_seal.rs:PreparedFunctionPhiSealV1' \
    'draft_seal.rs:PreparedFunctionPhiClosureReceiptV1' \
    'draft_seal.rs:prepare_phi_closure' \
    'draft_seal.rs:PhiClosureFailed' \
    'draft_seal.rs:prepare_type_facts' \
    'draft_seal.rs:prepare_metadata' \
    'draft_seal.rs:PreparedFunctionSignatureV1' \
    'draft_seal.rs:PreparedFunctionResultV1' \
    'draft_seal.rs:prepare_signature' \
    'draft_seal.rs:prepare_exit_borrowed' \
    'draft_seal.rs:MetadataContractFailed' \
    'draft_seal.rs:prepare_stale_facts' \
    'draft_seal.rs:TypedValueVerificationFailed' \
    'draft_seal.rs:ProjectedVerificationFailed'; do
    local file="${spec%%:*}"
    local anchor="${spec#*:}"
    guard_expect_fixed_in_file "$tag" "$anchor" "$lowering/$file" \
      "F1 DRAFT-SEAL0 vocabulary drifted: $file:$anchor"
  done
  local draft_products
  draft_products="$(rg -n 'struct (ReadyFunctionDraftSealV1|PreparedFunctionDraftSealV1|CompletedFunctionDraftV1|RejectedFunctionDraftSealV1)' "$draft_seal" | wc -l | tr -d '[:space:]')"
  if [[ "$draft_products" != "4" ]]; then
    guard_fail "$tag" "F1 DRAFT-SEAL0 owner vocabulary must have four products, found $draft_products"
  fi
  if rg -n 'current_module\.take\(' "$draft_seal"; then
    guard_fail "$tag" "F1 DRAFT-SEAL0 must not extract current_module during projection"
  fi
  for spec in \
    'prepare_draft_seal_close' \
    'open_resolved_function_draft_seal_session_v1' \
    'builder_view' \
    'PreparedFunctionSessionCloseV1' \
    'PreparedFunctionSessionCommitInputV1' \
    'commit_projected' \
    'discard_unpublished' \
    'RejectedFunctionSessionCloseV1'; do
    guard_expect_fixed_in_file "$tag" "$spec" "$session_terminal" \
      "F1 DRAFT-SEAL0 session-close seam drifted: $spec"
  done
  local session_prepare_count session_commit_count
  session_prepare_count="$(rg -n 'fn prepare_draft_seal_close\(' "$session_terminal" | wc -l | tr -d '[:space:]')"
  session_commit_count="$(rg -n 'fn commit\((mut )?self\) -> MirFunction' "$session_terminal" | wc -l | tr -d '[:space:]')"
  if [[ "$session_prepare_count" != "1" || "$session_commit_count" != "1" ]]; then
    guard_fail "$tag" "F1 DRAFT-SEAL0 session close must have one prepare and one infallible commit"
  fi
  if rg -n 'ValueId|BasicBlockId|MirBuilder|BindingRefV1|may_rebind|carrier|\bPHI\b' \
    "$completion" "$cleanup"; then
    guard_fail "$tag" "D′ SSA-E0 pre-Builder completion crossed the materialization/effect boundary"
  fi
  if rg -n 'BodyReturnPolicyV1|RootFinalOnly|allow_return' \
    "$root/src/mir/resolved_region_flow/analyzer.rs"; then
    guard_fail "$tag" "D′ SSA-E0 RegionFlow retained a duplicate Return policy"
  fi
  if rg -n 'emit_return_from_value' "$lowering/lowerer.rs"; then
    guard_fail "$tag" "D′ SSA-E0 canonical Return reached the legacy defer-capable emitter"
  fi
  if rg -n 'returns_value:\s*bool' "$compiler/capability.rs"; then
    guard_fail "$tag" "D′ SSA-E0 raw returns_value plan authority returned"
  fi
  if rg -n 'fn verify_function_completion_v1\(' "$flow" \
    --glob '!function_control.rs' --glob '!function_control_tests.rs'; then
    guard_fail "$tag" "F1 function-exit semantic seal gained a second completion producer"
  fi
  if rg -n 'CallableHeader|callable_header|catalog|ASTNode::FunctionDeclaration' "$completion"; then
    guard_fail "$tag" "F1 completion must use the owner-closed source view, not a second header/catalog walk"
  fi

  local file lines
  for file in \
    "$compiler/located.rs" \
    "$compiler/source_projection.rs" \
    "$compiler/source_view.rs" \
    "$compiler/source_view_tests.rs" \
    "$coverage" \
    "$flow/source_coverage_tests.rs" \
    "$cleanup" \
    "$completion" \
    "$flow/function_control_tests.rs" \
    "$if_control" \
    "$if_control_tests" \
    "$lowering/completion_consumption.rs" \
    "$lowering/completion_tests.rs" \
    "$draft_seal" \
    "$session_terminal" \
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
  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_control_flow::function_control_tests
  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_control_flow::if_control_tests
  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::builder::resolved_lowering::completion_tests

  echo "resolved_control_flow_s2prime_range=owner-body-start-nonzero-count"
  echo "resolved_control_flow_s2prime_site_identity=located-carriers-only"
  echo "resolved_control_flow_s2prime_verified_clone=0"
  echo "resolved_control_flow_s2prime_effect_rows=0"
  echo "resolved_control_flow_s2prime_production_consumers=0"
  echo "resolved_control_flow_s2prime_runtime_activation=0"
  echo "resolved_control_flow_ssa_s3_if_rows=exact-source-preorder"
  echo "resolved_control_flow_ssa_s3_nested_coverage=exclusive-partition"
  echo "resolved_control_flow_ssa_s3_internal_coverage_consumers=1"
  echo "resolved_control_flow_ssa_s3_production_analyzer_callers=0"
  echo "resolved_control_flow_ssa_s3_binding_effect_rows=0"
  echo "resolved_control_flow_ssa_s3_runtime_activation=0"
  echo "resolved_control_flow_ssa_e0_exact_completion=explicit-or-implicit"
  echo "resolved_control_flow_ssa_e0_cleanup=explicit-empty-only"
  echo "resolved_control_flow_ssa_e0_grammar_delta=0"
  echo "function_exit_f1_semantic_seal_producer=verify_function_completion_v1"
  echo "function_exit_f1_return_carrier_owner=existing-mir-return-exit-contract"
  echo "function_exit_f1_builder_materialization=0"
  echo "function_exit_f1_draft_seal_owner_vocabulary=4"
  echo "function_exit_f1_draft_seal_projection_live_mutation=0"
  echo "function_exit_f1_draft_seal_signature_plan=1"
  echo "function_exit_f1_draft_seal_legacy_writer_retirement=deferred"
  echo "function_exit_f1_draft_seal_session_close_prepare=1"
  echo "function_exit_f1_draft_seal_session_close_commit=1"
  echo "function_exit_f1_parser_activation=0"
}
