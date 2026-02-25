#!/usr/bin/env bash
# Nyash dev environment convenience script
# Usage: source tools/dev_env.sh [profile]
# Profiles:
#   pyvm       - Legacy alias (mapped to bridge profile)
#   bridge     - Bridge-only helpers (keep interpreter)
#   phi_off    - PHI-less MIR (edge-copy) + verifier relax; harness on
#   opbox      - Enable Operator Boxes (Stringify/Compare/Add) with adopt; VM tolerate void; AST using ON
#   hako-only  - Buildless Hako dev (alias using, hv1 direct verify)
#   hybrid     - Hako + Rust VM 両用（verifyはhakorune優先）
#   prod       - Prod-like (path using error; minimal trace)
#   reset      - Unset variables set by this script

set -euo pipefail

activate_pyvm() {
  activate_bridge
  echo "[dev-env] pyvm profile is retired; using bridge profile defaults." >&2
  echo "[dev-env] historical pyvm route: tools/historical/pyvm/*.sh" >&2
}

activate_bridge() {
  unset NYASH_DISABLE_PLUGINS || true
  export NYASH_NY_COMPILER_TIMEOUT_MS=${NYASH_NY_COMPILER_TIMEOUT_MS:-2000}
  export NYASH_MIR_UNIFIED_CALL=${NYASH_MIR_UNIFIED_CALL:-1}
  export NYASH_DEV_DISABLE_LEGACY_METHOD_REWRITE=1
  echo "[dev-env] Bridge profile activated (Rust VM bridge; plugins ON)" >&2
}

reset_env() {
  unset NYASH_DISABLE_PLUGINS || true
  unset NYASH_NY_COMPILER_TIMEOUT_MS || true
  unset NYASH_MIR_UNIFIED_CALL || true
  unset NYASH_DEV_DISABLE_LEGACY_METHOD_REWRITE || true
  unset NYASH_MIR_NO_PHI || true
  unset NYASH_VERIFY_ALLOW_NO_PHI || true
  unset NYASH_LLVM_USE_HARNESS || true
  echo "[dev-env] environment reset" >&2
}

activate_phi_off() {
  export NYASH_MIR_NO_PHI=1
  export NYASH_VERIFY_ALLOW_NO_PHI=1
  export NYASH_LLVM_USE_HARNESS=1
  echo "[dev-env] PHI-off (edge-copy) profile activated (harness on)" >&2
}

activate_opbox() {
  export NYASH_USING_AST=1
  # Runtime operator boxes
  export NYASH_OPERATOR_BOX_STRINGIFY=1
  export NYASH_OPERATOR_BOX_COMPARE=1
  export NYASH_OPERATOR_BOX_ADD=1
  export NYASH_OPERATOR_BOX_ALL=1
  export NYASH_OPERATOR_BOX_COMPARE_ADOPT=1
  export NYASH_OPERATOR_BOX_ADD_ADOPT=1
  export NYASH_VM_TOLERATE_VOID=1
  # Builder lowering to operator calls
  export NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL=1
  export NYASH_BUILDER_OPERATOR_BOX_ADD_CALL=1
  export NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=1
  # Unified call and legacy suppression
  export NYASH_MIR_UNIFIED_CALL=${NYASH_MIR_UNIFIED_CALL:-1}
  export NYASH_DEV_DISABLE_LEGACY_METHOD_REWRITE=1
  echo "[dev-env] Operator Boxes (stringify/compare/add) enabled (adopt+builder-call)" >&2
}

# Buildless Hako-only profile
activate_hako_only() {
  # Using/alias
  export NYASH_ENABLE_USING=1
  export NYASH_USING_AST=1
  export NYASH_ALLOW_USING_FILE=1
  # Resolver (optional helpers kept quiet by default)
  : "${NYASH_RESOLVE_TRACE:=0}"
  : "${NYASH_RESOLVE_NORMALIZE:=0}"
  # hv1 direct verify primary
  export HAKO_V1_DISPATCHER_FLOW=1
  export HAKO_VERIFY_PRIMARY=hakovm
  echo "[dev-env] Hako-only profile activated (buildless; hv1 direct verify)" >&2
}

# Hybrid: Hako + Rust VM（verifyはhakovm直行）
activate_hybrid() {
  export NYASH_ENABLE_USING=1
  export NYASH_USING_AST=1
  export NYASH_ALLOW_USING_FILE=1
  : "${NYASH_RESOLVE_TRACE:=0}"
  : "${NYASH_RESOLVE_NORMALIZE:=0}"
  export HAKO_V1_DISPATCHER_FLOW=1
  export HAKO_VERIFY_PRIMARY=hakovm
  # Rust VM 側は特別扱い不要（通常通り）
  echo "[dev-env] Hybrid profile activated (Hako + Rust VM; hv1 verify)" >&2
}

# Prod-like: Path using を ERROR にし、最小トレース
activate_prod() {
  # Disallow AST prelude merge and file using
  export NYASH_USING_AST=0
  export NYASH_ALLOW_USING_FILE=0
  # Mark using profile as prod to enable strict path-using errors
  export NYASH_USING_PROFILE=prod
  # Keep using gate explicit; alias resolution allowed via resolver (no AST merge)
  : "${NYASH_ENABLE_USING:=1}"
  # Quiet resolver by default
  export NYASH_RESOLVE_TRACE=0
  export NYASH_RESOLVE_NORMALIZE=0
  # Do not force hv1 verify primary here
  unset HAKO_VERIFY_PRIMARY || true
  unset HAKO_ROUTE_HAKOVM || true
  echo "[dev-env] Prod profile activated (path using ERROR; quiet)" >&2
}

case "${1:-bridge}" in
  pyvm) activate_pyvm ;;
  bridge) activate_bridge ;;
  phi_off) activate_phi_off ;;
  opbox) activate_opbox ;;
  hako-only) activate_hako_only ;;
  hybrid) activate_hybrid ;;
  prod) activate_prod ;;
  reset) reset_env ;;
  *) echo "usage: source tools/dev_env.sh [bridge|phi_off|opbox|hako-only|hybrid|prod|reset|pyvm]" >&2 ;;
esac
