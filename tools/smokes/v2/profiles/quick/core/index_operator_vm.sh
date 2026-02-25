#!/bin/bash
# index_operator_vm.sh - Array/Map indexing support tests

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

# Phase 287 P3: Top-level local spec pending (REPL/file mode distinction)
echo "[SKIP:spec] Top-level local declarations - spec pending"
exit 0

test_index_array_read() {
    local output
    output=$(NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm -c 'local arr = [1, 2, 3]; print(arr[0]);' 2>&1)
    check_exact "1" "$output" "index_array_read"
}

test_index_array_write() {
    local output
    output=$(NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm -c 'local arr = [1, 2]; arr[1] = 9; print(arr[1]);' 2>&1)
    check_exact "9" "$output" "index_array_write"
}

test_index_map_read_write() {
    local output
    output=$(NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm -c 'local m = { "a": 1 }; m["b"] = 7; print(m["b"]);' 2>&1)
    check_exact "7" "$output" "index_map_rw"
}

test_index_string_unsupported() {
    local output
    local status
    output=$(NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm -c 'local s = "hey"; print(s[0]);' 2>&1) && status=0 || status=$?
    if [ "$status" -eq 0 ]; then
        echo "[FAIL] index_string_unsupported: expected failure" >&2
        return 1
    fi
    # Expect builder to fail-fast with explicit diagnostic
    check_regex "index operator is only supported" "$output" "index_string_unsupported"
}

run_test "index_array_read" test_index_array_read
run_test "index_array_write" test_index_array_write
run_test "index_map_rw" test_index_map_read_write
run_test "index_string_unsupported" test_index_string_unsupported
