#!/bin/bash
# phase29ao_pattern6_strict_shadow_vm.sh - legacy compat stem; current semantic entry = scan_with_init_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/scan_with_init_strict_shadow_vm.sh" "$@"
