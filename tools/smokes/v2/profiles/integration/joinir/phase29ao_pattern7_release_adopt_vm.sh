#!/bin/bash
# phase29ao_pattern7_release_adopt_vm.sh - split_scan release adopt path smoke (VM)
# legacy compat stem; current semantic entry = split_scan_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/split_scan_release_adopt_vm.sh" "$@"
