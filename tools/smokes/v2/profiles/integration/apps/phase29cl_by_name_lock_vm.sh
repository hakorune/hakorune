#!/bin/bash
# Phase 29cl BYN-min1:
# lock `invoke_by_name_i64` owner set before further caller cutover.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

run_step() {
    local cmd="$1"
    if ! (cd "$NYASH_ROOT" && bash -lc "$cmd"); then
        test_fail "phase29cl_by_name_lock_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "bash tools/checks/phase29cl_by_name_mainline_guard.sh"
run_step "! rg -n 'invoke_by_name_i64|nyash\\.plugin\\.invoke_by_name_i64' lang/src/runner/launcher.hako"
run_step "! rg -n '\"selfhost\\.shared\\.backend\\.llvm_backend\"' lang/src/runner/launcher.hako"
run_step "! rg -n 'LlvmBackendBox\\.compile_obj|LlvmBackendBox\\.link_exe' lang/src/runner/launcher.hako"
run_step "rg -n 'MirBuilderBox\\.emit_root_from_source_v0|LlvmBackendBox\\.compile_obj_root|LlvmBackendBox\\.link_exe' lang/src/runner/launcher/compile_facade_impl.hako"
run_step "! rg -n 'compile_json_path|_call_codegen_compile_json_path|_emit_exe_from_mir_json_checked' lang/src/runner/launcher.hako"
run_step "bash tools/hakorune_emit_mir.sh lang/src/runner/entry/launcher_native_entry.hako /tmp/phase29cl_launcher_cutover.mir.json"
run_step "bash tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh"

test_pass "phase29cl_by_name_lock_vm: PASS (BYN-min1 guard stays green, launcher build-exe route is root-first, and launcher source lane stays off explicit by-name/module-string backend literal)"
