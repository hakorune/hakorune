#!/bin/bash
# phase29aq_stdlib_pack_vm.sh - legacy compat stem; current semantic entry = stdlib_string_pack_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/stdlib_string_pack_vm.sh" "$@"
