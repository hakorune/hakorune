#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-callable-loop-root-unpublished-scope-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SESSION="$ROOT_DIR/src/mir/builder/module_invocation_session.rs"
LIFECYCLE="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle.rs"
POST_INSTALL="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_post_install.rs"
ROOT_LOWER="$ROOT_DIR/src/mir/builder/program_root_lowering.rs"
RAW_PORT="$ROOT_DIR/src/mir/builder/raw_loop_child_port.rs"
RAW_ENTRY="$ROOT_DIR/src/mir/builder/raw_loop_child_entry.rs"
RECURSIVE="$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs"
ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_loop_physical_adapter.rs"
TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-callable-loop-root-unpublished-scope-i0-2026-08-23.md"
README="$ROOT_DIR/src/mir/builder/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_callable_loop_root_unpublished_scope_i0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$SESSION" "$LIFECYCLE" "$POST_INSTALL" \
  "$ROOT_LOWER" "$RAW_PORT" "$RAW_ENTRY" "$RECURSIVE" "$ADAPTER" "$TESTS" \
  "$CARD" "$README" "$INDEX"

for pattern_file in \
  "$SESSION|UnpublishedCallableLoopRootScopeV1" \
  "$SESSION|with_builder_and_pinned_text_invocation_binding_and_callable_loop_scope" \
  "$SESSION|validate_collector" \
  "$LIFECYCLE|with_builder_and_pinned_text_invocation_binding_and_callable_loop_scope" \
  "$POST_INSTALL|callable_loop_root_scope" \
  "$ROOT_LOWER|callable_loop_root_scope.validate_collector" \
  "$ROOT_LOWER|new_with_cleanup_exit_policy_and_callable_loop_scope" \
  "$RAW_PORT|self.callable_loop_root_scope.as_deref_mut" \
  "$RAW_ENTRY|lower_v1_with_root_scope" \
  "$RAW_ENTRY|callable-loop/root-scope/missing" \
  "$ADAPTER|_root_scope: &mut UnpublishedCallableLoopRootScopeV1" \
  "$TESTS|ready_source_facts_requires_the_unpublished_root_scope_before_physical_lowering"; do
  path="${pattern_file%%|*}"
  pattern="${pattern_file#*|}"
  guard_expect_fixed_in_file "$TAG" "$pattern" "$path" "root Ready scope contract must remain visible"
done

guard_expect_fixed_in_file "$TAG" "Callable Loop root unpublished scope I0" "$CARD" \
  "active I0 card must remain the root-scope implementation contract"
guard_expect_fixed_in_file "$TAG" "ModuleBuilderInvocationSessionV1" "$README" \
  "builder README must name the root unpublished owner"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the reusable root-scope guard"

if rg -n -U -- 'fn lower\(\s*builder: &mut MirBuilder,\s*recipe:' "$ADAPTER"; then
  guard_fail "$TAG" "physical adapter still exposes the bare Builder+Recipe production entry"
fi

adapter_callers="$(rg --glob '*.rs' --glob '!normal_callable_loop_source_facts_tests.rs' -F -o \
  -- 'CallableGenericLoopV1PhysicalAdapterV1::lower(' "$ROOT_DIR/src/mir/builder" \
  | wc -l | tr -d '[:space:]')"
if [[ "$adapter_callers" -ne 1 ]]; then
  guard_fail "$TAG" "Ready physical adapter must have one production caller; found $adapter_callers"
fi

root_scope_ctor_calls="$(rg --glob '*.rs' -F -o \
  -- 'new_with_cleanup_exit_policy_and_callable_loop_scope(' "$ROOT_DIR/src/mir/builder" \
  | wc -l | tr -d '[:space:]')"
if [[ "$root_scope_ctor_calls" -ne 2 ]]; then
  guard_fail "$TAG" "root-scoped raw port constructor must have one definition and one root caller; found $root_scope_ctor_calls"
fi

scoped_lifecycle_calls="$(rg -F -o -- 'with_builder_and_pinned_text_invocation_binding_and_callable_loop_scope(' "$LIFECYCLE" \
  | wc -l | tr -d '[:space:]')"
if [[ "$scoped_lifecycle_calls" -ne 1 ]]; then
  guard_fail "$TAG" "root lifecycle must bind the scope exactly once; found $scoped_lifecycle_calls"
fi

if rg -n -- 'with_builder_and_pinned_text_invocation_binding\(' "$LIFECYCLE"; then
  guard_fail "$TAG" "root lifecycle still uses the unscoped candidate callback"
fi

ready_branch="$(awk '
/Some\(CallableLoopBindingProjectionDispositionV1::Ready/ { inside = 1 }
inside { print }
inside && /Some\(CallableLoopBindingProjectionDispositionV1::Outside/ { exit }
' "$RAW_ENTRY")"
if rg -n -- 'lower_loop_or_freeze_v1|lower_non_callable_loop_legacy_v1|retry|fallback' <<<"$ready_branch"; then
  guard_fail "$TAG" "Ready branch still has a legacy/fallback route"
fi
if ! rg -q -- 'CallableGenericLoopV1PhysicalAdapterV1::lower\(builder, root_scope, recipe\)' <<<"$ready_branch"; then
  guard_fail "$TAG" "Ready branch does not consume the scoped physical adapter"
fi

if rg -n -- 'CanonicalFunctionLoweringSessionV1|ModuleLoweringInvocationV1' \
  "$ROOT_LOWER" "$RAW_PORT" "$RAW_ENTRY" "$ADAPTER"; then
  guard_fail "$TAG" "root Ready path opened a second function/module session"
fi

drain_calls="$(rg -F -o -- 'prepare_normal_collector_drain(' "$ROOT_LOWER" | wc -l | tr -d '[:space:]')"
commit_calls="$(rg -F -o -- 'prepared.commit();' "$ROOT_LOWER" | wc -l | tr -d '[:space:]')"
if [[ "$drain_calls" -ne 1 || "$commit_calls" -ne 1 ]]; then
  guard_fail "$TAG" "root collector drain/commit shape changed; drain=$drain_calls commit=$commit_calls"
fi

for file in "$SESSION" "$ROOT_LOWER" "$RAW_PORT" "$RAW_ENTRY" "$RECURSIVE" "$ADAPTER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one unpublished root scope, one scoped Ready adapter, no Ready fallback, one collector drain/commit)"
