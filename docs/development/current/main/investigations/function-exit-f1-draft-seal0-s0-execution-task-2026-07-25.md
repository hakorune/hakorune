# Function-exit F1 DRAFT-SEAL0 S0

Decision authority: `FUNCTION-EXIT-F1-DRAFT-SEAL-prime-r1`

Status: implementation authorized for this exact slice only.

First executable row:

```text
FUNCTION-EXIT-F1-DRAFT-SEAL0-S0
```

## Objective

Replace the current canonical completion close with one prepared,
non-Clone draft-seal owner. The owner must consume the already sealed F1
completion contract, validate the unpublished function by borrow, and commit
the physical exit, final facts, metadata, verification witness, draft
extraction, and session close without a fallible edge after commit begins.

The existing source contract remains the authority:

```text
SealedFunctionExitContractV1
VerifiedFunctionCompletionV1
function-owned TypeContext and lowering session
```

No source re-walk, last-ValueId inference, Return-scan signature inference,
postprocess repair, or Legacy fallback is allowed.

## Exact S0 boundary

Admit only the current exact 0-or-1 root-terminal topology for the canonical
and trivial-SSA lowerers:

```text
ImplicitUnit / EmptyBody
ImplicitUnit / ImplicitFallthrough
ExplicitUnit / ExplicitVoid or ExplicitNull
ExplicitValue / exact supported operand type
```

Supported physical result types for this row are the existing
`Integer`/`Bool`/`Float`/`Void` capability. Missing, Unknown, dynamic, and
unsupported result carriers reject before commit. Existing physical
preterminated Return/other terminators reject; a second Return is never
accepted.

Nested, multiple, cleanup-bearing, and all-path exits remain outside this row.
Bare-return grammar activation, Script tail classification, physical Main,
process-exit projection, public ingress, JSON, executor, and CUT0 remain
non-claims.

## Owner chain

```text
ReadyFunctionDraftSealV1
  -> OpenFunctionDraftSealV1::prepare(self)
       -> RejectedFunctionDraftSealV1 (discard-only)
       -> PreparedFunctionDraftSealV1
            -> commit(self) -> CompletedFunctionDraftV1
```

All products are move-only. `MirFunction` has no mutable public escape.
`current_function` is borrowed during preparation and taken exactly once in
the infallible commit. `current_module` is never taken by the canonical
draft seal; only a short-lived signature/header lookup loan is allowed.

## Preparation order

The one preparation authority performs borrow-only checks and plans in this
order:

```text
authority/session closure
-> logical exit projection
-> PHI/CFG closure verification (no legacy repair)
-> projected type-fact plan and type hints
-> stale-fact plan
-> exact signature/result plan
-> metadata and existing contract plan
-> projected completed-draft verification
-> extraction/session-close readiness
```

Each failure retains the exact unpublished open owner, typed stage, nested
cause, and progress. No partial Return/signature mutation is allowed before
the prepared product is issued.

## Commit law

`PreparedFunctionDraftSealV1::commit(self)` is the sole physical exit writer.
It performs only predetermined ownership/state transitions:

```text
planned synthetic Void allocation when required
-> one physical Return
-> final type facts and exact signature
-> metadata/contracts
-> current_function extraction
-> session restoration/closure
-> CompletedFunctionDraftV1
```

The canonical and trivial-SSA lowerers must emit no physical Return in this
row. `finalize_ready_function_completion`,
`finalize_preterminated_function_completion`, and canonical/trivial direct
Return writers are not new consumers of the seal.

## Implementation order

```text
DRAFT-SEAL-CONTRACT0
  move-only owner vocabulary, exit intent, typed rejection/progress

DRAFT-SEAL-PREPARE0
  pure exit/PHI/type/stale/signature/metadata/verification plans

DRAFT-SEAL-COMMIT0
  one infallible physical exit and draft/session close

DRAFT-SEAL-I0
  canonical and trivial-SSA lowerer handoff

DRAFT-SEAL-G0
  focused parity/retention fixtures and structural authority guard
```

Keep each changed source/check file below 800 lines. Do not broaden the
existing `finalize_function_draft` contract until the prepared owner has a
typed plan for every operation it currently performs.

## Landed substep

`DRAFT-SEAL-CONTRACT0` is now fixed in the Builder vocabulary. The new
`ReadyFunctionDraftSealV1 -> PreparedFunctionDraftSealV1 ->
CompletedFunctionDraftV1` chain classifies explicit value, explicit unit, and
implicit unit exits from the sealed completion plus the exact lowered operand
witness. This substep is intentionally not `DRAFT-SEAL-PREPARE0`: it does not
project `MirFunction`/`TypeContext`, run type or stale-fact planning, verify a
projected draft, or close the canonical session. The legacy finalizer and its
physical Return writers remain unchanged until those plans exist.

The next projection substep is also present as an isolated, non-mutating
helper: it copies the current function/type facts, validates the exact exit
block and supported result type, materializes a planned `Return` (and a
reserved-ID-safe synthetic `Void` when needed), and proves the live function
is unchanged. It is still not the full `DRAFT-SEAL-PREPARE0`: propagation,
hints, stale-fact normalization, projected verification, and canonical
session-close readiness remain pending.

The shared `TypePropagationPipeline` now also runs against that private image
only. Its failure remains a typed projection rejection; no live function or
live type map is passed to the pipeline.

The projection also carries a prepared stale-fact removal plan. Pending-PHI
and pinned-value retention sets are observed from the live session, while the
removal itself is not committed yet. The live type map remains byte-for-byte
unchanged in the focused fixture.

Projected typed-value verification now applies that plan to a second private
facts image and keeps the original plan available. This is verification-image
work only; the live map, draft extraction, and session close remain untouched.

## Acceptance gates

```text
one PreparedFunctionDraftSealV1 producer
one commit terminal and one physical Return writer
canonical direct Return writers = 0
trivial-SSA direct Return writers = 0
F1 Return-scan/signature inference = 0
legacy whole-function PHI repair = 0 in the seal
prepare failures retain exact owner and no Builder mutation
commit has no Result/fallback/retry/repair path
canonical/trivial empty, Unit, and exact value fixtures green
same Builder reuse after success and rejection
current-state pointer guard and diff check green
all modified source/check files < 800 lines
```

## Non-claims

```text
nested/multiple/all-path completion
cleanup-bearing return materialization
Script result tail semantics
physical Main/source-entry thunk
process-exit projection
App compatibility execution
public ingress, JSON, executor, selfhost/fastmem
old Raw-chain retirement
public-adapter repair
CUT0
```
