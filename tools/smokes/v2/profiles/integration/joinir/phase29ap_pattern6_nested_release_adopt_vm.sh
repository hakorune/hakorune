#!/bin/bash
# phase29ap_pattern6_nested_release_adopt_vm.sh - nested_loop_minimal release adopt (VM)
# legacy compat stem; current semantic entry = nested_loop_minimal_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/nested_loop_minimal_release_adopt_vm.sh" "$@"
