# Function-exit F1 MATERIALIZE0 design stop

Decision authority: `FUNCTION-EXIT-SEMANTICS-prime-r1`

Status: design consultation required before implementation.

The executable row is paused at:

```text
FUNCTION-EXIT-F1-MATERIALIZE0-D0
```

## Why implementation stops here

`VerifiedFunctionCompletionV1::function_exit_contract()` is the source
authority. The existing Builder completion consumer is the only physical
completion owner. However, its current finalizer combines two different
responsibilities:

```text
sealed source exit disposition -> physical Return/signature materialization
physical draft -> type/fact cleanup, verification, and draft extraction
```

The second part is still fallible after the first part may have mutated the
open function. In the current `finalize_function_draft` path, failure can
still occur during type propagation, stale-fact preparation/commit, type-hint
metadata, completed-draft verification, or `current_function.take()`. The
finalizer also scans Return operands to infer a signature. That scan is not a
source contract authority and cannot remain part of a strict F1 materializer.

The trivial-SSA lowerer is an additional convergence constraint: it currently
emits a synthetic Void Return before the shared finalizer while the finalizer
can also emit one. Removing that writer requires a sealed-function boundary,
not a local conditional.

Therefore the current implementation cannot honestly claim:

```text
borrow-only prepare -> private infallible materialization commit
```

Adding a signature helper around the existing finalizer would leave a
post-Return fallible edge and a second Return/signature authority.

## Authority and non-authority

Authority:

```text
SealedFunctionExitContractV1
VerifiedFunctionCompletionV1
existing completion-owner session and its typed coverage
```

Not authority:

```text
last lowered ValueId position
MIR Return scan for source semantics
finalize_function_draft signature inference
postprocess return repair
Legacy build_module fallback
```

## Candidate designs

### A — bounded card amendment

Define MATERIALIZE0 as only the existing exit materialization boundary. It may
prepare and commit the Return/signature facts, while the existing draft
finalizer remains a later, fallible post-materialization step. This is only
valid if the card is amended to remove the strict post-commit-infallible claim
and to record the remaining temporal mutation explicitly.

This keeps the current owner shape but does not close the stronger atomicity
law requested by the accepted F1 decision.

### B — strict draft-seal row (recommended)

Open a new design row before code:

```text
FUNCTION-EXIT-F1-DRAFT-SEAL-D0
```

Split `finalize_function_draft` into:

```text
borrow-only prepared draft seal
private infallible draft commit
```

The split must cover type propagation, stale-fact handling, type-hint
metadata, completed-draft verification, function-state extraction, and the
trivial-SSA synthetic-Return convergence. Only after that owner boundary is
sealed can MATERIALIZE0 co-seal signature, physical Return, completion, and
cleanup without a fallible edge after mutation.

## Selected next action

Select B for consultation. Do not implement MATERIALIZE0 or widen the
existing finalizer until the draft-seal owner, its failure retention, and its
trivial-SSA convergence are accepted.

The next consultation must answer:

```text
which facts are borrow-only plans
which state is retained in a rejected unpublished owner
where the sole synthetic Return writer lives
which operation issues the PreparedFunctionDraftSealV1
which commit is infallible
```

No new source, Builder, or guard implementation is authorized by this card.

## Non-claims

```text
MATERIALIZE0 implementation complete
nested/multiple/all-path completion
Script tail semantics
physical Main or process-exit projection
App compatibility execution
public ingress, JSON, executor, normal-entry cutover
old Raw-chain retirement
```
