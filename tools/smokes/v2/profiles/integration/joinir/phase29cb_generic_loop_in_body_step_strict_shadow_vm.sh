#!/bin/bash
# phase29cb_generic_loop_in_body_step_strict_shadow_vm.sh - legacy compat stem; current semantic entry = generic_loop_in_body_step_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/generic_loop_in_body_step_strict_shadow_vm.sh" "$@"
