#!/bin/bash
# archived replay stem; current semantic entry = loop_break_plan_subset_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/../../joinir/loop_break_plan_subset_vm.sh" "$@"
