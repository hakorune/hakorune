#!/bin/bash
# phase29ao_pattern7_strict_shadow_vm.sh - legacy compat stem; current semantic entry = split_scan_strict_shadow_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/split_scan_strict_shadow_vm.sh" "$@"
