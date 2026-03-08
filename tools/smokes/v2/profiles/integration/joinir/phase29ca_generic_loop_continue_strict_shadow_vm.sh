#!/bin/bash
# phase29ca_generic_loop_continue_strict_shadow_vm.sh - legacy compat stem; current semantic entry = generic_loop_continue_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/generic_loop_continue_strict_shadow_vm.sh" "$@"
