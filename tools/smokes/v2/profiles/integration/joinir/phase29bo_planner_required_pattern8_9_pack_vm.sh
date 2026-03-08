#!/bin/bash
# phase29bo_planner_required_pattern8_9_pack_vm.sh - legacy compat stem; current semantic entry = bool_predicate_accum_planner_required_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/bool_predicate_accum_planner_required_pack_vm.sh" "$@"
