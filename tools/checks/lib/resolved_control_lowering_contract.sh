#!/usr/bin/env bash

# Stable aggregate for family-specific canonical control-lowering contracts.
# The public authority guard sources only this file so new families do not
# grow its already bounded top-level entry.

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_control_flow_contract.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_binding_ssa_contract.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_if_lowering_contract.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_loop_lowering_contract.sh"

guard_resolved_control_lowering_contract() {
  guard_resolved_control_flow_contract "$1" "$2"
  guard_resolved_binding_ssa_contract "$1" "$2"
  guard_resolved_if_lowering_contract "$1" "$2"
  guard_resolved_loop_lowering_contract "$1" "$2"
}
