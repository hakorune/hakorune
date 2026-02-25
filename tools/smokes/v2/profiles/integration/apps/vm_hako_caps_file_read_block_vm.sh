#!/bin/bash
# Compatibility wrapper:
# - use vm_hako_caps_file_read_ported_vm.sh as canonical smoke.

set -euo pipefail

exec "$(dirname "$0")/vm_hako_caps_file_read_ported_vm.sh" "$@"
