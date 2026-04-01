#!/bin/bash
# phase29ap_pattern4_continue_min_vm.sh - loop_continue_only via plan routing (VM)
# legacy compat stem; current semantic entry = loop_continue_only_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_continue_only_vm.sh" "$@"
