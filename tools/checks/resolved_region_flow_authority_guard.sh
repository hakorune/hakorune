#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="resolved-region-flow-authority"
MODULE="$ROOT/src/mir/resolved_semantics"
MIR_MOD="$ROOT/src/mir/mod.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

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
  "$MODULE/source_site.rs" \
  "$MODULE/records.rs" \
  "$MODULE/normalized.rs" \
  "$MODULE/product.rs" \
  "$MODULE/verifier.rs" \
  "$MODULE/tests.rs" \
  "$MODULE/shadow/mod.rs" \
  "$MODULE/shadow/ids.rs" \
  "$MODULE/shadow/path.rs" \
  "$MODULE/shadow/product.rs" \
  "$MODULE/shadow/resolver.rs" \
  "$MODULE/shadow/expr.rs" \
  "$MODULE/shadow/stmt.rs" \
  "$MODULE/shadow/vocabulary.rs" \
  "$MODULE/shadow/tests.rs" \
  "$MIR_MOD" \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"

expected_manifest="$(printf '%s\n' \
  ids.rs \
  mod.rs \
  normalized.rs \
  product.rs \
  records.rs \
  shadow/expr.rs \
  shadow/ids.rs \
  shadow/mod.rs \
  shadow/path.rs \
  shadow/product.rs \
  shadow/resolver.rs \
  shadow/stmt.rs \
  shadow/tests.rs \
  shadow/vocabulary.rs \
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
mapfile -t SHADOW_FILES < <(
  find "$MODULE/shadow" -type f -name '*.rs' ! -name 'tests.rs' -print | LC_ALL=C sort
)

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
  "pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, BindingRefV1>" \
  "pub(crate) assignment_targets:" \
  "pub(crate) control_exits:"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/product.rs" \
    "sealed semantic product schema drifted: $required"
done

for required in \
  "pub struct FunctionOwnerIdV1" \
  "pub(crate) struct FunctionOwnerIssuerV1" \
  "pub struct BindingRefV1" \
  "pub struct ScopeId" \
  "pub struct RegionId"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/ids.rs" \
    "owner-scoped identity schema drifted: $required"
done

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
  "${CANONICAL_FILES[@]}"; then
  guard_fail "$TAG" "canonical resolved semantic arena must not own cloned AST payloads"
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
  "BTreeMap<SourceExprSiteV1, ShadowBindingOrdinalV0>" \
  "BTreeMap<SourceStmtSiteV1, ShadowControlExitV0>"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/product.rs" \
    "shadow product schema drifted: $required"
done
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
  "$MODULE/mod.rs" "$MODULE/shadow/mod.rs"; then
  guard_fail "$TAG" "shadow resolver/product must remain crate-private and disconnected"
fi

for required in \
  "SHADOW_ACCEPTED_STATEMENTS_V0" \
  "SHADOW_ACCEPTED_EXPRESSIONS_V0" \
  "SHADOW_ACCEPTED_ASSIGNMENT_TARGETS_V0"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/shadow/vocabulary.rs" \
    "shadow accepted vocabulary manifest drifted: $required"
done
for variant in Local Outbox Assignment ScopeBox If Loop Break Continue Return; do
  guard_expect_fixed_in_file "$TAG" "ASTNode::$variant" "$MODULE/shadow/stmt.rs" \
    "accepted statement lost its explicit resolver arm: $variant"
done
for variant in Literal Variable Me This UnaryOp BinaryOp MethodCall FieldAccess Index FunctionCall New; do
  guard_expect_fixed_in_file "$TAG" "ASTNode::$variant" "$MODULE/shadow/expr.rs" \
    "accepted expression lost its explicit resolver arm: $variant"
done

python3 - "$MODULE/shadow/expr.rs" "$MODULE/shadow/stmt.rs" <<'PY'
from pathlib import Path
import re
import sys

expr = set(re.findall(r"ASTNode::(\w+)", Path(sys.argv[1]).read_text()))
stmt = set(re.findall(r"ASTNode::(\w+)", Path(sys.argv[2]).read_text()))
expected_expr = {
    "Literal", "Variable", "Me", "This", "UnaryOp", "BinaryOp",
    "MethodCall", "FieldAccess", "Index", "FunctionCall", "New",
}
expected_stmt = expected_expr | {
    "Local", "Outbox", "Assignment", "ScopeBox", "If", "Loop",
    "Break", "Continue", "Return",
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
  "${CANONICAL_FILES[@]}"; then
  guard_fail "$TAG" "canonical resolved-semantics files must not allocate BindingIds"
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

if rg -n 'FunctionOwnerIdV1[[:space:]]*\(' \
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
    "scope",
    "region",
    "declaration_binding",
    "variable_binding",
    "assignment_target",
    "control_exit",
    "binding_count",
    "scope_count",
    "region_count",
    "normalized_graph",
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
    "$MIR_MOD"|"$MODULE"/*)
      ;;
    *)
      guard_fail "$TAG" "SA0 external product connection must remain zero: $consumer"
      ;;
  esac
done <<< "$consumer_output"

while IFS= read -r file; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached the 800-line stop boundary: $file ($lines)"
  fi
done < <(find "$MODULE" -type f -name '*.rs' -print; printf '%s\n' "$MIR_MOD")

if ! rg -q 'pub\(super\) struct BindingId\(u32\)' \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"; then
  guard_fail "$TAG" "private ownership BindingId inventory drifted; reclassify before migration"
fi

cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::tests
cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::shadow::tests

echo "semantic_arena_schema=present"
echo "semantic_arena_ast_clone_fields=0"
echo "semantic_arena_binding_allocator_calls=0"
echo "semantic_arena_value_id_imports=0"
echo "semantic_arena_basic_block_id_imports=0"
echo "semantic_arena_external_consumers=0"
echo "semantic_arena_planner_connection=0"
echo "semantic_arena_lower_connection=0"
echo "semantic_arena_source_files_under_800=1"
echo "shadow_resolver_canonical_binding_ids=0"
echo "shadow_resolver_external_consumers=0"
echo "shadow_resolver_planner_connection=0"
echo "shadow_resolver_lower_connection=0"
echo "ownership_private_binding_id_imports=0"
echo "summary=ok"
echo "[$TAG] ok"
