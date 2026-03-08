#!/bin/bash
# phase29ae_pattern7_scan_split_pack_vm.sh - legacy compat stem; current semantic entry = split_scan_regression_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/split_scan_regression_pack_vm.sh" "$@"
