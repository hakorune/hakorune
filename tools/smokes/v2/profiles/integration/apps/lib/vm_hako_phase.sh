#!/bin/bash
# vm_hako_phase.sh - shared vm-hako phase resolver (SSOT from Rust runner constant)

resolve_vm_hako_phase() {
    local lib_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local root="${NYASH_ROOT:-$(cd "$lib_dir/../../../../../../.." && pwd)}"
    local src="$root/src/runner/modes/vm_hako.rs"
    if [ ! -f "$src" ]; then
        echo ""; return 1
    fi
    local phase
    phase=$(sed -n 's/^const VM_HAKO_PHASE: &str = "\([^"]\+\)";.*/\1/p' "$src" | head -n 1)
    if [ -z "$phase" ]; then
        echo ""; return 1
    fi
    echo "$phase"
}
