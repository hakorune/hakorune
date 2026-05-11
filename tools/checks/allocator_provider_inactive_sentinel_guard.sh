#!/usr/bin/env bash
set -euo pipefail

TAG="[allocator-provider-inactive-sentinel]"
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

fail() {
  echo "${TAG} error: $*" >&2
  exit 1
}

source "${ROOT_DIR}/tools/checks/lib/allocator_provider_forbidden_patterns.sh"

allocator_provider_forbid_selection "$TAG"
allocator_provider_forbid_global_allocator "$TAG"
allocator_provider_forbid_proof_consumption "$TAG"
allocator_provider_forbid_rollback_preparation "$TAG"
allocator_provider_forbid_hook_activation "$TAG"
allocator_provider_forbid_activation_gate_open "$TAG"
allocator_provider_forbid_inc_matchers "$TAG"

echo "${TAG} ok"
