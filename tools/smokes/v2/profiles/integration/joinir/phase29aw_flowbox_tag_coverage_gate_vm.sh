#!/bin/bash
# phase29aw_flowbox_tag_coverage_gate_vm.sh - legacy compat stem; current semantic entry = flowbox_tag_coverage_gate_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/flowbox_tag_coverage_gate_vm.sh" "$@"
