#!/bin/bash
# phase29ao_pattern6_release_adopt_vm.sh - scan_with_init release adopt path smoke (VM)
# legacy compat stem; current semantic entry = scan_with_init_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/scan_with_init_release_adopt_vm.sh" "$@"
