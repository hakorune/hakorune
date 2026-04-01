#!/bin/bash
# phase29bi_planner_required_pattern2_pack_vm.sh - legacy compat stem; current semantic entry = loop_break_planner_required_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/loop_break_planner_required_pack_vm.sh" "$@"
