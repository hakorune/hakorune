#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-vm-nonconsumer-fence"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$ROOT_DIR/src/backend/mir_interpreter/exec/mod.rs" \
  "$ROOT_DIR/src/mir/a_prime_i64_physical_receipt.rs" \
  "$ROOT_DIR/docs/development/current/main/investigations/dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md"

shopt -s nullglob
VM_ROOTS=(
  "$ROOT_DIR/src/backend/mir_interpreter"
  "$ROOT_DIR/src/tests/vm"
  "$ROOT_DIR/src/tests"/vm*.rs
  "$ROOT_DIR/src/runner/dispatch.rs"
  "$ROOT_DIR/src/runner/route_orchestrator.rs"
  "$ROOT_DIR/src/runner/keep"/vm*.rs
  "$ROOT_DIR/src/runner/reference"/normal_file_vm*
  "$ROOT_DIR/src/runner/reference"/normal_file_canonical_core_vm*
  "$ROOT_DIR/src/runner/reference"/raw_vm_reference*
)

# The VM remains a bootstrap/reference/smoke lane.  These names are reserved
# for the later AOT/LLVM production capability and must not acquire a VM
# classifier, provider, receipt, adapter, session, or production edge.
for root in "${VM_ROOTS[@]}"; do
  hits="$(rg -n -i \
    "DynamicV2|SelectedDynamicV2|DynamicV2Physical|DynamicV2CallSlot|DynamicV2Provider|a_prime_i64_entry|APrimeVmI64Entry|classify_selected_a_prime_i64_argument|issue_selected_a_prime_i64_physical_demand" \
    "$root" --glob '*.rs' --glob '*.h' --glob '*.py' || true)"
  if [[ -n "$hits" ]]; then
    guard_fail "$TAG" "VM DynamicV2/A-prime production or adapter symbol detected under ${root#"$ROOT_DIR/"}:\n$hits"
  fi
done

if [[ -e "$ROOT_DIR/src/backend/mir_interpreter/exec/a_prime_i64_entry.rs" ]]; then
  guard_fail "$TAG" "caller-zero VM A-prime classifier module still exists"
fi

if rg -n -F -q "mod a_prime_i64_entry" \
  "$ROOT_DIR/src/backend/mir_interpreter/exec/mod.rs"; then
  guard_fail "$TAG" "caller-zero VM A-prime classifier module is still registered"
fi

# The physical receipt is an LLVM-only transport projection.  A VM variant
# would reopen the retired reference lane as a second capability authority.
if ! rg -n -F -q "Llvm" \
  "$ROOT_DIR/src/mir/a_prime_i64_physical_receipt.rs"; then
  guard_fail "$TAG" "A-prime physical receipt no longer declares its LLVM-only backend"
fi
if rg -n -e \
  "APrimeI64BackendFamilyV1::(Vm|Interpreter)|^[[:space:]]*(Vm|Interpreter)[[:space:]]*[,}]" \
  "$ROOT_DIR/src/mir/a_prime_i64_physical_receipt.rs"; then
  guard_fail "$TAG" "A-prime physical receipt declares a forbidden VM backend variant"
fi

echo "[$TAG] ok"
