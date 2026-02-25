#!/usr/bin/env bash
# guard_common.sh - shared assertions for check guards

guard_fail() {
    local tag="$1"
    local msg="$2"
    echo "[${tag}] ERROR: ${msg}" >&2
    exit 1
}

guard_require_command() {
    local tag="$1"
    local cmd="$2"
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "[${tag}] ERROR: ${cmd} is required" >&2
        exit 2
    fi
}

guard_require_files() {
    local tag="$1"
    shift
    local path
    for path in "$@"; do
        if [[ ! -f "$path" ]]; then
            guard_fail "$tag" "required file missing: $path"
        fi
    done
}

guard_require_exec_files() {
    local tag="$1"
    shift
    local path
    for path in "$@"; do
        if [[ ! -x "$path" ]]; then
            guard_fail "$tag" "file missing or not executable: $path"
        fi
    done
}

guard_expect_in_file() {
    local tag="$1"
    local pattern="$2"
    local file="$3"
    local msg="$4"
    if ! rg -q "$pattern" "$file"; then
        guard_fail "$tag" "$msg"
    fi
}
