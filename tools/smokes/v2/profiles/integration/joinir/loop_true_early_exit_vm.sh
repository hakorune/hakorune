#!/bin/bash
# current semantic wrapper; archive-fixed keep even if repo grep is self-only
exec bash "$(dirname "$0")/../apps/archive/phase286_pattern5_break_vm.sh" "$@"
