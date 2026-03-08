#!/bin/bash
# phase29av_flowbox_tags_gate_vm.sh - legacy compat stem; current semantic entry = flowbox_tags_gate_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/flowbox_tags_gate_vm.sh" "$@"
