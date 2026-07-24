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

The canonical session now also exposes a disconnected prepared-close seam:
`prepare_draft_seal_close(self)` validates the resolved-family session while
the unpublished function is still installed, and
`PreparedFunctionSessionCloseV1::commit(self)` takes `current_function` and
restores the caller context without a fallible edge. Rejection retains the
original session and discards it exactly once. Legacy `run`/`capture` paths
remain unchanged; this seam is not yet wired into the lowerers or the draft
seal projection, so the full PREPARE0/COMMIT0 row remains open.

The projection now has one metadata-plan step after type propagation:
`prepare_metadata()` refreshes and validates the existing parameter-entry and
return-exit carriers on the private image, snapshots final value types, and
passes that image into stale-fact preparation. No live metadata or contract
carrier is changed, and no second ReturnExit/parameter authority is created.

The metadata plan now also carries a prepared result/signature relation. It
uses the sealed exit intent plus the projected exact value type, never a
Return scan or last-ValueId heuristic. The S0 relation is `Unit` for explicit
or implicit Unit exits and `ExactOperand { value, return_type }` for the
supported Integer/Bool/Float/Void operand carriers. A projected signature
drift is rejected before any live function mutation; the live signature and
physical Return writer remain deferred to COMMIT0.

The projection path now has a strict PHI/CFG closure receipt before type-fact
planning. It calls the existing terminator-derived PHI edge verifier on the
private function image and rejects phantom, missing, duplicate, undefined, or
non-dominating edges. It does not call the legacy whole-function PHI repair;
the receipt is retained in the prepared type/metadata plan for the future
outer draft-seal owner.

The verification-image step now also runs the existing function verifier on
the projected function, after typed-value checks and stale-fact application.
Any failure is a typed `ProjectedVerificationFailed` rejection; no verifier
is run against live Builder state and no post-commit verification edge exists.

## DRAFT-SEAL0-S0 implementation checkpoint

The owner-preserving Open seam is now implemented as a separate
`draft_seal_owner.rs` box so the planner remains below the 800-line boundary.
`OpenFunctionDraftSealV1::prepare` performs exit, PHI, type/hint, metadata,
stale-fact, projected-verification, and session-readiness checks while the
canonical function session remains borrowed. Every failure returns the exact
Open owner; `discard(self)` is the only rejection terminal.

`PreparedFunctionDraftSealV1::commit` now consumes the typed session payload,
installs the projected function/type facts, takes `current_function` exactly
once, restores the caller context, and returns `CompletedFunctionDraftV1`.
The commit has no `Result`, lookup, repair, fallback, or retry edge. The
canonical lowerers and legacy finalizer remain disconnected; this row only
closes the owner/prepare/commit seam.

Focused completion tests cover one projected exact-value success and one
preterminated-return rejection with owner discard. The resolved-control-flow
guard now counts the four draft-seal products across the planner and owner
boxes and checks both files independently for the 800-line limit.

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
