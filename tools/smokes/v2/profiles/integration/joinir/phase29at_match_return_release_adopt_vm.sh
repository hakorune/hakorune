#!/bin/bash
# phase29at_match_return_release_adopt_vm.sh - legacy compat stem; current semantic entry = match_return_release_adopt_vm.sh
LEGACY_STEM_OVERRIDE="$(basename "$0" .sh)" \
  exec bash "$(dirname "$0")/match_return_release_adopt_vm.sh" "$@"
