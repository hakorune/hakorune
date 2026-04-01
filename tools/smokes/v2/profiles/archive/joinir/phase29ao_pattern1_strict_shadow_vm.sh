#!/bin/bash
# phase29ao_pattern1_strict_shadow_vm.sh - legacy compat stem; current semantic entry = loop_simple_while_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_simple_while_strict_shadow_vm.sh" "$@"
