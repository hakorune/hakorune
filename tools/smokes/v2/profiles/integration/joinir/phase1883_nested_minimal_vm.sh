#!/bin/bash
# phase1883_nested_minimal_vm.sh - historical compat wrapper for Phase 188.3 nested-loop smoke
# Current semantic strict-shadow gate: nested_loop_minimal_strict_shadow_vm.sh

exec "$(dirname "$0")/nested_loop_minimal_strict_shadow_vm.sh"
