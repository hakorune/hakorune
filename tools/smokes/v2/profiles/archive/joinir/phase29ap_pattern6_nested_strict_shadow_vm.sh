#!/bin/bash
# phase29ap_pattern6_nested_strict_shadow_vm.sh - legacy compat stem; current semantic entry = nested_loop_minimal_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/nested_loop_minimal_strict_shadow_vm.sh" "$@"
