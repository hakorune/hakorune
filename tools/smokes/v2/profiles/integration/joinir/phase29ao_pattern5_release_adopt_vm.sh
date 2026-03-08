#!/bin/bash
# phase29ao_pattern5_release_adopt_vm.sh - loop_true_early_exit release adopt path smoke (VM)
# legacy compat stem; current semantic entry = loop_true_early_exit_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_true_early_exit_release_adopt_vm.sh" "$@"
