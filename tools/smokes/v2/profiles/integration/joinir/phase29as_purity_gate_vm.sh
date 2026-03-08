#!/bin/bash
# phase29as_purity_gate_vm.sh - legacy compat stem; current semantic entry = joinir_purity_gate_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/joinir_purity_gate_vm.sh" "$@"
