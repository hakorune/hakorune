#!/bin/bash
# phase29ao_pattern3_release_adopt_vm.sh - if_phi_join release adopt path smoke (VM)
# legacy compat stem; current semantic entry = if_phi_join_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/if_phi_join_release_adopt_vm.sh" "$@"
