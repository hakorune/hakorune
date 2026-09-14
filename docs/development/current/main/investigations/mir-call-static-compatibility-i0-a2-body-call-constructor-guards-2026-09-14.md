---
Status: selected__fast__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A2-BODY-CALL-CONSTRUCTOR-GUARDS
Date: 2026-09-14
Parent: mir-call-static-compatibility-a2-body-call-constructor-d1-2026-09-14.md
NextCard: none__a2_body_call_constructor_guard_closeout
Implementation permission: true for focused guard evidence in existing owners; no new semantic product
---

# A2 body, direct-call, and constructor guards I0

## Six-line brief

```text
Decision: close the finite A2 body/direct-call/constructor relation with existing owner checks and reject terminals.
Source authority + canonical issuer: parser final syntax loan, constructor catalog, resolver callable index, and existing package co-seal remain the sole issuers.
Non-authority: AST rescans, names/arity as keys, slot reconstruction, MIR/ValueId, compatibility fallback, publication, StringBox, and import lineage.
Fail-fast boundary: preserve typed parser/resolver/package rejects for missing, duplicate, foreign, orphan, target/header, arity, parent, and generated-origin drift.
Smallest next slice: add focused positive/negative coverage in the existing test owners and update the owner evidence only.
Non-claims: no MixedProgram admission, nested-owner policy change, caller switch, fallback retirement, Windows proof, or R7.
```

## Acceptance

- Positive evidence covers one ordinary/static body and direct-call source-index
  path plus one Birth/no-Birth constructor pair.
- Negative evidence names the existing terminal for missing/duplicate/foreign/
  orphan body rows, unissued/nested/wrong-arity/missing-header direct calls,
  foreign-parent or transformed-parent constructor rows, and mixed generated
  method/Birth provenance.
- The current production caller remains the normal root catalog lifecycle
  path; no new semantic receipt, fallback, or CI lane is introduced.
- Every changed source stays below the 760-line design boundary and
  `git diff --check` plus the pointer guard pass.

## Explicit non-claims

This row does not admit MixedProgram, change nested-owner target policy, alter
import-lineage transport, publish/cut over a backend, remove compatibility
edges, fix StringBox readers, prove Windows lifecycle, or close R7.
