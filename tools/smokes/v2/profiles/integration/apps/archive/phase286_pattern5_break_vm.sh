#!/bin/bash
# archived replay stem; current semantic entry = loop_true_early_exit_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/../../joinir/loop_true_early_exit_vm.sh" "$@"
