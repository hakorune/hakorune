# RAW-SOURCE0 LOWER ROOT — POST0 publication consultation

Status: **Design stop — COMMIT0-S0 closed; awaiting publication decision**  
Date: 2026-07-24  
Predecessor: `cut0-i0-raw-source0-lower-root-post0-commit0-s0-execution-task-2026-07-24.md`

## Current evidence

COMMIT0-S0 now consumes the closed POST0 owner:

```text
RawPostprocessedInvocationV1::{Script, App}
  -> prepare_external_commit(self)
  -> PreparedRawExternalCommitV1::{Script, App}
```

The prepared product retains:

```text
opaque RawExternalCommitModuleV1
PreparedBuilderExternalCommitV1
complete RawPostprocessEvidenceV1
  route / continuation / runtime snapshot / module name
  child and callable receipts
  RawDrainWitness / sealed ledger / root witness
  FINAL0 and POST0 parity
  Raw schedule / pre-transform verification / progress
```

The new path has no publication terminal. The existing
`PreparedBuilderExternalCommitV1::commit` still replaces the live Builder,
while `PreparedModuleExternalCommitV1` still owns a bare `MirModule` and
the legacy Raw ledger/root evidence. No RawDirect caller has been added.

## Authority boundary

The next decision must define the one actual publication authority without
downgrading the RawDirect product to a bare module or the old Raw evidence
variant.

### Q1 — publication authority

Here “one authority” means one publication terminal/low-level Builder
replacement namespace. It does not require forcing the legacy
`PreparedModuleExternalCommitV1<'a>` bare-module fields or its fixed
`MirCompileResult` return type to own the Raw opaque carrier. A RawDirect
named payload may remain separate while sharing only the private one-shot
Builder replacement primitive.

Which owner performs the one live Builder replacement?

```text
1. Extend the existing PreparedModuleExternalCommitV1 / MirCompiler commit
   authority with a private RawDirect module/evidence variant. It remains the
   sole publication terminal; legacy routes keep their existing variant.

2. Add a separate Raw publication terminal beside the existing commit owner.

3. Convert RawDirect back into legacy PostprocessedModuleInvocationV1 and use
   the old publication path.
```

### Q2 — opaque module opening

When may `RawExternalCommitModuleV1` become a `MirModule`?

```text
1. Only inside the sole publication terminal, after all target Builder and
   evidence preflight. The opaque carrier is consumed immediately before the
   infallible live assignment; no compiler accessor or intermediate bare
   module crosses the boundary.

2. Open it in COMMIT0 and store a bare module in the prepared product.

3. Keep a clone/rollback copy and open it before target preflight.
```

### Q3 — published result contract

What should successful Raw publication return internally?

```text
1. A typed RawPublishedInvocationV1::{Script, App} retaining the complete
   RawDirect evidence and a publication seal. A later public adapter may
   project MirCompileResult; this row keeps route/evidence authority.

2. Return the existing MirCompileResult immediately and drop route/runtime/
   receipt evidence after assignment.

3. Infer a neutral route from the final module and return one untyped result.
```

### Q4 — Raw pre-transform verifier result

The Raw schedule already reports pre-transform verification and does not claim
a final-verifier barrier. How should a reportable Raw verifier error cross the
publication boundary?

```text
1. Preserve ModuleVerificationEvidenceV1::Raw { pre_transform: Err(..) } in
   the typed published evidence. A later MirCompileResult adapter may expose
   the same Err without rejecting the unpublished module here.

2. Reject the Raw publication before live Builder mutation.

3. Drop or normalize the error and publish success-only evidence.
```

### Q5 — target preflight and failure

What is the atomicity law before live Builder mutation?

```text
1. Borrow-only target Builder vacancy/family/brand/evidence preflight, then
   one infallible consuming publication. No fallible work remains after the
   prepared publication owner is issued; failure retains the exact prepared
   RawDirect owner and exposes discard only.

2. Reuse the old fallible prepare path and allow partial owner consumption.

3. Clone the module and roll back if live assignment or result creation fails.
```

### Q6 — first consumer scope

Which caller is allowed after publication is selected?

```text
1. One compiler-internal RawDirect publication consumer only. Public Raw
   ingress, executor wiring, AST-JSON, Program(JSON v0), and CUT0 remain zero.

2. Wire the public Raw ingress in the same row.

3. Replace every existing compile route in one cutover.
```

### Q7 — retirement boundary

How are old publication surfaces retired?

```text
1. Keep legacy PreparedModuleExternalCommitV1 and test-only old Raw callers
   disconnected until a later retirement row proves non-test caller count zero.
   The RawDirect variant gets its own guard and sunset record.

2. Delete old commit/evidence variants in the publication row.

3. Keep both paths indefinitely with no sunset condition.
```

## Recommended candidate

Candidate **RAW-PUBLICATION-prime-r1** is recommended:

```text
Q1 = 1
Q2 = 1
Q3 = 1
Q4 = 1
Q5 = 1
Q6 = 1
Q7 = 1
```

This is a new publication authority/result boundary, so implementation must
stop until the candidate is selected. After selection, the first executable
row is:

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION0-S0
```

## Non-claims at this stop

```text
live Builder replacement
MirCompileResult publication
public Raw ingress / executor
AST-JSON / Program(JSON v0) behavior
old legacy commit retirement
JSON bridge ownership
production CUT0 activation
fastmem or selfhost consumer activation
```
