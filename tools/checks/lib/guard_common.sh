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
    if ! rg -q -- "$pattern" "$file"; then
        guard_fail "$tag" "$msg"
    fi
}

guard_require_docs_slim_no_move_stop_line() {
    local tag="$1"
    local card="$2"

    guard_expect_in_file "$tag" "Do not move numbered cards in this row" "$card" "card must keep no-move stop-line"
}

guard_require_docs_slim_card_metadata() {
    local tag="$1"
    local card="$2"
    local archive_policy="$3"
    local check_index="$4"
    local self_script="$5"
    local doc_tag="$6"
    local phase_phrase="$7"

    guard_expect_in_file "$tag" "$doc_tag" "$card" "$doc_tag card must exist"
    guard_require_docs_slim_no_move_stop_line "$tag" "$card"
    guard_expect_in_file "$tag" "$phase_phrase" "$archive_policy" "archive policy must record $doc_tag"
    guard_expect_in_file "$tag" "$self_script" "$check_index" "check index must list $doc_tag guard"
}

guard_require_no_phase_card_resolver_leak() {
    local tag="$1"
    local dev_gate="$2"
    local allocator_gate="$3"

    local leak
    leak="$(mktemp "/tmp/${tag}.phase-card-leak.XXXXXX")"
    if rg -n 'phase_card_paths|guard_require_phase293x_card' "$dev_gate" "$allocator_gate" >"$leak" 2>&1; then
        echo "[${tag}] ERROR: phase-card resolver helper must not be wired into dev_gate or allocator-wide directly" >&2
        cat "$leak" >&2
        rm -f "$leak"
        exit 1
    fi
    rm -f "$leak"
}

guard_timeout_run() {
    local tag="$1"
    local seconds="$2"
    local out="$3"
    local err="$4"
    shift 4

    guard_require_command "$tag" timeout

    local rc
    if timeout --kill-after=2s "$seconds" "$@" >"$out" 2>"$err"; then
        return 0
    else
        rc=$?
    fi

    if [[ "$rc" == "124" || "$rc" == "137" ]]; then
        echo "[${tag}] ERROR: command timed out after ${seconds}: $*" >&2
    fi
    return "$rc"
}
