#!/bin/bash
# phase29ao_pattern1_subset_reject_extra_stmt_vm.sh - legacy compat stem; current semantic entry = loop_simple_while_subset_reject_extra_stmt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_simple_while_subset_reject_extra_stmt_vm.sh" "$@"
