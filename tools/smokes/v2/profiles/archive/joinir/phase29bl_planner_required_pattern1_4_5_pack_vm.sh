#!/bin/bash
# phase29bl_planner_required_pattern1_4_5_pack_vm.sh - legacy compat stem; current semantic entry = core_loop_routes_planner_required_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/core_loop_routes_planner_required_pack_vm.sh" "$@"
