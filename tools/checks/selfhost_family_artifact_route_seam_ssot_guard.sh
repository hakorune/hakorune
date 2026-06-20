#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/selfhost-family-artifact-route-seam-ssot.md"

for token in \
    "derived_hako" \
    "native_hako" \
    "rust_bootstrap" \
    "rust_compat" \
    "host_substrate" \
    "unsupported" \
    "fallback_policy=forbidden" \
    "do_not_runtime_fallback_from_Hako_to_Rust=1" \
    "do_not_delete_or_disable_Rust_bootstrap=1"
do
    if ! grep -q "$token" "$SSOT"; then
        echo "missing_route_seam_token=$token"
        exit 1
    fi
done

cat <<'REPORT'
output_contract=selfhost-family-artifact-route-seam-ssot-v0
selfhost_family_artifact_route_seam_ssot=1
allowed_routes={derived_hako,native_hako,rust_bootstrap,rust_compat,host_substrate,unsupported}
selection_requires_manifest=1
selection_requires_guard=1
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
implementation_started=0
backend_behavior_changed=0
summary=ok
REPORT
