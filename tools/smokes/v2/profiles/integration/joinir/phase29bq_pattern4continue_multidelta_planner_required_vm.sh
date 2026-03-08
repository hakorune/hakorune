#!/bin/bash
# phase29bq_pattern4continue_multidelta_planner_required_vm.sh - legacy compat stem; current semantic entry = loop_continue_only_multidelta_planner_required_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_continue_only_multidelta_planner_required_vm.sh" "$@"
