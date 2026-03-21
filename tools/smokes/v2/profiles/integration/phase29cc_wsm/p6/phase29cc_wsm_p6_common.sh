#!/bin/bash
# phase29cc_wsm_p6_common.sh
# Shared route-policy/default-noop helpers for WSM-P6 smoke scripts.

set -euo pipefail
_phase29cc_wsm_p6_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$_phase29cc_wsm_p6_dir/../p5/phase29cc_wsm_p5_route_trace_common.sh"
