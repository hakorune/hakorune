#!/bin/bash
# phase29ao_pattern2_release_adopt_vm.sh - loop_break release adopt path smoke (VM)
# legacy compat stem; current semantic entry = loop_break_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_break_release_adopt_vm.sh" "$@"
