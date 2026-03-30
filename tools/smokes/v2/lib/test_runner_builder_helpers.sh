#!/bin/bash
# test_runner_builder_helpers.sh - split from test_runner.sh
json_artifact_file_is_mir_module() {
    local json_path="$1"
    grep -q '"functions"' "$json_path" 2>/dev/null && grep -q '"blocks"' "$json_path" 2>/dev/null
}

run_direct_mir_json_file_route() {
    local json_path="$1"
    "$NYASH_BIN" --mir-json-file "$json_path" >/dev/null 2>&1
    return $?
}

run_compat_program_json_v0_route() {
    local json_path="$1"
    NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1
    return $?
}

verify_mir_rc() {
    local json_path="$1"
    # 20.36: hakovm を primary 既定へ（Core は診断 fallback）
    local primary="${HAKO_VERIFY_PRIMARY:-hakovm}"
    if [ "$primary" = "hakovm" ]; then
        # For MIR JSON v1, try Hakovm v1 dispatcher first (default ON), fallback to Core on failure.
        # Allow forcing Core with HAKO_VERIFY_V1_FORCE_CORE=1
        if json_artifact_file_is_mir_module "$json_path" && grep -q '"schema_version"' "$json_path" 2>/dev/null; then
          if [ "${HAKO_VERIFY_V1_FORCE_CORE:-0}" = "1" ]; then
            if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
            run_direct_mir_json_file_route "$json_path"; return $?
          fi
          # hv1 直行（main.rs 早期経路）。成功時は rc を採用、失敗時は Core にフォールバック。
          # ただしフロー検証（dispatcher flow / phi 実験）が有効な場合は Core を優先（hv1-inline は最小実装のため）。
          if [ "${HAKO_VERIFY_V1_FORCE_HAKOVM:-0}" != "1" ]; then
            local hv1_rc; hv1_rc=$(verify_v1_inline_file "$json_path" || true)
            if [[ "$hv1_rc" =~ ^-?[0-9]+$ ]]; then
              if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hv1_inline (rust)" >&2; fi
              local n=$hv1_rc; if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi; return $n
            fi
          fi
          # 強制 hv1（-c ラッパ）: NyVmDispatcherV1Box.run を直接呼び出して rc を取得
          if [ "${HAKO_VERIFY_V1_FORCE_HAKOVM:-0}" = "1" ]; then
            local hv1_rc_force; hv1_rc_force=$(verify_v1_inline_file "$json_path" || true)
            if [[ "$hv1_rc_force" =~ ^-?[0-9]+$ ]]; then
              if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hv1_inline (rust)" >&2; fi
              local n=$hv1_rc_force; if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi; return $n
            fi
            return 1
          fi
          # No include+preinclude fallback succeeded → Core にフォールバック
          if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
          run_direct_mir_json_file_route "$json_path"
          return $?
        fi
        # Build a tiny driver to call MiniVmEntryBox.run_min with JSON literal embedded
          if [ ! -f "$json_path" ]; then
            echo "[FAIL] verify_mir_rc: json not found: $json_path" >&2
            return 2
          fi
        # Escape JSON as a single string literal via jq -Rs (preserves newlines)
        local json_literal
        json_literal="$(jq -Rs . < "$json_path")"
        build_and_run_driver_alias() {
          local header="$1"
          local code=$(cat <<HCODE
$header
static box Main { method main(args) {
  local j = __MIR_JSON__;
  local rc = MiniVmEntryBox.run_min(j)
  print(MiniVmEntryBox.int_to_str(rc))
  return rc
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal}"
          HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
        }
        build_and_run_driver_include() {
          local inc_path="$1"
          local code=$(cat <<HCODE
include "$inc_path"
static box Main { method main(args) {
  local j = __MIR_JSON__;
  local rc = MiniVmEntryBox.run_min(j)
  print(MiniVmEntryBox.int_to_str(rc))
  return rc
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal}"
          HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_PREINCLUDE=1 NYASH_RESOLVE_FIX_BRACES=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
        }
        # Try alias header first; fallback to dev-file header; final fallback: include+preinclude
        local out
        out="$(build_and_run_driver_alias 'using selfhost.vm.entry as MiniVmEntryBox')"
        if ! [[ "$out" =~ ^-?[0-9]+$ ]]; then
          out="$(build_and_run_driver_alias 'using "lang/src/vm/boxes/mini_vm_entry.hako" as MiniVmEntryBox')"
        fi
        if ! [[ "$out" =~ ^-?[0-9]+$ ]]; then
          out="$(build_and_run_driver_include 'lang/src/vm/boxes/mini_vm_entry.hako')"
        fi
        if [[ "$out" =~ ^-?[0-9]+$ ]]; then
          local n=$out
          # normalize into [0,255]
          if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi
          return $n
        fi
        # Fallback: core primary when MiniVM resolution is unavailable
        if grep -q '"functions"' "$json_path" 2>/dev/null && grep -q '"blocks"' "$json_path" 2>/dev/null; then
          local json_literal3; json_literal3="$(jq -Rs . < "$json_path")"
          local code=$(cat <<HCODE
include "lang/src/vm/core/dispatcher.hako"
static box Main { method main(args) {
  local j = __MIR_JSON__
  local r = NyVmDispatcher.run(j)
  print("" + r)
  return r
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal3}"
          local tmpwrap="/tmp/hako_core_wrap_$$.hako"
          echo "$code" > "$tmpwrap"
          NYASH_PREINCLUDE=1 run_nyash_vm "$tmpwrap" >/dev/null 2>&1; local r=$?; rm -f "$tmpwrap"; return $r
        fi
        run_compat_program_json_v0_route "$json_path"; return $?
    else
        # Core primary: detect MIR(JSON) vs Program(JSON v0)
        if json_artifact_file_is_mir_module "$json_path"; then
          run_direct_mir_json_file_route "$json_path"; return $?
        fi
        run_compat_program_json_v0_route "$json_path"; return $?
    fi
}

# Program(JSON v0) -> provider emit wrapper (MIR v1 extern -> MIR JSON)
# Keep the provider owner exact while avoiding vm-hako subset debt from
# compiling a temporary .hako wrapper with newbox(MirBuilderBox/hostbridge).
emit_mir_json_via_provider_extern_v1() {
    local prog_json_raw="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local tmp_json
    local prog_json_quoted
    tmp_json=$(mktemp /tmp/nyash_provider_emit_v1.XXXXXX.json)
    prog_json_quoted=$(printf '%s' "$prog_json_raw" | jq -Rs .)

    cat >"$tmp_json" <<JSON
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "blocks": [
        { "id": 0, "instructions": [
          {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "env.mirbuilder.emit"}},
          {"op":"const","dst":1, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": ${prog_json_quoted}}},
          {"op":"mir_call","dst":2, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [1], "effects": [] },
          {"op":"const","dst":3, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_BEGIN]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [3], "effects": [] },
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [2], "effects": [] },
          {"op":"const","dst":4, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_END]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [4], "effects": [] },
          {"op":"const","dst":5, "value": {"type":"i64", "value": 0}},
          {"op":"ret", "value": 5}
        ] }
      ]
    }
  ]
}
JSON

    set +e
    "$NYASH_BIN" --mir-json-file "$tmp_json" 2>>"$builder_stderr" \
        | tee -a "$builder_stdout" \
        | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag'
    local rc=${PIPESTATUS[0]}
    set -e
    rm -f "$tmp_json" 2>/dev/null || true
    return $rc
}

mir_builder_output_missing() {
    local mir_json="${1:-}"
    [ "$mir_json" = "Builder failed" ] || [ -z "$mir_json" ]
}

emit_mir_json_via_min_runner() {
    local prog_json_raw="$1"
    local builder_code_min="$2"
    local builder_stderr="$3"
    local builder_stdout="$4"

    HAKO_MIR_BUILDER_INTERNAL=1 \
    HAKO_MIR_RUNNER_MIN_NO_METHODS=1 \
    HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
    HAKO_ROUTE_HAKOVM=1 \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
    NYASH_USING_AST=1 \
    NYASH_RESOLVE_FIX_BRACES=1 \
    NYASH_DISABLE_NY_COMPILER=1 \
    NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
    NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
    HAKO_BUILDER_PROGRAM_JSON="$prog_json_raw" \
    run_nyash_vm -c "$builder_code_min" 2>"$builder_stderr" \
        | tee "$builder_stdout" \
        | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag'
}

emit_mir_json_via_builder_lanes() {
    local prog_json_raw="$1"
    local builder_code_min="$2"
    local builder_stderr="$3"
    local builder_stdout="$4"
    local mir_json=""

    if [ "${HAKO_PREFER_MIRBUILDER:-0}" != "1" ]; then
        mir_json=$(emit_mir_json_via_min_runner "$prog_json_raw" "$builder_code_min" "$builder_stderr" "$builder_stdout")
    fi

    if mir_builder_output_missing "$mir_json"; then
        mir_json=$(emit_mir_json_via_provider_extern_v1 "$prog_json_raw" "$builder_stderr" "$builder_stdout")
    fi

    printf '%s' "$mir_json"
}

dump_builder_debug_logs() {
    local builder_stdout="$1"
    local builder_stderr="$2"

    if [ "${HAKO_MIR_BUILDER_DEBUG:-0}" != "1" ]; then
        return 0
    fi

    echo "[builder debug] stdout (tail):" >&2
    tail -n 60 "$builder_stdout" >&2 || true
    echo "[builder debug] stderr (tail):" >&2
    tail -n 60 "$builder_stderr" >&2 || true
}

builder_min_runner_code() {
    cat <<'HCODE'
using "hako.mir.builder.internal.runner_min" as BuilderRunnerMinBox
static box Main { method main(args) {
  local prog_json = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if prog_json == null { print("Builder failed"); return 1 }
  local mir_out = BuilderRunnerMinBox.run(prog_json)
  if mir_out == null { print("Builder failed"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + mir_out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
}

builder_module_program_json_runner_code() {
    local builder_module="$1"
    cat <<HAKO
using "${builder_module}" as MirBuilderBox
static box Main {
  method main(args) {
    local program_json = env.get("PROG_JSON")
    if program_json == null { print("[fail:nojson]"); return 1 }
    local out = MirBuilderBox.emit_from_program_json_v0(program_json, null)
    if out == null { print("[fail:builder]"); return 1 }
    print("[MIR_BEGIN]")
    print("" + out)
    print("[MIR_END]")
    return 0
  }
}
HAKO
}

render_builder_module_program_json_runner_file() {
    local builder_module="$1"
    local tmp_hako="$2"

    builder_module_program_json_runner_code "$builder_module" >"$tmp_hako"
}

apply_builder_module_program_json_route_env() {
    local use_registry_defaults="${1:-0}"
    local registry_only="${2:-}"
    local preinclude="${3:-0}"
    local diag_skip_loops="${4:-0}"

    if [ "$preinclude" = "1" ]; then
        export HAKO_PREINCLUDE=1
    fi
    if [ "$diag_skip_loops" = "1" ]; then
        export HAKO_MIR_BUILDER_SKIP_LOOPS=1
    fi
    if [ "$use_registry_defaults" = "1" ]; then
        if [ -n "$registry_only" ]; then
            export HAKO_MIR_BUILDER_REGISTRY_ONLY="$registry_only"
        else
            unset HAKO_MIR_BUILDER_REGISTRY_ONLY
        fi
        export NYASH_USE_NY_COMPILER="${NYASH_USE_NY_COMPILER:-0}"
        export HAKO_MIR_BUILDER_DELEGATE="${HAKO_MIR_BUILDER_DELEGATE:-0}"
        export HAKO_MIR_BUILDER_INTERNAL="${HAKO_MIR_BUILDER_INTERNAL:-1}"
        export HAKO_MIR_BUILDER_REGISTRY="${HAKO_MIR_BUILDER_REGISTRY:-1}"
        export HAKO_MIR_BUILDER_DEBUG="${HAKO_MIR_BUILDER_DEBUG:-1}"
    fi
}

apply_builder_module_program_json_common_env() {
    local prog_json="$1"

    export PROG_JSON="$prog_json"
    export NYASH_FAIL_FAST="${NYASH_FAIL_FAST:-0}"
    export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
    export NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}"
    export HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}"
}

execute_builder_module_program_json_runner() {
    local tmp_hako="$1"

    "$NYASH_BIN" --backend vm "$tmp_hako"
}

cleanup_builder_module_program_json_runner_file() {
    local tmp_hako="$1"

    rm -f "$tmp_hako" 2>/dev/null || true
}

prepare_builder_module_program_json_runner_context() {
    local builder_module="$1"
    local tmp_hako=""

    tmp_hako=$(mktemp --suffix .hako)
    if ! render_builder_module_program_json_runner_file "$builder_module" "$tmp_hako"; then
        cleanup_builder_module_program_json_runner_file "$tmp_hako"
        return 1
    fi

    printf '%s' "$tmp_hako"
}

run_rendered_builder_module_program_json_runner() {
    local tmp_hako="$1"
    local prog_json="$2"
    local use_registry_defaults="${3:-0}"
    local registry_only="${4:-}"
    local preinclude="${5:-0}"
    local diag_skip_loops="${6:-0}"

    (
        apply_builder_module_program_json_route_env \
            "$use_registry_defaults" \
            "$registry_only" \
            "$preinclude" \
            "$diag_skip_loops"
        apply_builder_module_program_json_common_env "$prog_json"
        execute_builder_module_program_json_runner "$tmp_hako"
    )
}

run_program_json_via_builder_module_vm_with_env() {
    local builder_module="$1"
    local prog_json="$2"
    local use_registry_defaults="${3:-0}"
    local registry_only="${4:-}"
    local preinclude="${5:-0}"
    local diag_skip_loops="${6:-0}"
    local tmp_hako=""
    local rc=0

    tmp_hako="$(prepare_builder_module_program_json_runner_context "$builder_module")" || return 1

    run_rendered_builder_module_program_json_runner \
        "$tmp_hako" \
        "$prog_json" \
        "$use_registry_defaults" \
        "$registry_only" \
        "$preinclude" \
        "$diag_skip_loops"
    rc=$?
    cleanup_builder_module_program_json_runner_file "$tmp_hako"
    return $rc
}

run_program_json_via_builder_module_vm() {
    local builder_module="$1"
    local prog_json="$2"
    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 0 "" 0 0
}

run_program_json_via_registry_builder_module_vm() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"
    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 0 0
}

run_program_json_via_registry_builder_module_vm_with_preinclude() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"

    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 1 0
}

run_program_json_via_registry_builder_module_vm_diag() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"

    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 0 1
}

emit_mir_json_via_builder_from_program_json_file() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local prog_json_raw
    local builder_code_min
    local mir_json=""

    prog_json_raw="$(cat "$prog_json_path")"
    builder_code_min="$(builder_min_runner_code)"
    mir_json=$(emit_mir_json_via_builder_lanes "$prog_json_raw" "$builder_code_min" "$builder_stderr" "$builder_stdout")
    dump_builder_debug_logs "$builder_stdout" "$builder_stderr"

    if mir_builder_output_missing "$mir_json"; then
        return 1
    fi
    if ! mir_json_looks_like_v0_module_text "$mir_json"; then
        return 1
    fi

    printf '%s' "$mir_json"
}

run_rust_cli_builder_fallback_for_verify() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local allow_builder_only="${4:-0}"

    if [ "${HAKO_MIR_BUILDER_DEBUG:-0}" = "1" ] && [ -f "$builder_stderr" ]; then
        echo "[builder debug] Hako builder output unusable, falling back to Rust CLI" >&2
        cat "$builder_stderr" >&2
        cp "$builder_stderr" /tmp/builder_last_error.log
    fi

    rm -f "$builder_stderr" "$builder_stdout"
    run_program_json_v0_via_rust_cli_builder "$prog_json_path" "$allow_builder_only"
}

verify_builder_no_fallback_requested() {
    local emit_rc="${1:-0}"
    [ "${HAKO_PRIMARY_NO_FALLBACK:-0}" = "1" ] && [ "$emit_rc" -ne 0 ]
}

mir_json_file_looks_like_v0_module() {
    local mir_json_path="$1"
    grep -q '"functions"' "$mir_json_path" && grep -q '"blocks"' "$mir_json_path"
}

cleanup_verify_builder_logs() {
    local builder_stderr="$1"
    local builder_stdout="$2"
    rm -f "$builder_stderr" "$builder_stdout"
}

apply_verify_program_via_builder_route_env() {
    local verify_builder_only="${1:-0}"
    local prefer_mirbuilder="${2:-0}"
    local primary_no_fallback="${3:-0}"
    local internal_builder="${4:-0}"
    local registry_builder="${5:-0}"

    if [ "$verify_builder_only" = "1" ]; then
        export HAKO_VERIFY_BUILDER_ONLY=1
    fi
    if [ "$prefer_mirbuilder" = "1" ]; then
        export HAKO_PREFER_MIRBUILDER=1
    fi
    if [ "$primary_no_fallback" = "1" ]; then
        export HAKO_PRIMARY_NO_FALLBACK=1
    fi
    if [ "$internal_builder" = "1" ]; then
        export HAKO_MIR_BUILDER_INTERNAL=1
    fi
    if [ "$registry_builder" = "1" ]; then
        export HAKO_MIR_BUILDER_REGISTRY=1
    fi
}

apply_verify_program_via_builder_common_env() {
    export NYASH_ENABLE_USING=1
    export HAKO_ENABLE_USING=1
    export NYASH_USING_AST=1
    export NYASH_RESOLVE_FIX_BRACES=1
    export NYASH_DISABLE_NY_COMPILER=1
    export NYASH_FEATURES=stage3
    export NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1
}

run_built_mir_json_file_via_core_v0() {
    local mir_json_path="$1"
    "$NYASH_BIN" --mir-json-file "$mir_json_path" >/dev/null 2>&1
}

# Program(JSON v0) -> Rust CLI builder fallback
# - with allow_builder_only=1, HAKO_VERIFY_BUILDER_ONLY=1 keeps the old structure-only contract
# - otherwise the helper executes the produced MIR and returns its rc
run_program_json_v0_via_rust_cli_builder() {
    local prog_json_path="$1"
    local allow_builder_only="${2:-0}"
    local tmp_mir="/tmp/ny_builder_conv_$$.json"

    if ! "$NYASH_BIN" --program-json-to-mir "$tmp_mir" --json-file "$prog_json_path" >/dev/null 2>&1; then
        return 1
    fi

    if [ "$allow_builder_only" = "1" ] && [ "${HAKO_VERIFY_BUILDER_ONLY:-0}" = "1" ]; then
        if mir_json_file_looks_like_v0_module "$tmp_mir"; then
            rm -f "$tmp_mir"
            return 0
        fi
        rm -f "$tmp_mir"
        return 1
    fi

    run_built_mir_json_file_via_core_v0 "$tmp_mir"
    local rc=$?
    rm -f "$tmp_mir"
    return $rc
}

mir_json_looks_like_v0_module_text() {
    local mir_json="$1"
    grep -q '"functions"' <<<"$mir_json" && grep -q '"blocks"' <<<"$mir_json"
}

return_normalized_signed_rc() {
    local n="$1"
    if [ "$n" -lt 0 ]; then
        n=$(( (n % 256 + 256) % 256 ))
    else
        n=$(( n % 256 ))
    fi
    return "$n"
}

run_built_mir_json_via_hv1_route() {
    local mir_json="$1"

    if ! grep -q '"schema_version"' <<<"$mir_json" && ! grep -q '"op"\s*:\s*"mir_call"' <<<"$mir_json"; then
        return 2
    fi

    local hv1_rc
    local mir_literal
    mir_literal="$(printf '%s' "$mir_json" | jq -Rs .)"
    hv1_rc=$(run_hv1_inline_alias_wrapper "$mir_literal")
    if [[ "$hv1_rc" =~ ^-?[0-9]+$ ]]; then
        if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hakovm (hako)" >&2; fi
        return_normalized_signed_rc "$hv1_rc"
        return $?
    fi

    return 1
}

mir_json_needs_hako_core_route() {
    local mir_json="$1"
    grep -q '"op"\s*:\s*"newbox"' <<<"$mir_json" || grep -q '"op"\s*:\s*"boxcall"' <<<"$mir_json"
}

hako_core_verify_runner_code() {
    cat <<'HCODE'
include "lang/src/vm/core/dispatcher.hako"
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local r = NyVmDispatcher.run(j)
  print("" + r)
  return r
} }
HCODE
}

run_hako_core_verify_runner() {
    local mir_json="$1"
    local mir_literal
    mir_literal="$(printf '%s' "$mir_json" | jq -Rs .)"
    local code
    code="$(hako_core_verify_runner_code)"
    NYASH_VERIFY_JSON="$mir_literal" NYASH_PREINCLUDE=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
}

run_built_mir_json_via_hako_core_route() {
    local mir_json="$1"

    if ! mir_json_needs_hako_core_route "$mir_json"; then
        return 2
    fi

    local out
    out=$(run_hako_core_verify_runner "$mir_json")
    if [[ "$out" =~ ^-?[0-9]+$ ]]; then
        return_normalized_signed_rc "$out"
        return $?
    fi

    return 1
}

persist_mir_json_text_to_path() {
    local mir_json="$1"
    local mir_json_path="$2"
    printf '%s' "$mir_json" > "$mir_json_path"
}

run_built_mir_json_file_via_core_v0_with_trace() {
    local mir_json_path="$1"
    if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
    run_built_mir_json_file_via_core_v0 "$mir_json_path"
}

cleanup_mir_json_path() {
    local mir_json_path="$1"
    rm -f "$mir_json_path"
}

run_built_mir_json_via_core_v0_route() {
    local mir_json="$1"
    local mir_json_path="$2"

    persist_mir_json_text_to_path "$mir_json" "$mir_json_path"
    run_built_mir_json_file_via_core_v0_with_trace "$mir_json_path"
    local rc=$?
    cleanup_mir_json_path "$mir_json_path"
    return $rc
}

run_built_mir_json_via_builder_only_route() {
    local mir_json="$1"

    if [ "${HAKO_VERIFY_BUILDER_ONLY:-0}" != "1" ]; then
        return 2
    fi
    if mir_json_looks_like_v0_module_text "$mir_json"; then
        return 0
    fi
    return 1
}

verify_primary_requests_core_v0() {
    [ "${HAKO_VERIFY_PRIMARY:-}" = "core" ]
}

run_built_mir_json_via_preferred_vm_routes() {
    local mir_json="$1"

    run_built_mir_json_via_hv1_route "$mir_json"
    local hv1_rc=$?
    if [ "$hv1_rc" != "2" ]; then
        return "$hv1_rc"
    fi

    run_built_mir_json_via_hako_core_route "$mir_json"
    return $?
}

# Built MIR(JSON) result routing after builder lanes are done.
# - caller handles upstream builder fallback / no-fallback policy
# - this helper only routes an already-built MIR payload to builder-only / hv1 / hako-core / core-v0 execution
run_built_mir_json_via_verify_routes() {
    local mir_json="$1"
    local mir_json_path="$2"

    run_built_mir_json_via_builder_only_route "$mir_json"
    local builder_only_rc=$?
    if [ "$builder_only_rc" != "2" ]; then
        return "$builder_only_rc"
    fi

    if verify_primary_requests_core_v0; then
        run_built_mir_json_via_core_v0_route "$mir_json" "$mir_json_path"
        return $?
    fi

    run_built_mir_json_via_preferred_vm_routes "$mir_json"
    local preferred_vm_rc=$?
    if [ "$preferred_vm_rc" != "2" ]; then
        return "$preferred_vm_rc"
    fi

    run_built_mir_json_via_core_v0_route "$mir_json" "$mir_json_path"
    return $?
}

run_verify_builder_emit_rust_cli_fallback() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"

    run_rust_cli_builder_fallback_for_verify "$prog_json_path" "$builder_stderr" "$builder_stdout" 1
}

coerce_verify_builder_emit_result_kind() {
    local emit_rc="${1:-0}"

    if verify_builder_no_fallback_requested "$emit_rc"; then
        printf '%s' "no_fallback_fail"
        return 0
    fi

    if [ "$emit_rc" -ne 0 ]; then
        printf '%s' "emit_fail"
        return 0
    fi

    printf '%s' "emit_ok"
}

run_verify_builder_emit_failure_policy() {
    local result_kind="$1"
    local prog_json_path="$2"
    local builder_stderr="$3"
    local builder_stdout="$4"

    if [ "$result_kind" = "no_fallback_fail" ]; then
        cleanup_verify_builder_logs "$builder_stderr" "$builder_stdout"
        return 1
    fi

    run_verify_builder_emit_rust_cli_fallback "$prog_json_path" "$builder_stderr" "$builder_stdout"
}

run_verify_builder_emit_success_policy() {
    local builder_stderr="$1"
    local builder_stdout="$2"
    local mir_json="$3"
    local mir_json_path="$4"

    cleanup_verify_builder_logs "$builder_stderr" "$builder_stdout"
    run_built_mir_json_via_verify_routes "$mir_json" "$mir_json_path"
}

handle_verify_builder_emit_result() {
    local prog_json_path="$1"
    local mir_json="$2"
    local mir_json_path="$3"
    local builder_stderr="$4"
    local builder_stdout="$5"
    local emit_rc="${6:-0}"
    local result_kind=""

    result_kind="$(coerce_verify_builder_emit_result_kind "$emit_rc")"
    if [ "$result_kind" != "emit_ok" ]; then
        run_verify_builder_emit_failure_policy \
            "$result_kind" \
            "$prog_json_path" \
            "$builder_stderr" \
            "$builder_stdout"
        return $?
    fi

    run_verify_builder_emit_success_policy \
        "$builder_stderr" \
        "$builder_stdout" \
        "$mir_json" \
        "$mir_json_path"
    return $?
}

# New function: verify_program_via_builder_to_core
# Purpose: Program(JSON v0) → MirBuilder(Hako) → MIR(JSON v0) → Core execution
# This is dev-only for testing builder output quality
verify_program_via_builder_to_core() {
    local prog_json_path="$1"
    local mir_json_path="/tmp/builder_output_$$.json"
    local builder_stderr="/tmp/builder_stderr_$$.log"
    local builder_stdout="/tmp/builder_stdout_$$.log"
    local mir_json=""
    local emit_rc=0
    mir_json=$(emit_mir_json_via_builder_from_program_json_file "$prog_json_path" "$builder_stderr" "$builder_stdout")
    emit_rc=$?
    handle_verify_builder_emit_result \
        "$prog_json_path" \
        "$mir_json" \
        "$mir_json_path" \
        "$builder_stderr" \
        "$builder_stdout" \
        "$emit_rc"
    return $?
}

run_verify_program_via_builder_to_core_with_env() {
    local prog_json_path="$1"
    local verify_builder_only="${2:-0}"
    local prefer_mirbuilder="${3:-0}"
    local primary_no_fallback="${4:-0}"
    local internal_builder="${5:-0}"
    local registry_builder="${6:-0}"

    (
        apply_verify_program_via_builder_route_env \
            "$verify_builder_only" \
            "$prefer_mirbuilder" \
            "$primary_no_fallback" \
            "$internal_builder" \
            "$registry_builder"
        apply_verify_program_via_builder_common_env

        verify_program_via_builder_to_core "$prog_json_path"
    )
}

run_verify_program_via_core_default_to_core() {
    local prog_json_path="$1"
    local _unused="${2:-0}"

    (
        export HAKO_VERIFY_PRIMARY=core
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 0 0 0 0
    )
}

run_verify_program_via_preferred_mirbuilder_core_to_core() {
    local prog_json_path="$1"
    local builder_only="${2:-0}"

    (
        export HAKO_VERIFY_PRIMARY=core
        if [ "$builder_only" = "1" ]; then
            run_verify_program_via_builder_to_core_with_env "$prog_json_path" 1 1 0 0 0
            return $?
        fi

        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 1 0 0 0
    )
}

run_verify_program_via_builder_only_to_core() {
    local prog_json_path="$1"
    local _unused="${2:-0}"

    run_verify_program_via_builder_to_core_with_env "$prog_json_path" 1 0 0 0 0
}

run_verify_program_via_internal_builder_to_core() {
    local prog_json_path="$1"
    local verify_primary_core="${2:-0}"

    (
        if [ "$verify_primary_core" = "1" ]; then
            export HAKO_VERIFY_PRIMARY=core
        fi
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 0 0 1 0
    )
}

run_verify_program_via_internal_builder_no_methods_to_core() {
    local prog_json_path="$1"
    local _unused="${2:-0}"

    (
        export HAKO_MIR_RUNNER_MIN_NO_METHODS=1
        run_verify_program_via_internal_builder_to_core "$prog_json_path" 1
    )
}

run_verify_program_via_registry_internal_to_core() {
    local prog_json_path="$1"
    local _unused="${2:-0}"

    (
        export HAKO_VERIFY_PRIMARY=core
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 0 0 1 1
    )
}

run_verify_program_via_preferred_mirbuilder_to_core() {
    local prog_json_path="$1"
    local builder_only="${2:-0}"

    if [ "$builder_only" = "1" ]; then
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 1 1 0 0 0
        return $?
    fi

    run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 1 0 0 0
}

run_verify_program_via_hako_primary_no_fallback_to_core() {
    local prog_json_path="$1"
    local prefer_mirbuilder="${2:-0}"

    if [ "$prefer_mirbuilder" = "1" ]; then
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 1 1 1 0
        return $?
    fi

    run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 0 1 1 0
}
