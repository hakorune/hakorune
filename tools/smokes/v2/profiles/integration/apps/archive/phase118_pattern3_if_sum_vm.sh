#!/bin/bash
# archived replay stem; current semantic entry = if_phi_join_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/../../joinir/if_phi_join_vm.sh" "$@"
