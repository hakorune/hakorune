---
Status: Complete
Date: 2026-06-24
Scope: Fix same-module global-call contract inference drift.
---

# GLOBAL-CALL-SAME-MODULE-CONTRACT-INFERENCE-FIX-001

## Decision

Tighten same-module static-helper return-contract inference so unproven
return-value contracts do not silently preserve scalar or void-sentinel direct
ABI routes.

This is orthogonal to descriptor generation. The previous descriptor drain only
moved the fixed direct contract tuple into generated data; these failures were
in the Rust route-plan contract inference owner.

## Fix

```text
merge_same_module_static_helper_contract:
  ObjectHandle may be preserved when the child route has no additional return
  contract.

  ScalarI64 / VoidSentinelI64Zero are not preserved across unknown child
  contracts.

same_module_static_helper_contract_allowed:
  VoidSentinelI64Zero is allowed only for void-returning helpers.
```

## Acceptance

```text
param-or-void helper without string evidence no longer becomes DirectAbi
string concat / unknown value-returning helper no longer preserves scalar route
current string-or-void sentinel target uses the explicit
  typed_global_call_generic_string_or_void_sentinel contract
descriptor generator behavior changed = 0
C shim behavior changed = 0
```

## Verification

```text
cargo test -q global_call_route_plan:: --lib
git diff --check
```

Result:

```text
All commands above are green.
```
