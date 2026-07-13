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
  "$MODULE/product.rs" \
  "$MODULE/tests.rs" \
  "$MIR_MOD" \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"

expected_manifest="$(printf '%s\n' \
  ids.rs \
  mod.rs \
  product.rs \
  records.rs \
  source_site.rs \
  tests.rs)"
actual_manifest="$(find "$MODULE" -type f -name '*.rs' -printf '%P\n' | LC_ALL=C sort)"
if [[ "$actual_manifest" != "$expected_manifest" ]]; then
  printf '%s\n' "$actual_manifest" >&2
  guard_fail "$TAG" "SA0 Rust source manifest drifted; classify every new file explicitly"
fi

mapfile -t PRODUCTION_FILES < <(
  find "$MODULE" -type f -name '*.rs' ! -name 'tests.rs' -print | LC_ALL=C sort
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
  "pub struct BindingRefV1" \
  "pub struct ScopeId" \
  "pub struct RegionId"; do
  guard_expect_fixed_in_file "$TAG" "$required" "$MODULE/ids.rs" \
    "owner-scoped identity schema drifted: $required"
done

if rg -n 'ASTNode|Box[[:space:]]*<[[:space:]]*AST|Vec[[:space:]]*<[[:space:]]*AST' \
  "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "resolved semantic arena must not own cloned AST payloads"
fi

if rg -n 'allocate_binding_id|next_binding|BindingId::new|BindingId[[:space:]]*\(' \
  "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "SA0 resolved-semantics production files must not allocate BindingIds"
fi
if rg -n 'BindingId[[:space:]]+as|hakorune_mir_core[[:space:]]+as|hakorune_mir_core::\*|type[[:space:]].*BindingId|\.next[[:space:]]*\(' \
  "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "SA0 must not alias or indirectly advance the canonical BindingId"
fi
binding_imports="$(rg '^[[:space:]]*use[[:space:]]+hakorune_mir_core::.*BindingId' \
  "${PRODUCTION_FILES[@]}" || true)"
while IFS= read -r import_line; do
  [[ -z "$import_line" ]] && continue
  if [[ "${import_line#*:}" != "use hakorune_mir_core::BindingId;" ]]; then
    guard_fail "$TAG" "canonical BindingId import must use the exact non-aliased spelling: $import_line"
  fi
done <<< "$binding_imports"

if rg -n 'ValueId|BasicBlockId|MirBuilder|CoreContext' "${PRODUCTION_FILES[@]}"; then
  guard_fail "$TAG" "SA0 schema must not import MIR materialization owners"
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
    "from_unverified_data_for_schema_test",
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
done < <(printf '%s\n' "${PRODUCTION_FILES[@]}" "$MODULE/tests.rs" "$MIR_MOD")

if ! rg -q 'pub\(super\) struct BindingId\(u32\)' \
  "$ROOT/src/mir/join_ir/ownership/ast_analyzer/core.rs"; then
  guard_fail "$TAG" "private ownership BindingId inventory drifted; reclassify before migration"
fi

cargo test -q --manifest-path "$ROOT/Cargo.toml" --lib mir::resolved_semantics::tests

echo "semantic_arena_schema=present"
echo "semantic_arena_ast_clone_fields=0"
echo "semantic_arena_binding_allocator_calls=0"
echo "semantic_arena_value_id_imports=0"
echo "semantic_arena_basic_block_id_imports=0"
echo "semantic_arena_external_consumers=0"
echo "semantic_arena_planner_connection=0"
echo "semantic_arena_lower_connection=0"
echo "semantic_arena_source_files_under_800=1"
echo "ownership_private_binding_id_imports=0"
echo "summary=ok"
echo "[$TAG] ok"
