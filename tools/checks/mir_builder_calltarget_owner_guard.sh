#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-builder-calltarget-owner-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
cd "$ROOT_DIR"

BUILDER_ROOT="src/mir/builder.rs"
COMPAT_SHELL="src/mir/builder/builder_calls.rs"
BOX_KIND_POLICY="src/mir/policies/callee_box_kind.rs"
CALL_NAME_POLICY="src/mir/policies/call_name_classification.rs"
BOX_KIND_CONSUMERS=(
  "src/mir/builder/calls/resolver.rs"
  "src/mir/builder/calls/unified_emitter.rs"
  "src/mir/builder/calls/method_resolution.rs"
  "src/mir/builder/utils/boxcall_emit.rs"
)
CALL_NAME_CONSUMERS=(
  "src/mir/builder/calls/build.rs"
  "src/mir/builder/calls/resolver.rs"
  "src/mir/builder/calls/method_resolution.rs"
)

guard_require_command "$TAG" rg
guard_require_files \
  "$TAG" \
  "$BUILDER_ROOT" \
  "src/mir/builder/calls/call_target.rs" \
  "$BOX_KIND_POLICY" \
  "$CALL_NAME_POLICY" \
  "${BOX_KIND_CONSUMERS[@]}" \
  "${CALL_NAME_CONSUMERS[@]}"

echo "[$TAG] checking CallTarget owner path"

if [ -e "$COMPAT_SHELL" ]; then
  echo "[$TAG] ERROR: builder_calls compatibility shell reintroduced: $COMPAT_SHELL" >&2
  exit 1
fi

bad_builder_calls_module="$(
  rg -n '^\s*mod\s+builder_calls\s*;' "$BUILDER_ROOT" || true
)"
if [ -n "$bad_builder_calls_module" ]; then
  echo "[$TAG] ERROR: builder_calls module reintroduced" >&2
  printf '%s\n' "$bad_builder_calls_module" >&2
  exit 1
fi

guard_expect_in_file \
  "$TAG" \
  '^pub\(crate\) use calls::CallTarget;' \
  "$BUILDER_ROOT" \
  "builder root must re-export CallTarget from calls::CallTarget"

bad_builder_calls_path="$(
  rg -n 'builder_calls::CallTarget' src/mir/builder -g '*.rs' || true
)"
if [ -n "$bad_builder_calls_path" ]; then
  echo "[$TAG] ERROR: CallTarget imported through builder_calls compatibility path" >&2
  printf '%s\n' "$bad_builder_calls_path" >&2
  exit 1
fi

bad_compat_reexport="$(
  rg -n 'pub use super::calls::call_target::CallTarget|pub use super::calls::CallTarget' \
    src/mir/builder -g '*.rs' || true
)"
if [ -n "$bad_compat_reexport" ]; then
  echo "[$TAG] ERROR: builder_calls CallTarget compatibility re-export found" >&2
  printf '%s\n' "$bad_compat_reexport" >&2
  exit 1
fi

guard_expect_in_file \
  "$TAG" \
  '^pub\(crate\) fn classify_callee_box_kind_v1\(' \
  "$BOX_KIND_POLICY" \
  "neutral CalleeBoxKind policy owner must exist"

old_box_kind_owner="$(
  rg -n '\bclassify_box_kind\b' src/mir -g '*.rs' || true
)"
if [ -n "$old_box_kind_owner" ]; then
  echo "[$TAG] ERROR: old local CalleeBoxKind classifier remains" >&2
  printf '%s\n' "$old_box_kind_owner" >&2
  exit 1
fi

box_kind_calls="$(
  rg -n 'classify_callee_box_kind_v1\(' "${BOX_KIND_CONSUMERS[@]}" | wc -l
)"
if [ "$box_kind_calls" -ne 6 ]; then
  guard_fail "$TAG" "expected 6 production CalleeBoxKind policy calls, got $box_kind_calls"
fi

extended_contexts="$(
  rg -n 'CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler' \
    "${BOX_KIND_CONSUMERS[@]}" | wc -l
)"
if [ "$extended_contexts" -ne 2 ]; then
  guard_fail "$TAG" "expected 2 resolver-extended contexts, got $extended_contexts"
fi

general_contexts="$(
  rg -n 'CalleeBoxKindPolicyContextV1::GeneralEmission' \
    "${BOX_KIND_CONSUMERS[@]}" | wc -l
)"
if [ "$general_contexts" -ne 4 ]; then
  guard_fail "$TAG" "expected 4 general-emission contexts, got $general_contexts"
fi

guard_expect_in_file \
  "$TAG" \
  '^pub\(crate\) fn classify_call_name_v1\(' \
  "$CALL_NAME_POLICY" \
  "neutral call-name classification owner must exist"

old_call_name_owners="$(
  rg -n '\bfn\s+is_(builtin|extern)_function\(' src/mir -g '*.rs' || true
)"
if [ -n "$old_call_name_owners" ]; then
  echo "[$TAG] ERROR: old local call-name classifier remains" >&2
  printf '%s\n' "$old_call_name_owners" >&2
  exit 1
fi

for consumer in "${CALL_NAME_CONSUMERS[@]}"; do
  call_count="$(rg -n 'classify_call_name_v1\(' "$consumer" | wc -l)"
  if [ "$call_count" -ne 1 ]; then
    guard_fail "$TAG" "expected one call-name classification in $consumer, got $call_count"
  fi
done

old_call_name_calls="$(
  rg -n '\b(is_builtin_function|is_extern_function)\(' src/mir/builder -g '*.rs' || true
)"
if [ -n "$old_call_name_calls" ]; then
  echo "[$TAG] ERROR: old call-name predicate call remains" >&2
  printf '%s\n' "$old_call_name_calls" >&2
  exit 1
fi

build_raw_fact="$( (rg -n '\.raw_unified_admission\(\)' "${CALL_NAME_CONSUMERS[0]}" || true) | wc -l)"
build_callee_fact="$( (rg -n '\.callee_class\(\)' "${CALL_NAME_CONSUMERS[0]}" || true) | wc -l)"
resolver_raw_fact="$( (rg -n '\.raw_unified_admission\(\)' "${CALL_NAME_CONSUMERS[1]}" || true) | wc -l)"
resolver_callee_fact="$( (rg -n '\.callee_class\(\)' "${CALL_NAME_CONSUMERS[1]}" || true) | wc -l)"
method_raw_fact="$( (rg -n '\.raw_unified_admission\(\)' "${CALL_NAME_CONSUMERS[2]}" || true) | wc -l)"
method_callee_fact="$( (rg -n '\.callee_class\(\)' "${CALL_NAME_CONSUMERS[2]}" || true) | wc -l)"
if [ "$build_raw_fact" -ne 1 ] || [ "$build_callee_fact" -ne 0 ] \
  || [ "$resolver_raw_fact" -ne 0 ] || [ "$resolver_callee_fact" -ne 1 ] \
  || [ "$method_raw_fact" -ne 0 ] || [ "$method_callee_fact" -ne 2 ]; then
  guard_fail "$TAG" "call-name fact projection drift"
fi

unexpected_call_name_consumers="$(
  rg -l 'classify_call_name_v1\(' src/mir -g '*.rs' \
    | grep -v '^src/mir/policies/call_name_classification.rs$' \
    | grep -v -F -x -f <(printf '%s\n' "${CALL_NAME_CONSUMERS[@]}") \
    || true
)"
if [ -n "$unexpected_call_name_consumers" ]; then
  echo "[$TAG] ERROR: unregistered call-name classification consumer" >&2
  printf '%s\n' "$unexpected_call_name_consumers" >&2
  exit 1
fi

echo "[$TAG] ok"
