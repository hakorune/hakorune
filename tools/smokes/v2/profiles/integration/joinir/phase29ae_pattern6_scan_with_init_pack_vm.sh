#!/bin/bash
# phase29ae_pattern6_scan_with_init_pack_vm.sh - legacy compat stem; current semantic entry = scan_with_init_regression_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/scan_with_init_regression_pack_vm.sh" "$@"
