---
Status: Complete
Date: 2026-06-24
Scope: Generate the first same-module global-call descriptor consumer.
---

# GLOBAL-CALL-PROOF-DESCRIPTOR-GENERATION-001

## Decision

Use `lowering_plan_proof_is_typed_global_call_contract()` as the first
same-module global-call descriptor consumer.

This is intentionally narrower than full same-module global-call lowering. It
removes one C-side handwritten proof allowlist and replaces it with generated
descriptor data from the Rust global-call proof authority.

## Source Authority

```text
src/mir/global_call_route_plan/model.rs
  GlobalCallProof
  GlobalCallProof::as_json_name()
```

## Generated Consumer

```text
lang/c-abi/shims/hako_llvmc_ffi_global_call_route_registry.inc
```

The C shim must ask the generated registry whether a proof is a valid typed
global-call contract. It must not keep its own list of
`typed_global_call_*` proof strings.

## Acceptance

```text
generator validates every GlobalCallProof variant has one JSON proof name
generated C registry contains all typed_global_call_* proof strings
lowering_plan_proof_is_typed_global_call_contract has no handwritten proof list
global-call emit/route behavior changed = 0
user-box method behavior changed = 0
extern descriptor behavior changed = 0
new canonical MIR instruction = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/global_call_route_descriptor_codegen.py --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result:

```text
All commands above are green.
```

## Non-Claims

```text
stage1 emit route matcher generation = 0
same-module function definition descriptor generation = 0
user-box method descriptor generation = 0
global-call planner redesign = 0
```
