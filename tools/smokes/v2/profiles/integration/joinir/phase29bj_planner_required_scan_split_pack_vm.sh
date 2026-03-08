#!/bin/bash
# phase29bj_planner_required_scan_split_pack_vm.sh - legacy compat stem; current semantic entry = scan_split_planner_required_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/scan_split_planner_required_pack_vm.sh" "$@"
