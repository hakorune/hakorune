#!/bin/bash
# phase29cc_wsm_p7_common.sh
# Shared default-route and compat-retention helpers for WSM-P7 smoke scripts.

set -euo pipefail
_phase29cc_wsm_p7_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$_phase29cc_wsm_p7_dir/../p6/phase29cc_wsm_p6_common.sh"
