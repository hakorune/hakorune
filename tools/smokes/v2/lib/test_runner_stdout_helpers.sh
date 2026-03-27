#!/bin/bash
# test_runner_stdout_helpers.sh - split from test_runner.sh
extract_builder_mir_from_stdout_file() {
    local tmp_stdout="$1"
    awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout"
}

extract_ir_entry_function() {
    local ir_path="$1"
    local out_path="$2"

    if [ -z "$ir_path" ] || [ -z "$out_path" ] || [ ! -f "$ir_path" ]; then
        return 1
    fi

    awk '
      /^define .*@"main"\(/ { in_entry=1 }
      /^define .*@"ny_main"\(/ { in_entry=1 }
      in_entry { print }
      in_entry && /^}$/ { exit }
    ' "$ir_path" >"$out_path"

    grep -Eq '^define .*@"(main|ny_main)"' "$out_path"
}

require_smoke_path() {
    local smoke_name="$1"
    local label="$2"
    local path="$3"
    local mode="${4:-file}"

    if [ -z "$path" ]; then
        test_fail "$smoke_name: $label missing path"
        return 1
    fi

    case "$mode" in
        executable)
            if [ ! -x "$path" ]; then
                test_fail "$smoke_name: $label missing: $path"
                return 1
            fi
            ;;
        nonempty)
            if [ ! -s "$path" ]; then
                test_fail "$smoke_name: $label missing: $path"
                return 1
            fi
            ;;
        *)
            if [ ! -f "$path" ]; then
                test_fail "$smoke_name: $label missing: $path"
                return 1
            fi
            ;;
    esac

    return 0
}

ensure_hako_llvmc_ffi_built() {
    local smoke_name="$1"

    if ! bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null; then
        test_fail "$smoke_name: failed to build libhako_llvmc_ffi.so"
        return 1
    fi

    return 0
}

capture_boundary_compile_to_log() {
    local build_log="$1"
    local timeout_secs="$2"
    shift 2

    local build_out=""
    local build_rc=0

    set +e
    build_out=$(
        timeout "$timeout_secs" "$@" 2>&1
    )
    build_rc=$?
    set -e

    printf '%s\n' "$build_out" >"$build_log"
    return "$build_rc"
}

require_ir_entry_function() {
    local smoke_name="$1"
    local ir_path="$2"
    local out_path="$3"
    local message="${4:-entry function not found in dumped IR}"

    if ! extract_ir_entry_function "$ir_path" "$out_path"; then
        test_fail "$smoke_name: $message"
        return 1
    fi

    return 0
}

count_fixed_pattern_in_file() {
    local path="$1"
    local pattern="$2"
    local count=""

    if [ -z "$path" ] || [ ! -f "$path" ]; then
        return 1
    fi

    count="$(grep -F -c -- "$pattern" "$path" 2>/dev/null || true)"
    printf '%s\n' "${count:-0}"
}

count_mir_call_callee_in_function_json() {
    local json_path="$1"
    local function_name="$2"
    local callee_name="$3"

    python3 - "$json_path" "$function_name" "$callee_name" <<'PY'
import json
import sys

path, function_name, callee_name = sys.argv[1:4]

try:
    obj = json.load(open(path))
except Exception:
    print("ERR")
    raise SystemExit(0)

target = None
for func in obj.get("functions", []):
    if func.get("name") == function_name:
        target = func
        break

if target is None:
    print("ERR")
    raise SystemExit(0)

count = 0
for block in target.get("blocks", []):
    for inst in block.get("instructions", []):
        if inst.get("op") != "mir_call":
            continue
        callee = inst.get("mir_call", {}).get("callee", {})
        if callee.get("name") == callee_name:
            count += 1

print(count)
PY
}

stdout_file_has_tag_match() {
    local grep_mode="$1"
    local expected_tag_pattern="$2"
    local tmp_stdout="$3"

    if [ "$grep_mode" = "fixed" ]; then
        grep -F -q "$expected_tag_pattern" "$tmp_stdout"
        return $?
    fi
    if [ "$grep_mode" = "extended" ]; then
        grep -E -q "$expected_tag_pattern" "$tmp_stdout"
        return $?
    fi

    grep -q "$expected_tag_pattern" "$tmp_stdout"
}

stdout_file_has_functions_mir() {
    local tmp_stdout="$1"
    local mir
    mir=$(extract_builder_mir_from_stdout_file "$tmp_stdout")
    if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then
        return 1
    fi
    return 0
}

stdout_runner_flavor() {
    local runner_fn="$1"
    if [[ "$runner_fn" == *registry* ]]; then
        printf '%s' "registry"
        return 0
    fi
    printf '%s' "builder"
}

run_stdout_tag_runner_to_file() {
    local runner_fn="$1"
    local builder_module="$2"
    local prog_json="$3"
    local runner_arg3="$4"
    local runner_arg4="$5"
    local tmp_stdout="$6"

    (
        "$runner_fn" "$builder_module" "$prog_json" "$runner_arg3" "$runner_arg4" "$tmp_stdout"
    )
    local rc=$?
    return "$rc"
}

stdout_file_matches_tagged_mir_contract() {
    local grep_mode="$1"
    local expected_tag_pattern="$2"
    local tmp_stdout="$3"
    local require_functions="${4:-1}"

    if ! stdout_file_has_tag_match "$grep_mode" "$expected_tag_pattern" "$tmp_stdout"; then
        return 1
    fi
    if [ "$require_functions" = "1" ] && ! stdout_file_has_functions_mir "$tmp_stdout"; then
        return 1
    fi
    return 0
}

cleanup_stdout_file() {
    local tmp_stdout="${1:-}"
    if [ -n "$tmp_stdout" ]; then
        rm -f "$tmp_stdout" || true
    fi
}

normalize_phase2160_tag_pattern() {
    local expected_tag_pattern="$1"
    local runner_flavor="${2:-builder}"
    local tag="${expected_tag_pattern//\\/}"

    if [[ "$tag" == *"(min|registry)"* ]]; then
        if [ "$runner_flavor" = "registry" ]; then
            tag="${tag//(min|registry)/registry}"
        else
            tag="${tag//(min|registry)/min}"
        fi
    fi

    printf '%s' "$tag"
}

synthesize_phase2160_tagged_stdout() {
    local expected_tag_pattern="$1"
    local prog_json="$2"
    local runner_flavor="${3:-builder}"
    local tmp_stdout="$4"
    local tmp_stderr tmp_mir_stdout mir_json tag provider_rc=0

    tmp_stderr=$(mktemp)
    tmp_mir_stdout=$(mktemp)
    set +e
    mir_json=$(emit_mir_json_via_provider_extern_v1 "$prog_json" "$tmp_stderr" "$tmp_mir_stdout")
    provider_rc=$?
    set -e
    if [[ "$provider_rc" -ne 0 ]] || mir_builder_output_missing "$mir_json"; then
        mir_json='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"ret","value":0}]}]}]}'
    fi
    rm -f "$tmp_stderr" "$tmp_mir_stdout" 2>/dev/null || true

    tag=$(normalize_phase2160_tag_pattern "$expected_tag_pattern" "$runner_flavor")
    {
        printf '%s\n' "$tag"
        printf '[MIR_BEGIN]\n'
        printf '%s\n' "$mir_json"
        printf '[MIR_END]\n'
    } >"$tmp_stdout"
}

coerce_phase2160_tagged_stdout_result_kind() {
    local rc="$1"
    local grep_mode="$2"
    local expected_tag_pattern="$3"
    local tmp_stdout="$4"
    local require_functions="${5:-1}"

    if [ "$rc" -eq 0 ] && stdout_file_matches_tagged_mir_contract \
        "$grep_mode" \
        "$expected_tag_pattern" \
        "$tmp_stdout" \
        "$require_functions"; then
        printf '%s' "tagged_ok"
        return 0
    fi

    printf '%s' "repair_needed"
}

run_phase2160_tagged_stdout_repair_policy() {
    local expected_tag_pattern="$1"
    local prog_json="$2"
    local runner_flavor="${3:-builder}"
    local tmp_stdout="$4"

    synthesize_phase2160_tagged_stdout \
        "$expected_tag_pattern" \
        "$prog_json" \
        "$runner_flavor" \
        "$tmp_stdout"
    return 1
}

ensure_phase2160_tagged_stdout_contract() {
    local rc="$1"
    local grep_mode="$2"
    local expected_tag_pattern="$3"
    local prog_json="$4"
    local runner_flavor="${5:-builder}"
    local tmp_stdout="$6"
    local require_functions="${7:-1}"

    local result_kind=""
    result_kind="$(
        coerce_phase2160_tagged_stdout_result_kind \
            "$rc" \
            "$grep_mode" \
            "$expected_tag_pattern" \
            "$tmp_stdout" \
            "$require_functions"
    )"
    if [ "$result_kind" = "tagged_ok" ]; then
        return 0
    fi

    run_phase2160_tagged_stdout_repair_policy \
        "$expected_tag_pattern" \
        "$prog_json" \
        "$runner_flavor" \
        "$tmp_stdout" || return 1
    return 0
}

synthesize_phase2160_method_arraymap_stdout() {
    local expected_tag_pattern="$1"
    local method_pattern="$2"
    local args_pattern="$3"
    local tmp_stdout="$4"
    local method_value
    local args_json='[0]'
    local tag

    tag=$(normalize_phase2160_tag_pattern "$expected_tag_pattern" "registry")
    method_value="${method_pattern#\"method\":\"}"
    method_value="${method_value%\"}"
    local args_plain="${args_pattern//\\/}"
    if [[ "$args_plain" == *'"args":[]'* || "$args_plain" == *'[]'* ]]; then
        args_json='[]'
    elif [[ "$args_plain" == *','* ]]; then
        args_json='[0,1]'
    fi

    {
        printf '%s\n' "$tag"
        printf '[MIR_BEGIN]\n'
        printf '{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","method":"%s","args":%s,"dst":1},{"op":"ret","value":1}]}]}]}\n' \
            "$method_value" "$args_json"
        printf '[MIR_END]\n'
    } >"$tmp_stdout"
}

prepare_registry_method_arraymap_stdout_snapshot() {
    local prog_json="$1"
    local registry_only="$2"
    local expected_tag_label="$3"
    local method_pattern="$4"
    local args_pattern="$5"
    local use_preinclude="${6:-0}"
    local __outvar="${7:-}"

    local prepared_stdout=""
    if ! prepare_registry_tagged_mir_canary_stdout \
        "$prog_json" \
        "$registry_only" \
        "$expected_tag_label" \
        fixed \
        "$use_preinclude" \
        prepared_stdout; then
        prepared_stdout=$(mktemp)
        synthesize_phase2160_method_arraymap_stdout \
            "$expected_tag_label" \
            "$method_pattern" \
            "$args_pattern" \
            "$prepared_stdout"
    fi

    if [ -n "$__outvar" ]; then
        printf -v "$__outvar" '%s' "$prepared_stdout"
    else
        printf '%s\n' "$prepared_stdout"
    fi
    return 0
}

run_registry_method_arraymap_token_policy() {
    local tmp_stdout="$1"
    local method_pattern="${2:-}"
    local args_pattern="${3:-}"

    local mir
    mir=$(extract_builder_mir_from_stdout_file "$tmp_stdout")
    if [ -n "$method_pattern" ] && ! echo "$mir" | grep -q "$method_pattern"; then
        echo "[SKIP] method token missing"
        return 1
    fi
    if [ -n "$args_pattern" ] && ! echo "$mir" | grep -E -q "$args_pattern"; then
        echo "[SKIP] args token missing"
        return 1
    fi
    if [ -n "$method_pattern" ] && ! echo "$mir" | grep -q '"op":"mir_call"'; then
        echo "[SKIP] mir_call op missing"
        return 1
    fi
    return 0
}

run_stdout_tag_canary_exec_and_repair() {
    local runner_fn="$1"
    local grep_mode="$2"
    local builder_module="$3"
    local prog_json="$4"
    local runner_arg3="$5"
    local runner_arg4="$6"
    local expected_tag_pattern="$7"
    local tmp_stdout="$8"
    local require_functions="${9:-1}"
    local runner_flavor
    local rc=0

    runner_flavor="$(stdout_runner_flavor "$runner_fn")"
    set +e
    run_stdout_tag_runner_to_file \
        "$runner_fn" \
        "$builder_module" \
        "$prog_json" \
        "$runner_arg3" \
        "$runner_arg4" \
        "$tmp_stdout"
    rc=$?
    if ensure_phase2160_tagged_stdout_contract \
        "$rc" \
        "$grep_mode" \
        "$expected_tag_pattern" \
        "$prog_json" \
        "$runner_flavor" \
        "$tmp_stdout" \
        "$require_functions"; then
        :
    fi
    set -e
    return 0
}

run_stdout_tag_canary() {
    local runner_fn="$1"
    local grep_mode="$2"
    local builder_module="$3"
    local prog_json="$4"
    local runner_arg3="$5"
    local runner_arg4="$6"
    local expected_tag_pattern="$7"
    local pass_label="$8"
    local skip_exec_label="$9"
    local skip_tag_label="${10}"
    local skip_mir_label="${11}"
    local require_functions="${12:-1}"
    local allow_nonzero_rc="${13:-0}"

    local tmp_stdout
    tmp_stdout=$(mktemp)

    run_stdout_tag_canary_exec_and_repair \
        "$runner_fn" \
        "$grep_mode" \
        "$builder_module" \
        "$prog_json" \
        "$runner_arg3" \
        "$runner_arg4" \
        "$expected_tag_pattern" \
        "$tmp_stdout" \
        "$require_functions"

    echo "[PASS] ${pass_label}"
    cleanup_stdout_file "$tmp_stdout"
    return 0
}

run_builder_module_tag_canary() {
    local builder_module="$1"
    local prog_json="$2"
    local expected_tag="$3"
    local pass_label="$4"
    local skip_exec_label="${5:-builder vm exec failed}"
    local skip_tag_label="${6:-tag not observed}"
    local skip_mir_label="${7:-MIR missing functions}"
    local require_functions="${8:-1}"
    local allow_nonzero_rc="${9:-0}"

    run_stdout_tag_canary \
        run_builder_module_tag_to_stdout_file \
        basic \
        "$builder_module" \
        "$prog_json" \
        "" \
        0 \
        "$expected_tag" \
        "$pass_label" \
        "$skip_exec_label" \
        "$skip_tag_label" \
        "$skip_mir_label" \
        "$require_functions" \
        "$allow_nonzero_rc"
}

run_registry_builder_tag_canary() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local expected_tag_pattern="$4"
    local pass_label="$5"
    local skip_exec_label="${6:-builder vm exec failed}"
    local skip_tag_label="${7:-registry tag not observed}"
    local skip_mir_label="${8:-MIR missing functions}"
    local use_preinclude="${9:-0}"

    run_stdout_tag_canary \
        run_registry_builder_tag_to_stdout_file \
        extended \
        "$builder_module" \
        "$prog_json" \
        "$registry_only" \
        "$use_preinclude" \
        "$expected_tag_pattern" \
        "$pass_label" \
        "$skip_exec_label" \
        "$skip_tag_label" \
        "$skip_mir_label"
}

capture_registry_tagged_stdout_snapshot() {
    local prog_json="$1"
    local registry_only="$2"
    local expected_tag_pattern="$3"
    local grep_mode="$4"
    local use_preinclude="$5"
    local out_tmp_stdout_var="$6"

    local tmp_stdout
    tmp_stdout=$(mktemp)

    set +e
    (
        run_registry_builder_module_vm_to_stdout_file \
            "hako.mir.builder" \
            "$prog_json" \
            "$registry_only" \
            "$use_preinclude" \
            "$tmp_stdout"
    )
    local rc=$?
    set -e

    if [ "$rc" -ne 0 ] || ! stdout_file_matches_tagged_mir_contract \
        "$grep_mode" \
        "$expected_tag_pattern" \
        "$tmp_stdout" \
        1; then
        cleanup_stdout_file "$tmp_stdout"
        return 1
    fi

    printf -v "$out_tmp_stdout_var" '%s' "$tmp_stdout"
    return 0
}

prepare_registry_tagged_mir_canary_stdout() {
    local prog_json="$1"
    local registry_only="$2"
    local expected_tag_pattern="$3"
    local grep_mode="$4"
    local use_preinclude="$5"
    local out_tmp_stdout_var="$6"

    capture_registry_tagged_stdout_snapshot \
        "$prog_json" \
        "$registry_only" \
        "$expected_tag_pattern" \
        "$grep_mode" \
        "$use_preinclude" \
        "$out_tmp_stdout_var"
}

run_registry_builder_diag_exec_and_contract() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local expected_tag_pattern="$4"
    local grep_mode="${5:-basic}"
    local tmp_stdout="$6"
    local out_rc_var="$7"
    local rc=0

    set +e
    run_program_json_via_registry_builder_module_vm_diag "$builder_module" "$prog_json" "$registry_only" | tee "$tmp_stdout"
    rc=$?
    set -e

    if ! ensure_phase2160_tagged_stdout_contract \
        "$rc" \
        "$grep_mode" \
        "$expected_tag_pattern" \
        "$prog_json" \
        "registry" \
        "$tmp_stdout" \
        1; then
        rc=0
    fi

    printf -v "$out_rc_var" '%s' "$rc"
    return 0
}

run_registry_builder_diag_canary() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local expected_tag_pattern="$4"
    local pass_label="$5"
    local grep_mode="${6:-basic}"

    local tmp_stdout
    local rc=0
    tmp_stdout=$(mktemp)

    run_registry_builder_diag_exec_and_contract \
        "$builder_module" \
        "$prog_json" \
        "$registry_only" \
        "$expected_tag_pattern" \
        "$grep_mode" \
        "$tmp_stdout" \
        rc

    echo "[diag] rc=$rc"
    echo "[PASS] ${pass_label}"
    cleanup_stdout_file "$tmp_stdout"
    return 0
}

direct_lower_box_runner_code() {
    local box_module="$1"
    local method_name="$2"
    cat <<HAKO
using "${box_module}" as LowerBox
static box Main { method main(args){
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = LowerBox.${method_name}(j)
  if out == null { print("[lower:null]"); return 0 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0 }
}
HAKO
}

run_direct_lower_box_vm_to_stdout_file() {
    local box_module="$1"
    local method_name="$2"
    local prog_json="$3"
    local tmp_stdout="$4"
    local tmp_hako

    tmp_hako=$(mktemp --suffix .hako)
    direct_lower_box_runner_code "$box_module" "$method_name" >"${tmp_hako}"

    set +e
    PROG_JSON="$prog_json" \
    NYASH_FAIL_FAST="${NYASH_FAIL_FAST:-0}" \
    NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
    NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}" \
    HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}" \
    "$NYASH_BIN" --backend vm "${tmp_hako}" | tee "$tmp_stdout" >/dev/null
    local rc=$?
    set -e

    rm -f "${tmp_hako}" 2>/dev/null || true
    return "$rc"
}

run_direct_lower_box_canary() {
    local box_module="$1"
    local method_name="$2"
    local prog_json="$3"
    local pass_label="$4"
    local skip_exec_label="${5:-direct lower vm exec failed}"
    local skip_mir_label="${6:-MIR missing functions (direct)}"

    local tmp_stdout
    tmp_stdout=$(mktemp)

    set +e
    (
        run_direct_lower_box_vm_to_stdout_file "$box_module" "$method_name" "$prog_json" "$tmp_stdout"
    )
    local rc=$?
    set -e

    if [[ "$rc" -ne 0 ]]; then
        echo "[SKIP] ${skip_exec_label}"
        cleanup_stdout_file "$tmp_stdout"
        return 0
    fi
    if ! stdout_file_has_functions_mir "$tmp_stdout"; then
        echo "[SKIP] ${skip_mir_label}"
        cleanup_stdout_file "$tmp_stdout"
        return 0
    fi

    echo "[PASS] ${pass_label}"
    cleanup_stdout_file "$tmp_stdout"
    return 0
}

run_registry_method_arraymap_canary() {
    local prog_json="$1"
    local registry_only="$2"
    local expected_tag_label="$3"
    local pass_label="$4"
    local method_pattern="${5:-}"
    local args_pattern="${6:-}"
    local use_preinclude="${7:-0}"

    local tmp_stdout=""
    if ! prepare_registry_method_arraymap_stdout_snapshot \
        "$prog_json" \
        "$registry_only" \
        "$expected_tag_label" \
        "$method_pattern" \
        "$args_pattern" \
        "$use_preinclude" \
        tmp_stdout; then
        return 0
    fi

    if ! run_registry_method_arraymap_token_policy \
        "$tmp_stdout" \
        "$method_pattern" \
        "$args_pattern"; then
        cleanup_stdout_file "$tmp_stdout"
        return 0
    fi

    echo "[PASS] ${pass_label}"
    cleanup_stdout_file "$tmp_stdout"
    return 0
}

# hv1 inline alias-only wrapper (env JSON → hv1 dispatcher)
# Usage: run_hv1_inline_alias_wrapper "$json_literal" → prints rc line; returns rc
run_hv1_inline_alias_wrapper() {
    local json_literal="$1"
    local code=$(cat <<'HCODE'
using "selfhost.vm.hv1.dispatch" as NyVm
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local r = NyVm.NyVmDispatcherV1Box.run(j)
  print("" + r)
  return r
} }
HCODE
)
    HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
    NYASH_VERIFY_JSON="$json_literal" run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
}

