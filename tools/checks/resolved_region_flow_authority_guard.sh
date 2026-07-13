#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="resolved-region-flow-authority"
MODULE="$ROOT/src/mir/resolved_semantics"
MIR_MOD="$ROOT/src/mir/mod.rs"
LOWER_STATE="$ROOT/src/mir/builder/vars/resolved_binding_state.rs"
LOWER_LOCAL="$ROOT/src/mir/builder/vars/lexical_scope.rs"
LOWER_PARAM="$ROOT/src/mir/builder/calls/parameter_setup.rs"
LOWERING_INPUT="$ROOT/src/mir/compiler/lowering_input.rs"
RESOLVED_LOWER="$ROOT/src/mir/builder/resolved_lowering"
BLOCKEXPR_INVENTORY="$ROOT/tools/checks/fixtures/blockexpr_producer_inventory_v1.json"
source "$ROOT/tools/checks/lib/guard_common.sh"
source "$ROOT/tools/checks/lib/resolved_blockexpr_lowering_contract.sh"
source "$ROOT/tools/checks/lib/resolved_if_lowering_contract.sh"
guard_require_command "$TAG" cargo
guard_require_command "$TAG" find
guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_command "$TAG" sort
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$MODULE/README.md" \
  "$MODULE/mod.rs" \
  "$MODULE/ids.rs" \
  "$MODULE/function_root.rs" \
  "$MODULE/function_root_tests.rs" \
  "$MODULE/function_view.rs" \
  "$MODULE/source_site.rs" \
  "$MODULE/records.rs" \
  "$MODULE/resolver.rs" \
  "$MODULE/resolver_tests.rs" \
  "$MODULE/normalized.rs" \
  "$MODULE/owner_forest.rs" \
  "$MODULE/owner_forest_tests.rs" \
  "$MODULE/owner_resolver.rs" \
  "$MODULE/product.rs" \
  "$MODULE/verifier.rs" \
  "$LOWER_STATE" \
  "$LOWER_LOCAL" \
  "$LOWER_PARAM" \
  "$ROOT/src/mir/compiler/README.md" "$LOWERING_INPUT" \
  "$ROOT/src/mir/compiler/located.rs" "$ROOT/src/mir/compiler/source_projection.rs" "$ROOT/src/mir/compiler/source_view.rs" "$ROOT/src/mir/compiler/source_view_tests.rs" "$ROOT/src/mir/builder/calls/context_lifecycle.rs" "$ROOT/src/mir/builder/calls/function_session.rs" "$ROOT/src/mir/builder/calls/function_session_tests.rs" "$ROOT/src/mir/builder/calls/lowering.rs" "$ROOT/src/mir/builder/calls/skeleton_builder.rs" \
  "$ROOT/src/mir/compiler/capability.rs" "$ROOT/src/mir/compiler/capability_tests.rs" "$ROOT/src/mir/compiler/function_input.rs" "$ROOT/src/mir/compiler/module_session.rs" "$RESOLVED_LOWER/README.md" "$RESOLVED_LOWER/mod.rs" "$RESOLVED_LOWER/identity.rs" "$RESOLVED_LOWER/lowerer.rs" "$RESOLVED_LOWER/tests.rs" \
  "$BLOCKEXPR_INVENTORY" \
  "$MODULE/tests.rs" \
  "$MODULE/shadow/mod.rs" \
  "$MODULE/shadow/owner_boundary.rs" \
  "$MODULE/shadow/assignment_traversal_tests.rs" \
  "$MODULE/shadow/block_expr.rs" \
  "$MODULE/shadow/ids.rs" \
  "$MODULE/shadow/path.rs" \
  "$MODULE/shadow/product.rs" \
  "$MODULE/shadow/resolver.rs" \
  "$MODULE/shadow/scope_container_tests.rs" \
  "$MODULE/shadow/expr.rs" \
  "$MODULE/shadow/leaf_traversal_tests.rs" \
  "$MODULE/shadow/stmt.rs" \
  "$MODULE/shadow/vocabulary.rs" \
  "$MODULE/shadow/vocabulary_tests.rs" \
  "$MODULE/shadow/tests.rs" \
  "$MIR_MOD" \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"

expected_manifest="$(printf '%s\n' \
  block_expr_tests.rs \
  function_root.rs function_root_tests.rs \
  function_view.rs \
  ids.rs \
  if_region.rs if_region_tests.rs \
  mod.rs \
  normalized.rs \
  owner_forest.rs \
  owner_forest_tests.rs \
  owner_resolver.rs \
  product.rs \
  records.rs \
  resolver.rs \
  resolver_tests.rs \
  shadow/assignment_traversal_tests.rs \
  shadow/block_expr.rs \
  shadow/expr.rs \
  shadow/ids.rs \
  shadow/leaf_traversal_tests.rs \
  shadow/mod.rs \
  shadow/owner_boundary.rs \
  shadow/path.rs \
  shadow/product.rs \
  shadow/resolver.rs \
  shadow/scope_container_tests.rs \
  shadow/stmt.rs \
  shadow/tests.rs \
  shadow/vocabulary.rs \
  shadow/vocabulary_tests.rs \
  source_site.rs \
  tests.rs \
  verifier.rs)"
actual_manifest="$(find "$MODULE" -type f -name '*.rs' -printf '%P\n' | LC_ALL=C sort)"
if [[ "$actual_manifest" != "$expected_manifest" ]]; then
  printf '%s\n' "$actual_manifest" >&2
  guard_fail "$TAG" "SA0 Rust source manifest drifted; classify every new file explicitly"
fi

mapfile -t PRODUCTION_FILES < <(
  find "$MODULE" -type f -name '*.rs' ! -name 'tests.rs' -print | LC_ALL=C sort
)
mapfile -t CANONICAL_FILES < <(
  find "$MODULE" -maxdepth 1 -type f -name '*.rs' ! -name 'tests.rs' -print | LC_ALL=C sort
)
mapfile -t CANONICAL_NON_RESOLVER_FILES < <(
  find "$MODULE" -maxdepth 1 -type f -name '*.rs' ! -name '*_tests.rs' ! -name 'tests.rs' \
    ! -name 'resolver.rs' ! -name 'owner_resolver.rs' -print | LC_ALL=C sort
)
mapfile -t CANONICAL_ARENA_FILES < <(
  find "$MODULE" -maxdepth 1 -type f -name '*.rs' ! -name '*_tests.rs' ! -name 'tests.rs' \
    ! -name 'function_view.rs' -print | LC_ALL=C sort
)
mapfile -t SHADOW_FILES < <(
  find "$MODULE/shadow" -type f -name '*.rs' ! -name 'tests.rs' -print | LC_ALL=C sort
)

python3 - "$ROOT" "$BLOCKEXPR_INVENTORY" <<'PY'
from collections import Counter
import json
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
inventory_path = Path(sys.argv[2])
data = json.loads(inventory_path.read_text())

if set(data) != {"schema", "decision", "rows", "selection"}:
    raise SystemExit("BlockExpr producer inventory top-level schema drifted")
if data["schema"] != "BlockExprProducerInventoryV1":
    raise SystemExit("BlockExpr producer inventory schema name drifted")
if data["decision"] != "lexical_blockexpr":
    raise SystemExit("BlockExpr decision must remain lexical_blockexpr")

row_fields = {
    "producer_id", "producer_path", "producer_status", "output_family",
    "consumer_entry", "classification", "required_scope_semantics",
    "live_caller_evidence", "retirement_owner",
}
statuses = {"Active", "Planned", "TestOnly", "Dead"}
classifications = {
    "CanonicalRustSource", "CanonicalHakoTypedSourcePlanned",
    "CompilerGeneratedCanonical", "LegacyProgramV0Compatibility",
    "InternalSequenceRequired", "TestOnly", "RejectedOrUnknown",
}
rows = data["rows"]
ids = [row.get("producer_id") for row in rows]
if len(ids) != len(set(ids)):
    raise SystemExit("BlockExpr producer inventory contains duplicate producer_id")

for row in rows:
    producer_id = row["producer_id"]
    if set(row) != row_fields:
        raise SystemExit(f"{producer_id}: row schema drifted")
    if row["producer_status"] not in statuses:
        raise SystemExit(f"{producer_id}: unknown producer_status")
    if row["classification"] not in classifications:
        raise SystemExit(f"{producer_id}: unknown classification")
    producer_path = root / row["producer_path"]
    if not producer_path.is_file():
        raise SystemExit(f"{producer_id}: producer_path is missing: {producer_path}")
    for field in row_fields - {"live_caller_evidence"}:
        value = row[field]
        if not isinstance(value, str) or not value:
            raise SystemExit(f"{producer_id}: {field} must be a non-empty string")
    evidence = row["live_caller_evidence"]
    if not isinstance(evidence, list) or not all(isinstance(item, str) for item in evidence):
        raise SystemExit(f"{producer_id}: live_caller_evidence must be a string array")
    if row["producer_status"] in {"Active", "Planned", "TestOnly"} and not evidence:
        raise SystemExit(f"{producer_id}: live/planned/test producer lacks evidence")
    for item in evidence:
        if "#" not in item:
            raise SystemExit(f"{producer_id}: evidence must use path#literal format: {item}")
        rel, literal = item.split("#", 1)
        evidence_path = root / rel
        if not evidence_path.is_file() or literal not in evidence_path.read_text():
            raise SystemExit(f"{producer_id}: stale evidence: {item}")

counts = Counter(row["classification"] for row in rows)
unknown_production = sum(
    row["producer_status"] == "Active" and row["classification"] == "RejectedOrUnknown"
    for row in rows
)
selection = data["selection"]
expected_selection = {
    "internal_sequence_required_count": counts["InternalSequenceRequired"],
    "source_parser_compat_sequence_count": 0,
    "unknown_production_producer_count": unknown_production,
    "program_v0_schema_delta": 0,
    "program_v0_source_kind_recovery": 0,
    "b0_c_disposition": "skipped_by_zero_callers",
    "selected_next_slice": "B0-S",
}
if selection != expected_selection:
    raise SystemExit(
        f"BlockExpr mechanical selection drifted: expected={expected_selection} actual={selection}"
    )
if counts["InternalSequenceRequired"] != 0:
    raise SystemExit("B0-C is forbidden unless a live InternalSequenceRequired producer exists")

# Conservative syntax inventory: count every Rust ASTNode::BlockExpr literal whose
# top-level fields contain a colon. Current destructuring sites use shorthand fields,
# so a new constructor or renamed-field pattern intentionally forces reclassification.
def blockexpr_literal_counts():
    result = Counter()
    needle = "ASTNode::BlockExpr {"
    for base in (root / "src", root / "crates"):
        for path in base.rglob("*.rs"):
            text = path.read_text(errors="ignore")
            start = 0
            while True:
                pos = text.find(needle, start)
                if pos < 0:
                    break
                opening = text.find("{", pos)
                depth = 0
                has_top_level_colon = False
                in_string = False
                escaped = False
                index = opening
                while index < len(text):
                    char = text[index]
                    if in_string:
                        if escaped:
                            escaped = False
                        elif char == "\\":
                            escaped = True
                        elif char == '"':
                            in_string = False
                    elif char == '"':
                        in_string = True
                    elif char == "{":
                        depth += 1
                    elif char == "}":
                        depth -= 1
                        if depth == 0:
                            break
                    elif char == ":" and depth == 1:
                        has_top_level_colon = True
                    index += 1
                if has_top_level_colon:
                    result[str(path.relative_to(root))] += 1
                start = pos + len(needle)
    return result

actual_literal_counts = blockexpr_literal_counts()
classified_literal_paths = {
    row["producer_path"] for row in rows if "ASTNode::BlockExpr" in row["output_family"]
}
if set(actual_literal_counts) != classified_literal_paths:
    raise SystemExit(
        "ASTNode::BlockExpr producer-path inventory drifted: "
        f"unclassified={sorted(set(actual_literal_counts) - classified_literal_paths)} "
        f"stale={sorted(classified_literal_paths - set(actual_literal_counts))}"
    )

program_v0_literal_paths = set()
program_v0_literal = re.compile(r'(?:\\?\")type(?:\\?\")\s*:\s*(?:\\?\")BlockExpr')
for base in (root / "src", root / "lang" / "src"):
    for suffix in ("*.rs", "*.hako"):
        for path in base.rglob(suffix):
            if program_v0_literal.search(path.read_text(errors="ignore")):
                program_v0_literal_paths.add(str(path.relative_to(root)))
classified_program_v0_literal_paths = {
    row["producer_path"] for row in rows if "ProgramV0" in row["output_family"]
}
if program_v0_literal_paths != classified_program_v0_literal_paths:
    raise SystemExit(
        "ProgramV0 BlockExpr literal producer inventory drifted: "
        f"unclassified={sorted(program_v0_literal_paths - classified_program_v0_literal_paths)} "
        f"stale={sorted(classified_program_v0_literal_paths - program_v0_literal_paths)}"
    )

program_v0_ast = (root / "src/runner/json_v0_bridge/ast.rs").read_text()
if program_v0_ast.count("BlockExpr {") != 1 or "CompatSequence" in program_v0_ast:
    raise SystemExit("ProgramV0 BlockExpr schema drifted or gained CompatSequence")
ingress_rows = [
    row for row in rows
    if row["producer_path"] == "src/runner/json_v0_bridge/ast.rs"
    and row["classification"] == "LegacyProgramV0Compatibility"
]
if len(ingress_rows) != 1:
    raise SystemExit("ProgramV0 BlockExpr ingress must be classified exactly once")
for rel in (
    "src/parser/expr/primary/block.rs",
    "src/parser/expr/ternary.rs",
    "src/parser/expr/match_expr_impl.rs",
    "lang/src/compiler/parser/expr/parser_literal_box.hako",
):
    if "CompatSequence" in (root / rel).read_text():
        raise SystemExit(f"source parser must not produce CompatSequence: {rel}")
PY

guard_expect_fixed_in_file "$TAG" "pub(crate) mod resolved_semantics" "$MIR_MOD" \
  "SA0 module must remain crate-private"
if [[ "$(rg -c 'resolved_semantics' "$MIR_MOD")" != "1" ]]; then
  guard_fail "$TAG" "MIR root may contain only the crate-private module declaration"
fi

for required in \
  "pub struct VerifiedResolvedFunctionV1" \
  "pub(crate) struct ResolvedFunctionDraftV1" \
  "pub(crate) struct ResolvedFunctionDataV1" \
  "pub(crate) owner: FunctionOwnerIdV1" \
  "pub(crate) function_scope: ScopeId" \
  "pub(crate) function_region: RegionId" \
  "pub(crate) bindings: BTreeMap<BindingId, ResolvedBindingRecordV1>" \
  "pub(crate) scopes: BTreeMap<ScopeId, ResolvedScopeRecordV1>" \
  "pub(crate) regions: BTreeMap<RegionId, ResolvedRegionRecordV1>" \
  "pub(crate) declarations: BTreeMap<SourceBindingSiteV1, BindingRefV1>" \
  "pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>" \
  "pub(crate) assignment_targets:" \
  "pub(crate) resolved_exits: BTreeMap<ResolvedExitSiteV1, ResolvedExitRecordV1>"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/product.rs" \
    "sealed semantic product schema drifted: $required"
done
for required in 'ScopeKindV1 {' BlockExpr 'RegionKindV1 {'; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/records.rs" \
    "B0-S passive BlockExpr kind vocabulary drifted: $required"
done
for required in verify_blockexpr_scope_region_contract BlockExprScopeContractMismatch \
  BlockExprRegionContractMismatch 'RegionKindV1::BlockExpr =>'; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/verifier.rs" \
    "B0-S BlockExpr seal/containment contract drifted: $required"
done

for required in \
  "static NEXT_COMPILATION_BRAND: AtomicU64" \
  "fetch_update(Ordering::Relaxed, Ordering::Relaxed" \
  "compilation: u64"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/ids.rs" \
    "function-owner uniqueness contract drifted: $required"
done

for required in \
  "pub struct FunctionOwnerIdV1" \
  "pub(crate) struct FunctionOwnerIssuerV1" \
  "pub struct BindingRefV1" \
  "pub struct UpvarRefV1" \
  "pub struct ScopeId" \
  "pub struct RegionId"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/ids.rs" \
    "owner-scoped identity schema drifted: $required"
done

for required in \
  "pub struct OwnedExprSiteV1" \
  "owner: FunctionOwnerIdV1" \
  "site: SourceExprSiteV1"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/source_site.rs" \
    "P0 owner-branded expression provenance drifted: $required"
done
for role in \
  LambdaBodyRoot \
  'LambdaBody(u32)' \
  QMarkOperand \
  MatchScrutinee \
  'MatchArm(u32)' \
  MatchElse \
  EnumMatchScrutinee \
  'EnumMatchArm(u32)' \
  EnumMatchElse \
  BlockExprPreludeRoot \
  'BlockExprPrelude(u32)' \
  BlockExprTail \
  TryBodyRoot \
  'TryBody(u32)' \
  'CatchClause(u32)' \
  CatchBodyRoot \
  'CatchBody(u32)' \
  CleanupBodyRoot \
  'CleanupBody(u32)'; do
  guard_expect_fixed_in_file "$TAG" "$role" "$MODULE/source_site.rs" \
    "P0 source-role vocabulary drifted: $role"
done
if rg -n '\b(CaptureId|CaptureSlotId)\b' \
  "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "UP0 structural Upvar must not create capture or runtime-slot identities"
fi

for required in \
  "pub struct VerifiedSemanticOwnerForestV1" \
  "owners: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>" \
  "parents: BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>" \
  "root: FunctionOwnerIdV1" \
  "child_at: BTreeMap<OwnedExprSiteV1, FunctionOwnerIdV1>" \
  "upvar_observations: Box<[UpvarObservationV1]>" \
  "upvars: Box<[UpvarRefV1]>" \
  "normalized: NormalizedSemanticOwnerForestGraphV1" \
  "pub struct NormalizedOwnerKeyV1" \
  "pub struct UpvarObservationV1" \
  "pub enum UpvarAccessKindV1" \
  "pub struct NormalizedUpvarObservationV1" \
  "pub struct NormalizedUpvarEdgeV1" \
  "pub struct NormalizedSemanticOwnerForestGraphV1" \
  "fn derive_and_verify_upvars(" \
  "pub fn upvar_observations(&self)" \
  "fn verify_nearest_visible_source(" \
  "pub(crate) fn insert_parent(" \
  "pub(crate) fn seal("; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/owner_forest.rs" \
    "OF0 sealed owner-forest authority drifted: $required"
done
for required in \
  "resolve_owner_shadow_view_v0" \
  "visible_bindings_for_child" \
  "lambda.syntax_view()" \
  "seal_owner_with_ancestors"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/owner_resolver.rs" \
    "OF0 recursive owner resolution drifted: $required"
done
if rg -n 'Arc<VerifiedResolvedFunctionV1>|ValueId|BasicBlockId|MirBuilder|CoreContext|Planner|Lower|Recipe' \
  "$MODULE/owner_forest.rs" "$MODULE/owner_resolver.rs"; then
  guard_fail "$TAG" "OF0 forest must directly own sealed owners and remain disconnected"
fi

for required in \
  "pub enum ResolvedExitSiteV1" \
  "Statement(SourceStmtSiteV1)" \
  "Expression(SourceExprSiteV1)"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/source_site.rs" \
    "E0 typed exit source vocabulary drifted: $required"
done
for required in \
  "pub enum ResolvedExitOriginV1" \
  "ExplicitContinue" \
  "ExplicitBreak" \
  "ExplicitReturn" \
  "pub enum ResolvedControlTransferV1" \
  "pub struct ResolvedExitRecordV1" \
  "source_region: RegionId" \
  "origin: ResolvedExitOriginV1" \
  "transfer: ResolvedControlTransferV1"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/records.rs" \
    "E0 atomic exit record drifted: $required"
done
for required in \
  "pub enum ResolvedLexicalRefV1" \
  "Upvar(UpvarRefV1)" \
  "pub enum ResolvedAssignmentTargetV1" \
  "UpvarRebind(UpvarRefV1)"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/records.rs" \
    "UP1 structural Upvar read/rebind vocabulary drifted: $required"
done
if rg -n 'ResolvedControlExitV1|control_exit_regions' "$MODULE"; then
  guard_fail "$TAG" "E0 sealed product must not retain the parallel exit-map schema"
fi

for required in \
  "pub(crate) fn seal(" \
  "verify_resolved_function(&self.data)?" \
  "build_normalized_graph(&self.data)" \
  "pub fn normalized_graph(&self)"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/product.rs" \
    "verified seal/publication boundary drifted: $required"
done

guard_expect_fixed_in_file "$TAG" "pub enum ResolvedFunctionVerificationErrorV1" \
  "$MODULE/verifier.rs" "resolved-function verifier outcome vocabulary missing"
guard_expect_fixed_in_file "$TAG" "pub struct NormalizedResolvedFunctionGraphV1" \
  "$MODULE/normalized.rs" "normalized semantic graph missing"

if rg -n 'ASTNode|Box[[:space:]]*<[[:space:]]*AST|Vec[[:space:]]*<[[:space:]]*AST' \
  "${CANONICAL_ARENA_FILES[@]}"; then
  guard_fail "$TAG" "canonical resolved semantic arena must not own cloned AST payloads"
fi
for required in \
  "params: &'a [String]" \
  "body: &'a [ASTNode]" \
  "pub(crate) fn from_ast(function: &'a ASTNode)" \
  "pub(crate) fn from_lambda_ast(lambda: &'a ASTNode)"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/function_view.rs" \
    "canonical function syntax view must remain borrowed and AST-derived: $required"
done
if rg -n 'fn new\(|Vec[[:space:]]*<[[:space:]]*AST|Box[[:space:]]*<[[:space:]]*AST|Arc[[:space:]]*<[[:space:]]*AST|Rc[[:space:]]*<[[:space:]]*AST' \
  "$MODULE/function_view.rs"; then
  guard_fail "$TAG" "canonical function syntax view must not be forgeable or own AST payloads"
fi

for required in \
  "pub(crate) struct ShadowBindingOrdinalV0" \
  "pub(crate) struct ShadowScopeIdV0" \
  "pub(crate) struct ShadowRegionIdV0"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/ids.rs" \
    "shadow-only identity schema drifted: $required"
done
for required in \
  "pub(crate) struct ShadowResolvedFunctionV0" \
  "BTreeMap<ShadowBindingOrdinalV0, ShadowBindingRecordV0>" \
  "BTreeMap<SourceExprSiteV1, ShadowLexicalRefV0>" \
  "AncestorRebind(Box<str>)" \
  "pub(crate) struct ShadowExitRecordV0" \
  "pub(crate) source_region: ShadowRegionIdV0" \
  "pub(crate) origin: ShadowExitOriginV0" \
  "pub(crate) transfer: ShadowControlExitV0" \
  "DuplicateExitSite" \
  "BlockExprNonLocalExit" \
  "BTreeMap<SourceStmtSiteV1, ShadowExitRecordV0>"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/product.rs" \
    "shadow product schema drifted: $required"
done
for required in \
  "pub site: ResolvedExitSiteV1" \
  "pub source_region: NormalizedRegionKeyV1" \
  "pub origin: ResolvedExitOriginV1" \
  "pub transfer: NormalizedControlTransferV1"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/normalized.rs" \
    "E0 normalized exit record drifted: $required"
done
for required in \
  "UnsupportedExitSiteKind(ResolvedExitSiteV1)" \
  "ResolvedExitOriginV1::ExplicitContinue" \
  "ResolvedExitOriginV1::ExplicitBreak" \
  "ResolvedExitOriginV1::ExplicitReturn" \
  "source_region_contains_site_v1"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/verifier.rs" \
    "E0 statement-only exit verifier drifted: $required"
done
if rg -n 'QMark|Throw' "$MODULE/records.rs"; then
  guard_fail "$TAG" "E0 must not activate QMark/Throw exit vocabulary"
fi
guard_expect_fixed_in_file "$TAG" "pub(super) fn resolve_function_shadow_v0" \
  "$MODULE/shadow/resolver.rs" "shadow resolver entry must remain explicit"

if rg -n '\b(BindingId|ScopeId|RegionId|BindingRefV1|ResolvedFunctionDraftV1|ResolvedFunctionDataV1|VerifiedResolvedFunctionV1)\b|hakorune_mir_core|super::super::(ids|product|records|\*)|resolved_semantics::\*|::\*' \
  "${SHADOW_FILES[@]}"; then
  guard_fail "$TAG" "SA1 shadow resolver crossed into canonical identity/product authority"
fi
if rg -n 'ASTNode|Box[[:space:]]*<[[:space:]]*AST|Vec[[:space:]]*<[[:space:]]*AST' \
  "$MODULE/shadow/ids.rs" "$MODULE/shadow/path.rs" "$MODULE/shadow/product.rs"; then
  guard_fail "$TAG" "shadow IDs/product must not retain canonical AST payloads"
fi
guard_expect_fixed_in_file "$TAG" "fn classify_shadow_ast_disposition_v0(node: &ASTNode)" \
  "$MODULE/shadow/vocabulary.rs" "exhaustive AST disposition classifier missing"
if rg -n '(^|[[:space:]])_[[:space:]]*=>' "$MODULE/shadow/vocabulary.rs"; then
  guard_fail "$TAG" "AST disposition classifier must remain exhaustive without wildcard"
fi
if rg -n 'ValueId|BasicBlockId|MirBuilder|CoreContext|control_flow::plan|lowerer|Recipe|join_ir::ownership' \
  "${SHADOW_FILES[@]}"; then
  guard_fail "$TAG" "SA1 shadow resolver crossed a planner/lower/materialization boundary"
fi
if rg -n 'Shadow[A-Za-z0-9_]*V0' "$MIR_MOD" "$MODULE/mod.rs"; then
  guard_fail "$TAG" "root modules must not re-export or alias disconnected shadow products"
fi
if ! rg -q '^mod shadow;$' "$MODULE/mod.rs"; then
  guard_fail "$TAG" "shadow module must remain private to resolved_semantics"
fi
if rg -n 'pub(\([^)]*\))?[[:space:]]+(use|mod).*shadow|pub[[:space:]]+use.*ShadowResolvedFunctionV0' \
  "$MODULE/mod.rs"; then
  guard_fail "$TAG" "construction-local resolver drafts must not escape resolved_semantics"
fi

for required in \
  "SHADOW_ACCEPTED_STATEMENTS_V0" \
  "SHADOW_ACCEPTED_EXPRESSIONS_V0" \
  "SHADOW_ACCEPTED_ASSIGNMENT_TARGETS_V0"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/vocabulary.rs" \
    "shadow accepted vocabulary manifest drifted: $required"
done
for variant in Local Outbox Nowait Assignment CompoundAssignment ScopeBox TaskScope FastMemRegion If Loop Break Continue Return Print; do
  guard_expect_fixed_in_file "$TAG" "ASTNode::$variant" "$MODULE/shadow/stmt.rs" \
    "accepted statement lost its explicit resolver arm: $variant"
done
for variant in Literal Variable Me UnaryOp BinaryOp MethodCall FieldAccess Index FunctionCall New \
  AwaitExpression ArrayLiteral MapLiteral RecordLiteral RecordUpdate CheckExpr FromCall Call \
  GroupedAssignmentExpr BlockExpr; do
  guard_expect_fixed_in_file "$TAG" "ASTNode::$variant" "$MODULE/shadow/expr.rs" \
    "accepted expression lost its explicit resolver arm: $variant"
done
guard_expect_fixed_in_file "$TAG" "lambda @ ASTNode::Lambda" "$MODULE/shadow/expr.rs" \
  "OF0 Lambda inventory arm missing"
for required in resolve_block_expr BlockExprPreludeRoot BlockExprNonLocalExit; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/block_expr.rs" "B0-F BlockExpr traversal contract drifted: $required"
done
guard_expect_fixed_in_file "$TAG" 'SourcePathSegmentV1::BlockExprPreludeRoot, SourcePathSegmentV1::BlockExprTail' "$MODULE/owner_forest.rs" "B0-F BlockExpr tail owner order drifted"

python3 - "$MODULE/shadow/expr.rs" "$MODULE/shadow/stmt.rs" <<'PY'
from pathlib import Path
import re
import sys

expr = set(re.findall(r"ASTNode::(\w+)", Path(sys.argv[1]).read_text()))
stmt = set(re.findall(r"ASTNode::(\w+)", Path(sys.argv[2]).read_text()))
expected_expr = {
    "Literal", "Variable", "Me", "UnaryOp", "BinaryOp", "MethodCall",
    "FieldAccess", "Index", "FunctionCall", "New", "AwaitExpression",
    "ArrayLiteral", "MapLiteral", "RecordLiteral", "RecordUpdate",
    "CheckExpr", "FromCall", "Call",
    "GroupedAssignmentExpr", "BlockExpr", "Lambda",
}
expected_stmt = {
    "Local", "Outbox", "Nowait", "Assignment", "CompoundAssignment", "ScopeBox",
    "TaskScope", "FastMemRegion", "If", "Loop",
    "Break", "Continue", "Return", "Print",
}
if expr != expected_expr:
    raise SystemExit(
        "shadow expression implementation/vocabulary drift: "
        f"missing={sorted(expected_expr - expr)} extra={sorted(expr - expected_expr)}"
    )
if stmt != expected_stmt:
    raise SystemExit(
        "shadow statement implementation/vocabulary drift: "
        f"missing={sorted(expected_stmt - stmt)} extra={sorted(stmt - expected_stmt)}"
    )
PY

if rg -n 'allocate_binding_id|next_binding|BindingId::new|BindingId[[:space:]]*\(' \
  "${CANONICAL_NON_RESOLVER_FILES[@]}"; then
  guard_fail "$TAG" "canonical resolved-semantics files must not allocate BindingIds"
fi
if [[ "$(rg -n 'BindingId::new' "$MODULE/resolver.rs" | wc -l)" != "1" ]]; then
  guard_fail "$TAG" "canonical resolver must own exactly one construction-site BindingId allocator"
fi
if rg -n 'BindingId[[:space:]]+as|hakorune_mir_core[[:space:]]+as|hakorune_mir_core::\*|type[[:space:]].*BindingId|\.next[[:space:]]*\(' \
  "${CANONICAL_FILES[@]}"; then
  guard_fail "$TAG" "canonical schema must not alias or indirectly advance BindingId"
fi
binding_imports="$(rg '^[[:space:]]*use[[:space:]]+hakorune_mir_core::.*BindingId' \
  "${CANONICAL_FILES[@]}" || true)"
while IFS= read -r import_line; do
  [[ -z "$import_line" ]] && continue
  if [[ "${import_line#*:}" != "use hakorune_mir_core::BindingId;" ]]; then
    guard_fail "$TAG" "canonical BindingId import must use the exact non-aliased spelling: $import_line"
  fi
done <<< "$binding_imports"

if rg -n 'ValueId|BasicBlockId|MirBuilder|CoreContext' "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "SA0 schema must not import MIR materialization owners"
fi

if rg -n -U 'FunctionOwnerIdV1[[:blank:]]*\{[[:space:]]*(compilation|slot)[[:blank:]]*:' \
  $(find "$MODULE" -type f -name '*.rs' ! -path "$MODULE/ids.rs" -print); then
  guard_fail "$TAG" "only the compilation-scoped issuer may construct function owner brands"
fi
if rg -n 'from_unverified_data_for_schema_test' "$MODULE"; then
  guard_fail "$TAG" "unverified semantic products must not bypass seal, including tests"
fi

if rg -n 'join_ir::ownership|mir::region::RegionId|control_flow::plan|lowerer|Recipe' \
  "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "SA0 schema crossed a forbidden ownership/planner/lower boundary"
fi

if rg -n 'pub[[:space:]]+(use|mod).*resolved_semantics|pub[[:space:]]+data:' "$MIR_MOD" "$MODULE"; then
  guard_fail "$TAG" "SA0 must not publicly re-export the module or unverified product data"
fi
if rg -n 'pub[[:space:]]+fn[[:space:]]+(new|from|seal)' "$MODULE/product.rs"; then
  guard_fail "$TAG" "SA0 verified product must have no production public constructor"
fi
if rg -n 'pub use product::.*(Draft|Data)' "$MODULE/mod.rs"; then
  guard_fail "$TAG" "draft/data publication is forbidden"
fi

python3 - "$MODULE/product.rs" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
text = path.read_text()
allowed = {
    "owner",
    "function_origin",
    "function_scope",
    "function_region",
    "binding_ref",
    "binding",
    "bindings",
    "scope", "scopes",
    "region", "regions",
    "declaration_binding",
    "declaration_sites",
    "variable_ref",
    "variable_refs",
    "assignment_target",
    "assignment_targets",
    "resolved_exit", "resolved_exits",
    "binding_count",
    "scope_count",
    "region_count",
    "normalized_graph",
    "exact_scope_containing", "from_verified",
    "block_expr_scope_region_pair", "lowering_roots",
    "seal",
}
methods = set(re.findall(r"pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+(\w+)", text))
unexpected = sorted(methods - allowed)
missing = sorted(allowed - methods)
if unexpected or missing:
    raise SystemExit(
        "verified product public-method allowlist drift: "
        f"unexpected={unexpected} missing={missing}"
    )
PY

consumer_output=""
if consumer_output="$(
  rg -l 'resolved_semantics|VerifiedResolvedFunctionV1' "$ROOT" --glob '*.rs'
)"; then
  :
else
  consumer_rc=$?
  if [[ "$consumer_rc" != "1" ]]; then
    guard_fail "$TAG" "repository-wide semantic-arena consumer scan failed: rc=$consumer_rc"
  fi
fi
while IFS= read -r consumer; do
  [[ -z "$consumer" ]] && continue
  case "$consumer" in
    "$MIR_MOD"|"$MODULE"/*|"$LOWER_STATE"|"$LOWER_LOCAL"|"$LOWER_PARAM"|"$LOWERING_INPUT"|"$ROOT/src/mir/compiler/located.rs"|"$ROOT/src/mir/compiler/source_projection.rs"|"$ROOT/src/mir/compiler/source_view.rs"|"$ROOT/src/mir/compiler/source_view_tests.rs"|"$ROOT/src/mir/compiler/capability.rs"|"$ROOT/src/mir/compiler/capability_tests.rs"|"$ROOT/src/mir/compiler/function_input.rs"|"$ROOT/src/mir/resolved_region_flow"/*|"$RESOLVED_LOWER"/*)
      ;;
    *)
      guard_fail "$TAG" "resolved semantic product escaped its bounded resolver/compiler/lower files: $consumer"
      ;;
  esac
done <<< "$consumer_output"

guard_expect_fixed_in_file "$TAG" "legacy_allocation_forbidden" "$LOWER_STATE" \
  "SA3-B legacy BindingId allocator veto missing"
guard_resolved_blockexpr_lowering_contract "$TAG" "$ROOT"
guard_resolved_if_lowering_contract "$TAG" "$ROOT"

while IFS= read -r file; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached the 800-line stop boundary: $file ($lines)"
  fi
done < <(find "$MODULE" "$ROOT/src/mir/compiler" "$ROOT/src/mir/builder/calls" "$RESOLVED_LOWER" -type f -name '*.rs' -print; printf '%s\n' "$MIR_MOD" "$LOWER_STATE")

if ! rg -q 'pub\(super\) struct BindingId\(u32\)' \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"; then
  guard_fail "$TAG" "private ownership BindingId inventory drifted; reclassify before migration"
fi

python3 "$ROOT/tools/checks/lib/resolved_lowering_ingress_inventory.py" "$ROOT" "$ROOT/tools/checks/fixtures/resolved_lowering_ingress_inventory_v1.json"

cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::block_expr_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::resolver_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::owner_forest_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::assignment_traversal_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::leaf_traversal_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::scope_container_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::vocabulary_tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::builder::vars::resolved_binding_state::tests
for test in mir::compiler::lowering_input::tests mir::compiler::source_view_tests mir::builder::calls::function_session_tests mir::builder::resolved_lowering; do cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib "$test"; done

echo "semantic_arena_schema=present"
echo "semantic_arena_ast_clone_fields=0"
echo "canonical_resolver_binding_allocator_sites=1"
echo "canonical_resolver_production_installs=1-closed-family"
echo "semantic_arena_value_id_imports=0"
echo "semantic_arena_basic_block_id_imports=0"
echo "semantic_arena_bounded_lower_transport=1"
echo "semantic_arena_planner_connection=0"
echo "semantic_arena_active_canonical_lowerer=1"
echo "semantic_arena_legacy_allocator_during_canonical=forbidden"
echo "semantic_arena_parallel_exit_maps=0"
echo "semantic_arena_statement_exit_records=1"
echo "semantic_arena_expression_exit_records=0"
echo "semantic_arena_qmark_throw_acceptance=0"
echo "semantic_owner_forest_noncapturing_lambda=1"
echo "semantic_owner_forest_readonly_upvar=1"
echo "semantic_owner_forest_upvar_write=1"
echo "semantic_owner_forest_capture_mode=0"
echo "semantic_owner_forest_runtime_slot=0"
echo "blockexpr_producer_inventory=closed"
echo "blockexpr_internal_sequence_required=0"
echo "blockexpr_source_parser_compat_sequence=0"
echo "blockexpr_unknown_production_producers=0"
echo "blockexpr_b0_c=skipped_by_zero_callers"
echo "blockexpr_selected_next_slice=B0-S"
echo "blockexpr_scope_kind=present"
echo "blockexpr_region_kind=present"
echo "blockexpr_identity_pair=verified"
echo "blockexpr_exact_origin=verified"
echo "blockexpr_resolver_acceptance=1"
echo "blockexpr_non_local_exit_rejection=1"
echo "blockexpr_lambda_declaration_order=verified"
echo "blockexpr_canonical_straight_line_lower_connection=1"
echo "blockexpr_planner_regionflow_if_loop_lambda_connections=0"
echo "semantic_arena_source_files_under_800=1"
echo "shadow_resolver_canonical_binding_ids=0"
echo "shadow_resolver_external_consumers=0"
echo "shadow_resolver_planner_connection=0"
echo "shadow_resolver_lower_connection=0"
echo "ownership_private_binding_id_imports=0"
echo "summary=ok"
echo "[$TAG] ok"
