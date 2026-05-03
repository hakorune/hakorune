---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv source-execution owner boundary canaries
Related:
  - docs/development/current/main/phases/phase-29cv/P337A-RETURN-STR-LENGTH-METHOD-EMIT.md
  - lang/src/mir/builder/internal/README.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P338A: Owner Contract Canaries

## Problem

P337A put `return "hello".length()` back on the existing
`LowerReturnMethodStringLengthBox` owner and kept `CallMethodizeBox` as an
identity pass for already-canonical call MIR.

That behavior was correct, but the boundary was still easy to break later:

- `CallMethodizeBox` could start rewriting canonical call MIR when methodize is
  enabled
- `LowerReturnStringBox` could regress and accept
  `Return(Method(recv=Str|String, method=length|size))` as a direct string return
- `MirBuilderMinBox` could grow a dispatch for `return.method.string.length`
  even though that belongs to the registry authority path
- `LowerReturnMethodStringLengthBox` documented `args=[]` but did not enforce it

## Boundary

Do not add a new pattern.

Do not widen `MirBuilderMinBox`.

Do not let `return.string` accept nested method receivers.

Do not let methodize reinterpret canonical calls that already carry structured
callee metadata.

## Implementation

- add `mirbuilder_call_methodize_canonical_identity_canary_vm.sh`
- add `mirbuilder_return_string_owner_boundary_canary_vm.sh`
- add `phase29cv-owner-contracts.txt` as a small suite manifest for these
  boundary canaries
- document the source-execution owner matrix in
  `lang/src/mir/builder/internal/README.md`
- enforce `args=[]` inside `LowerReturnMethodStringLengthBox`

## Acceptance

```text
bash tools/smokes/v2/profiles/integration/core/phase2034/mirbuilder_call_methodize_canonical_identity_canary_vm.sh
-> PASS
```

```text
bash tools/smokes/v2/profiles/integration/core/phase2034/mirbuilder_return_string_owner_boundary_canary_vm.sh
-> PASS
```

```text
bash tools/smokes/v2/run.sh --profile integration --suite phase29cv-owner-contracts
-> PASS
```

```text
bash tools/checks/current_state_pointer_guard.sh
-> ok

git diff --check
-> ok
```
