#!/bin/bash
# ===== SSOT: Smoke Environment Configuration =====
# All environment variable configurations for Phase 131+ smoke tests
#
# This file centralizes environment variable settings to:
# - Prevent "sparrow" bugs (scattered duplicates across scripts)
# - Provide single source of truth (SSOT) for smoke test environment
# - Enable easy modification of defaults across all tests
#
# Usage:
#   source "$(dirname "$0")/../lib/env.sh"
#   setup_smoke_env [mode]  # mode: dev|integration|quick (default: dev)

set -uo pipefail

SMOKE_ENV_SKIP_EXPORTS="${SMOKE_ENV_SKIP_EXPORTS:-0}"

# Optional: skip default exports when this file is sourced for helper functions only.
if [ "${SMOKE_ENV_SKIP_EXPORTS}" != "1" ]; then

# ============================================================================
# JoinIR Development Mode (Phase 131+)
# ============================================================================
# Required for dev-only fixtures that use normalized shadow control flow.
# Without these, LLVM EXE emission may freeze on dev-only patterns.
#
# Default: enabled (all Phase 131+ fixtures require this)
export NYASH_JOINIR_DEV="${NYASH_JOINIR_DEV:-1}"
export HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}"

# ============================================================================
# LLVM Features
# ============================================================================
# LLVM harness execution (Python llvmlite backend)
export NYASH_LLVM_USE_HARNESS="${NYASH_LLVM_USE_HARNESS:-1}"

# LLVM backend selection (crate|python|auto)
export NYASH_LLVM_BACKEND="${NYASH_LLVM_BACKEND:-crate}"

# LLVM verification (0=off, 1=on)
export NYASH_LLVM_VERIFY="${NYASH_LLVM_VERIFY:-0}"
export NYASH_LLVM_VERIFY_IR="${NYASH_LLVM_VERIFY_IR:-0}"

# ============================================================================
# Tmpdir EXDEV Mitigation (for build_llvm.sh)
# ============================================================================
# TARGET_TMPDIR allows smoke scripts to override TMPDIR location.
# This prevents "Invalid cross-device link" errors when /tmp is on different
# filesystem than target directory.
#
# Default: use current directory (workspace target/release/deps)
export TARGET_TMPDIR="${TARGET_TMPDIR:-.}"

# ============================================================================
# Plugin Loader Strategy
# ============================================================================
# NYASH_LOAD_NY_PLUGINS: Load .hako-based plugins from nyash.toml
# Default: 0 (use .so plugins)
export NYASH_LOAD_NY_PLUGINS="${NYASH_LOAD_NY_PLUGINS:-0}"

# NYASH_DISABLE_PLUGINS: Disable all plugins except core builtins
# Default: 0 (plugins enabled)
export NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-0}"

# ============================================================================
# Parser Features
# ============================================================================
# Stage 3 parser features (Phase 251+)
export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"

# Using system (namespace/package imports)
export NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}"
export HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}"

# Using file imports
export NYASH_ALLOW_USING_FILE="${NYASH_ALLOW_USING_FILE:-1}"
export HAKO_ALLOW_USING_FILE="${HAKO_ALLOW_USING_FILE:-1}"

# Using AST mode
export NYASH_USING_AST="${NYASH_USING_AST:-1}"

# ============================================================================
# Debug Features (Optional, Gated by SMOKE_DEBUG)
# ============================================================================
if [ "${SMOKE_DEBUG:-0}" = "1" ]; then
    # Verbose CLI output
    export NYASH_CLI_VERBOSE=1

    # Trace execution
    export HAKO_TRACE_EXECUTION=1
    export HAKO_VERIFY_SHOW_LOGS=1

    # Unlimited debug fuel
    export NYASH_DEBUG_FUEL="unlimited"
else
    # Production defaults (quiet)
    export NYASH_CLI_VERBOSE="${NYASH_CLI_VERBOSE:-0}"
    export HAKO_TRACE_EXECUTION="${HAKO_TRACE_EXECUTION:-0}"
    export HAKO_VERIFY_SHOW_LOGS="${HAKO_VERIFY_SHOW_LOGS:-0}"
    export NYASH_DEBUG_FUEL="${NYASH_DEBUG_FUEL:-10000}"
fi

# ============================================================================
# Silent Tags (Reduce Log Noise)
# ============================================================================
export HAKO_SILENT_TAGS="${HAKO_SILENT_TAGS:-1}"

# ============================================================================
# End of default exports (optional)
# ============================================================================
fi

# ============================================================================
# Mode-Specific Configuration
# ============================================================================
# Allows smoke scripts to request different environment modes
setup_smoke_env() {
    local mode="${1:-dev}"

    case "$mode" in
        dev)
            # Development mode: verbose, unlimited fuel
            export NYASH_CLI_VERBOSE=1
            export NYASH_DEBUG_FUEL="unlimited"
            export NYASH_JOINIR_DEV=1
            export HAKO_JOINIR_STRICT=1
            ;;

        integration)
            # Integration mode: JoinIR dev enabled, moderate verbosity
            export NYASH_JOINIR_DEV=1
            export HAKO_JOINIR_STRICT=1
            export NYASH_CLI_VERBOSE="${NYASH_CLI_VERBOSE:-0}"
            export NYASH_DEBUG_FUEL="${NYASH_DEBUG_FUEL:-10000}"
            ;;

        quick)
            # Quick mode: minimal logging, fast execution
            export NYASH_CLI_VERBOSE=0
            export NYASH_DEBUG_FUEL="10000"
            export HAKO_SILENT_TAGS=1
            ;;

        *)
            echo "[WARN] setup_smoke_env: unknown mode '$mode', using dev defaults"
            setup_smoke_env dev
            ;;
    esac
}

# ============================================================================
# Stage-1 Modules/ModuleRoots Collectors (Shell SSOT)
# ============================================================================
_stage1_trim_ws() {
    local s="$1"
    s="${s#"${s%%[![:space:]]*}"}"
    s="${s%"${s##*[![:space:]]}"}"
    printf "%s" "$s"
}

_stage1_normalize_rel_path() {
    local path="$1"
    local -a parts=()
    local -a out=()
    local part
    IFS='/' read -r -a parts <<< "$path"
    for part in "${parts[@]}"; do
        case "$part" in
            ""|".")
                ;;
            "..")
                if [ "${#out[@]}" -gt 0 ]; then
                    unset 'out[${#out[@]}-1]'
                fi
                ;;
            *)
                out+=("$part")
                ;;
        esac
    done
    local IFS='/'
    printf "%s" "${out[*]}"
}

stage1_find_toml_config() {
    local root="${1:-${NYASH_ROOT:-$(pwd)}}"
    local candidate
    for candidate in "hako.toml" "hakorune.toml" "nyash.toml"; do
        if [ -f "$root/$candidate" ]; then
            echo "$root/$candidate"
            return 0
        fi
    done
    return 1
}

collect_stageb_modules_list() {
    local root="${1:-${NYASH_ROOT:-$(pwd)}}"
    local toml
    toml="$(stage1_find_toml_config "$root")" || {
        echo ""
        return 0
    }

    local -a order=()
    declare -A values=()
    declare -A seen=()

    _stage1_add_module_entry() {
        local key="$1"
        local val="$2"
        if [ -z "$key" ] || [ -z "$val" ]; then
            return
        fi
        if [ -z "${seen[$key]+x}" ]; then
            order+=("$key")
            seen["$key"]=1
        fi
        values["$key"]="$val"
    }

    # 1) [modules.workspace] members first (module exports as base registry)
    local in_workspace=0
    local in_members=0
    local line
    while IFS= read -r line || [ -n "$line" ]; do
        line="${line%%#*}"
        line="$(_stage1_trim_ws "$line")"
        [ -z "$line" ] && continue

        if [[ "$line" == "["* ]]; then
            if [[ "$line" == "[modules.workspace]" ]]; then
                in_workspace=1
                in_members=0
            else
                in_workspace=0
                in_members=0
            fi
            continue
        fi

        if [ "$in_workspace" -ne 1 ]; then
            continue
        fi

        if [[ "$line" == "members"* ]] && [[ "$line" == *"["* ]]; then
            in_members=1
        fi
        if [ "$in_members" -ne 1 ]; then
            continue
        fi

        local member
        while IFS= read -r member; do
            member="${member#\"}"
            member="${member%\"}"
            [ -z "$member" ] && continue

            local module_file
            if [[ "$member" == /* ]]; then
                module_file="$member"
            else
                module_file="$root/$member"
            fi
            [ -f "$module_file" ] || continue

            local module_name
            module_name="$(awk '
                function trim(s) { sub(/^[ \t]+/, "", s); sub(/[ \t]+$/, "", s); return s }
                BEGIN { in_module = 0 }
                {
                    line = $0
                    sub(/#.*/, "", line)
                    line = trim(line)
                    if (line == "") next
                    if (line ~ /^\[/) {
                        if (in_module == 1) exit
                        if (line == "[module]") in_module = 1
                        next
                    }
                    if (in_module == 1 && line ~ /^name[ \t]*=/) {
                        split(line, a, "=")
                        val = trim(a[2])
                        gsub(/^"/, "", val)
                        gsub(/"$/, "", val)
                        print val
                        exit
                    }
                }
            ' "$module_file")"
            [ -z "$module_name" ] && continue

            local module_dir
            module_dir="$(dirname "$module_file")"
            local export_pair
            while IFS= read -r export_pair; do
                [ -z "$export_pair" ] && continue
                local export_key="${export_pair%%$'\t'*}"
                local export_val="${export_pair#*$'\t'}"
                [ -z "$export_key" ] && continue
                [ -z "$export_val" ] && continue

                local full_name="${module_name}.${export_key}"
                local resolved="$export_val"
                if [[ "$resolved" != /* ]]; then
                    resolved="$module_dir/$resolved"
                    if [[ "$resolved" == "$root/"* ]]; then
                        resolved="${resolved#$root/}"
                    fi
                    resolved="$(_stage1_normalize_rel_path "$resolved")"
                fi
                _stage1_add_module_entry "$full_name" "$resolved"
            done < <(awk '
                function trim(s) { sub(/^[ \t]+/, "", s); sub(/[ \t]+$/, "", s); return s }
                BEGIN { in_exports = 0; export_prefix = "" }
                {
                    line = $0
                    sub(/#.*/, "", line)
                    line = trim(line)
                    if (line == "") next
                    if (line ~ /^\[/) {
                        in_exports = 0
                        export_prefix = ""
                        if (line == "[exports]") {
                            in_exports = 1
                            next
                        }
                        if (line ~ /^\[exports\.[^]]+\]$/) {
                            in_exports = 1
                            export_prefix = line
                            sub(/^\[exports\./, "", export_prefix)
                            sub(/\]$/, "", export_prefix)
                            next
                        }
                        next
                    }
                    if (in_exports == 1 && index(line, "=") > 0) {
                        eq = index(line, "=")
                        key = trim(substr(line, 1, eq - 1))
                        val = trim(substr(line, eq + 1))
                        if (val !~ /^".*"$/) next
                        gsub(/^"/, "", key)
                        gsub(/"$/, "", key)
                        gsub(/^"/, "", val)
                        gsub(/"$/, "", val)
                        if (export_prefix != "") {
                            key = export_prefix "." key
                        }
                        if (key != "" && val != "") print key "\t" val
                    }
                }
            ' "$module_file")
        done < <(printf '%s\n' "$line" | grep -oE '"[^"]+"' || true)

        if [[ "$line" == *"]"* ]]; then
            in_members=0
        fi
    done < "$toml"

    # 2) [modules] exact entries override workspace entries on key collision.
    local in_modules=0
    while IFS= read -r line || [ -n "$line" ]; do
        line="${line%%#*}"
        line="$(_stage1_trim_ws "$line")"
        [ -z "$line" ] && continue
        if [[ "$line" == "["* ]]; then
            if [[ "$line" == "[modules]" ]]; then
                in_modules=1
            else
                in_modules=0
            fi
            continue
        fi
        if [ "$in_modules" -ne 1 ]; then
            continue
        fi
        if [[ "$line" == *"="* ]]; then
            local key="${line%%=*}"
            local val="${line#*=}"
            key="$(_stage1_trim_ws "$key")"
            val="$(_stage1_trim_ws "$val")"
            key="${key#\"}"
            key="${key%\"}"
            val="${val#\"}"
            val="${val%\"}"
            _stage1_add_module_entry "$key" "$val"
        fi
    done < "$toml"

    # Add well-known aliases required by Stage-1 CLI if absent.
    local pair
    for pair in \
        "lang.compiler.entry.using_resolver_box=lang/src/compiler/entry/using_resolver_box.hako" \
        "selfhost.shared.host_bridge.codegen_bridge=lang/src/shared/host_bridge/codegen_bridge_box.hako"
    do
        local key="${pair%%=*}"
        local val="${pair#*=}"
        if [ -z "${values[$key]+x}" ]; then
            _stage1_add_module_entry "$key" "$val"
        fi
    done

    if [ "${#order[@]}" -eq 0 ]; then
        echo ""
        return 0
    fi

    local out=""
    local key
    for key in "${order[@]}"; do
        local pair="${key}=${values[$key]}"
        if [ -z "$out" ]; then
            out="$pair"
        else
            out="${out}|||${pair}"
        fi
    done
    echo "$out"
}

collect_stageb_module_roots_list() {
    local root="${1:-${NYASH_ROOT:-$(pwd)}}"
    local toml
    toml="$(stage1_find_toml_config "$root")" || {
        echo ""
        return 0
    }
    local in_roots=0
    local -a sortable=()
    declare -A seen=()
    local line
    while IFS= read -r line || [ -n "$line" ]; do
        line="${line%%#*}"
        line="$(_stage1_trim_ws "$line")"
        [ -z "$line" ] && continue
        if [[ "$line" == "["* ]]; then
            if [[ "$line" == "[module_roots]" ]]; then
                in_roots=1
            else
                in_roots=0
            fi
            continue
        fi
        if [ "$in_roots" -ne 1 ]; then
            continue
        fi
        if [[ "$line" == *"="* ]]; then
            local key="${line%%=*}"
            local val="${line#*=}"
            key="$(_stage1_trim_ws "$key")"
            val="$(_stage1_trim_ws "$val")"
            key="${key#\"}"
            key="${key%\"}"
            val="${val#\"}"
            val="${val%\"}"
            if [ -n "$key" ] && [ -n "$val" ] && [ -z "${seen[$key]+x}" ]; then
                seen["$key"]=1
                sortable+=("${#key}"$'\t'"${key}=${val}")
            fi
        fi
    done < "$toml"

    if [ "${#sortable[@]}" -eq 0 ]; then
        echo ""
        return 0
    fi

    local sorted
    sorted="$(printf "%s\n" "${sortable[@]}" | sort -nr | cut -f2-)"
    local -a entries=()
    local kv
    while IFS= read -r kv || [ -n "$kv" ]; do
        [ -z "$kv" ] && continue
        entries+=("$kv")
    done <<< "$sorted"

    local out=""
    for kv in "${entries[@]}"; do
        if [ -z "$out" ]; then
            out="$kv"
        else
            out="${out}|||${kv}"
        fi
    done
    echo "$out"
}

# ============================================================================
# Hermetic VM Runner (Planner-first gates)
# ============================================================================
# Ensures no developer-local debug/trace envs leak into stdout/stderr.
run_hermetic_vm() {
    env \
        NYASH_DISABLE_PLUGINS=1 \
        NYASH_CLI_VERBOSE=0 \
        HAKO_JOINIR_STRICT=1 \
        HAKO_JOINIR_PLANNER_REQUIRED=1 \
        HAKO_JOINIR_DEBUG=0 \
        HAKO_DEBUG=0 \
        HAKO_SHOW_CALL_LOGS=0 \
        HAKO_SILENT_TAGS=1 \
        "$@"
}

# ============================================================================
# Validation Helpers
# ============================================================================
validate_env_setup() {
    local warnings=0

    # Check JoinIR dev mode (required for Phase 131+)
    if [ "${NYASH_JOINIR_DEV:-0}" != "1" ]; then
        echo "[WARN] NYASH_JOINIR_DEV not enabled (may fail on Phase 131+ fixtures)"
        warnings=$((warnings + 1))
    fi

    if [ "${HAKO_JOINIR_STRICT:-0}" != "1" ]; then
        echo "[WARN] HAKO_JOINIR_STRICT not enabled (may fail on Phase 131+ fixtures)"
        warnings=$((warnings + 1))
    fi

    # Check LLVM harness (required for LLVM EXE smokes)
    if [ "${NYASH_LLVM_USE_HARNESS:-0}" != "1" ]; then
        echo "[INFO] NYASH_LLVM_USE_HARNESS not enabled (LLVM EXE tests will be skipped)"
    fi

    return $warnings
}

# Show active environment configuration
show_smoke_env() {
    cat <<EOF
[INFO] Smoke Environment Configuration (SSOT: lib/env.sh)
  JoinIR Dev:          NYASH_JOINIR_DEV=${NYASH_JOINIR_DEV}
  JoinIR Strict:       HAKO_JOINIR_STRICT=${HAKO_JOINIR_STRICT}
  LLVM Harness:        NYASH_LLVM_USE_HARNESS=${NYASH_LLVM_USE_HARNESS}
  LLVM Backend:        NYASH_LLVM_BACKEND=${NYASH_LLVM_BACKEND}
  Plugins:             NYASH_DISABLE_PLUGINS=${NYASH_DISABLE_PLUGINS}
  Debug Fuel:          NYASH_DEBUG_FUEL=${NYASH_DEBUG_FUEL}
  Verbose:             NYASH_CLI_VERBOSE=${NYASH_CLI_VERBOSE}
EOF
}

# Auto-validate on source (optional, gated by flag)
if [ "${SMOKE_ENV_VALIDATE:-0}" = "1" ]; then
    validate_env_setup
fi
