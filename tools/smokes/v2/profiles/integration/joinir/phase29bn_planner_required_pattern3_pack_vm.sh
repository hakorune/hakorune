#!/bin/bash
# phase29bn_planner_required_pattern3_pack_vm.sh - legacy compat stem; current semantic entry = if_phi_join_planner_required_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/if_phi_join_planner_required_pack_vm.sh" "$@"
