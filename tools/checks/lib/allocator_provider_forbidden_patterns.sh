#!/usr/bin/env bash
# Shared allocator-provider guard checks.
#
# Callers must define fail() before invoking these functions.

ALLOCATOR_PROVIDER_FORBIDDEN_SOURCE_ROOTS=(src crates lang/c-abi/shims lang/src -g '!**/*.md')
ALLOCATOR_PROVIDER_FORBIDDEN_SELECTION_PATTERN='(^|[^A-Za-z0-9_])select_allocator_provider([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_select([^A-Za-z0-9_]|$)|(^|[^A-Za-z0-9_])allocator_provider_selection_env([^A-Za-z0-9_]|$)|NYASH_ALLOCATOR_PROVIDER'
ALLOCATOR_PROVIDER_FORBIDDEN_GLOBAL_ALLOCATOR_PATTERN='#\[global_allocator\]|GlobalAlloc'
ALLOCATOR_PROVIDER_FORBIDDEN_PROOF_CONSUMPTION_PATTERN='consume_allocator_provider_proof|allocator_provider_proof_bundle_consume|consume_allocator_provider_proof_bundle|consume_provider_proof_bundle'
ALLOCATOR_PROVIDER_FORBIDDEN_ROLLBACK_PREPARATION_PATTERN='(^|[^A-Za-z0-9_])prepare_rollback([^A-Za-z0-9_]|$)'
ALLOCATOR_PROVIDER_FORBIDDEN_HOOK_ACTIVATION_PATTERN='allocator_hook_activate|activate_allocator|install_allocator_hook|replace_allocator'
ALLOCATOR_PROVIDER_FORBIDDEN_ACTIVATION_GATE_PATTERN='(^|[^A-Za-z0-9_])open_activation_gate([^A-Za-z0-9_]|$)'
ALLOCATOR_PROVIDER_FORBIDDEN_INC_MATCHER_PATTERN='HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator|debug_guarded_allocator|hako_model_allocator|native_mimalloc|native_system_malloc'

allocator_provider_forbid_source_pattern() {
  local tag="$1"
  local tmp_suffix="$2"
  local pattern="$3"
  local message="$4"
  local tmp="/tmp/${tag}.${tmp_suffix}"

  if rg -n "$pattern" "${ALLOCATOR_PROVIDER_FORBIDDEN_SOURCE_ROOTS[@]}" >"$tmp" 2>&1; then
    cat "$tmp" >&2
    rm -f "$tmp"
    fail "$message"
  fi
  rm -f "$tmp"
}

allocator_provider_forbid_selection() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    provider_selection \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_SELECTION_PATTERN" \
    "provider selection implementation/env toggle must stay absent"
}

allocator_provider_forbid_global_allocator() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    global_allocator \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_GLOBAL_ALLOCATOR_PATTERN" \
    "process allocator replacement must stay inactive"
}

allocator_provider_forbid_proof_consumption() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    proof_consumption \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_PROOF_CONSUMPTION_PATTERN" \
    "proof consumption implementation must stay absent"
}

allocator_provider_forbid_rollback_preparation() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    rollback \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_ROLLBACK_PREPARATION_PATTERN" \
    "rollback preparation implementation must stay absent"
}

allocator_provider_forbid_hook_activation() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    hook_activation \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_HOOK_ACTIVATION_PATTERN" \
    "hook activation/process allocator replacement must stay absent"
}

allocator_provider_forbid_activation_gate_open() {
  allocator_provider_forbid_source_pattern \
    "$1" \
    gate_open \
    "$ALLOCATOR_PROVIDER_FORBIDDEN_ACTIVATION_GATE_PATTERN" \
    "activation gate opening must stay absent"
}

allocator_provider_forbid_inc_matchers() {
  local tag="$1"
  local tmp="/tmp/${tag}.inc"

  if rg -n "$ALLOCATOR_PROVIDER_FORBIDDEN_INC_MATCHER_PATTERN" lang/c-abi/shims >"$tmp" 2>&1; then
    cat "$tmp" >&2
    rm -f "$tmp"
    fail "allocator provider/hook/facade/policy matcher leaked into .inc"
  fi
  rm -f "$tmp"
}
