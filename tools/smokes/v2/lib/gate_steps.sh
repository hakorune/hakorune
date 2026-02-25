#!/usr/bin/env bash
# gate_steps.sh - common helpers for matrix/lane gate scripts

run_gate_step() {
    local gate_name="$1"
    local cmd="$2"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "${gate_name}: step failed: ${cmd}"
        exit 1
    fi
}
