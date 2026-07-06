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

guard_find_mimalloc_library() {
    local tag="$1"
    guard_require_command "$tag" ldconfig

    local path
    path="$(ldconfig -p 2>/dev/null | awk '/libmimalloc\.so\.2[[:space:]]/ { print $NF; exit }')"
    if [[ -z "$path" ]]; then
        guard_fail "$tag" "libmimalloc.so.2 not found; pass an explicit library path"
    fi
    if [[ ! -f "$path" ]]; then
        guard_fail "$tag" "libmimalloc.so.2 path does not exist: $path"
    fi
    printf '%s\n' "$path"
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

guard_expect_fixed_in_file() {
    local tag="$1"
    local pattern="$2"
    local file="$3"
    local msg="$4"
    if ! rg -F -q -- "$pattern" "$file"; then
        guard_fail "$tag" "$msg"
    fi
}

guard_require_design_stop_pause_contract() {
    local tag="$1"
    local root_dir="$2"
    local blocker_token="$3"
    local contract_file="$4"

    local blocker_class=""
    local entry
    local target
    local pattern
    while IFS= read -r entry; do
        [[ -z "$entry" ]] && continue
        [[ "$entry" = \#* ]] && continue
        if [[ "$entry" == blocker_token_contains=* ]]; then
            blocker_class="${entry#blocker_token_contains=}"
            if [[ -z "$blocker_class" ]]; then
                guard_fail "$tag" "design-stop contract missing blocker token class"
            fi
            break
        fi
    done < "$contract_file"

    if [[ -z "$blocker_class" ]]; then
        guard_fail "$tag" "design-stop contract missing blocker token class"
    fi

    if [[ "$blocker_token" != *"$blocker_class"* ]]; then
        return 0
    fi

    while IFS= read -r entry; do
        [[ -z "$entry" ]] && continue
        [[ "$entry" = \#* ]] && continue
        if [[ "$entry" == blocker_token_contains=* ]]; then
            continue
        fi
        IFS='|' read -r target pattern <<<"$entry"
        if [[ -z "$target" || -z "$pattern" ]]; then
            guard_fail "$tag" "invalid design-stop contract entry: $entry"
        fi
        if [[ "$target" = /* ]]; then
            guard_fail "$tag" "design-stop pause target must be repo-relative: $target"
        fi
        if [[ ! -f "$root_dir/$target" ]]; then
            guard_fail "$tag" "design-stop pause target points to missing file: $target"
        fi
        if ! rg -Fq "$pattern" "$root_dir/$target"; then
            guard_fail "$tag" "$(realpath --relative-to="$root_dir" "$root_dir/$target") missing CURRENT_STATE token: $pattern"
        fi
    done < "$contract_file"

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

guard_worktree_clean_for_cache() {
    local root_dir="$1"

    git -C "$root_dir" diff --quiet --ignore-submodules -- &&
        git -C "$root_dir" diff --cached --quiet --ignore-submodules -- &&
        [[ -z "$(git -C "$root_dir" ls-files --others --exclude-standard)" ]]
}

guard_cached_run() {
    local tag="$1"
    shift

    if [[ "$#" -eq 0 ]]; then
        guard_fail "$tag" "guard_cached_run requires a command"
    fi

    if [[ "${HAKO_GUARD_RESULT_CACHE:-1}" == "0" ]]; then
        "$@"
        return $?
    fi

    local root_dir
    root_dir="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
    local dirty_digest="clean"
    if ! guard_worktree_clean_for_cache "$root_dir"; then
        if [[ "${HAKO_GUARD_RESULT_CACHE_ALLOW_DIRTY:-0}" != "1" ]]; then
            "$@"
            return $?
        fi
        dirty_digest="$({
            git -C "$root_dir" status --porcelain=v1
            git -C "$root_dir" diff --binary
            git -C "$root_dir" diff --cached --binary
        } | sha256sum | awk '{ print $1 }')"
    fi

    if [[ "$dirty_digest" != "clean" && -n "$(git -C "$root_dir" ls-files --others --exclude-standard)" ]]; then
        "$@"
        return $?
    fi

    local head
    head="$(git -C "$root_dir" rev-parse HEAD 2>/dev/null || printf 'no-head')"

    local command_digest
    command_digest="$(printf '%s\0' "$@" | sha256sum | awk '{ print $1 }')"

    local script_digest="no-script"
    if [[ -f "$1" ]]; then
        script_digest="$(sha256sum "$1" | awk '{ print $1 }')"
    fi

    local bin_fingerprint="no-bin"
    if [[ -x "$root_dir/target/release/hakorune" ]]; then
        bin_fingerprint="$(stat -c '%n:%s:%Y' "$root_dir/target/release/hakorune" 2>/dev/null || printf 'hakorune')"
    fi

    local env_digest
    env_digest="$(env | LC_ALL=C sort | awk '/^(HAKO|NYASH)_/' | sha256sum | awk '{ print $1 }')"

    local key
    key="$(printf '%s\n%s\n%s\n%s\n%s\n%s\n' \
        "$head" "$dirty_digest" "$command_digest" "$script_digest" "$bin_fingerprint" "$env_digest" \
        | sha256sum | awk '{ print $1 }')"

    local cache_root cache_out cache_meta
    cache_root="${HAKO_GUARD_RESULT_CACHE_DIR:-$root_dir/target/guard-result-cache/v1}"
    cache_out="$cache_root/$key.out"
    cache_meta="$cache_root/$key.meta"

    if [[ -s "$cache_out" && -s "$cache_meta" ]]; then
        cat "$cache_out"
        return 0
    fi

    mkdir -p "$cache_root"
    local tmp_out tmp_err rc
    tmp_out="$(mktemp "$cache_root/${key}.out.XXXXXX")"
    tmp_err="$(mktemp "$cache_root/${key}.err.XXXXXX")"

    set +e
    "$@" >"$tmp_out" 2>"$tmp_err"
    rc=$?
    set -e

    if [[ "$rc" -eq 0 ]]; then
        cat >"$cache_meta.$$" <<EOF
head=$head
dirty_digest=$dirty_digest
command_digest=$command_digest
script_digest=$script_digest
bin_fingerprint=$bin_fingerprint
env_digest=$env_digest
EOF
        mv "$tmp_out" "$cache_out"
        mv "$cache_meta.$$" "$cache_meta"
        rm -f "$tmp_err"
        cat "$cache_out"
        return 0
    fi

    cat "$tmp_out"
    cat "$tmp_err" >&2
    rm -f "$tmp_out" "$tmp_err"
    return "$rc"
}
