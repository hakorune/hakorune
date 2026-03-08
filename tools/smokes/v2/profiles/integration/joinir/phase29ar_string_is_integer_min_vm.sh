#!/bin/bash
# phase29ar_string_is_integer_min_vm.sh - legacy compat stem; current semantic entry = string_is_integer_strict_reject_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/string_is_integer_strict_reject_vm.sh" "$@"
