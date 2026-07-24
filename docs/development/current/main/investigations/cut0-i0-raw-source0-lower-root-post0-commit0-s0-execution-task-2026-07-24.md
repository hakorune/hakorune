# RAW-SOURCE0 LOWER ROOT — POST0-COMMIT0-S0 execution task

Status: **In progress — RawDirect external-commit preparation only**  
Date: 2026-07-24  
Decision: **RAW-COMMIT-prime-r1**  
Predecessor: `cut0-i0-raw-source0-lower-root-post0-commit-consultation-question-2026-07-24.md`

The design consultation is closed. This row consumes the already-closed
`RawPostprocessedInvocationV1::{Script, App}` owner and prepares a named,
opaque RawDirect external-commit product. It does not publish the live
Builder, issue `MirCompileResult`, or wire public ingress.

## Decision lock

```text
Q1  RawPostprocessedInvocationV1::prepare_external_commit(self) is the sole
    compiler-visible entry. It returns PreparedRawExternalCommitV1::{Script,
    App} or a discard-only RejectedRawExternalCommitInvocationV1.

Q2  One Builder sibling terminal borrows the complete physical owner and then
    performs one infallible preflighted handoff. Existing
    PreparedBuilderModuleSessionV1 is already the readiness proof; COMMIT0
    only uses its infallible into_external_commit transition.

Q3  RawPostprocessEvidenceV1 is the sole complete RawDirect evidence
    aggregate. It retains route, continuation, runtime snapshot, module name,
    child/callable receipts, RawDrainWitness, FINAL0/POST0 parity, schedule,
    verification evidence, and progress. No ledger re-projection or module
    re-scan is allowed.

Q4  All validation is borrow-only. Failure returns the exact unchanged
    RawPostprocessedInvocationV1 owner with stage, typed cause, inspection,
    and discard(self) only. After preflight, remaining operations are
    infallible ownership conversions.

Q5  Script/App remain typed in PreparedRawExternalCommitV1. App retains its
    callable-Main outcome. Route is never inferred from symbols or manifest
    topology.

Q6  This row stops at PreparedRawExternalCommitV1. Existing commit authority,
    live Builder mutation, MirCompileResult publication, public ingress, JSON
    bridges, executor wiring, old-chain retirement, and CUT0 remain zero.
```

## S0a — complete evidence aggregate

Promote the current split route/physical evidence into one non-Clone
`RawPostprocessEvidenceV1` owner without cloning or weakening its proof.

It must retain, by value:

```text
RawPostprocessRouteEvidenceV1::{Script, App}
RawPostCallableMainContinuationV1
RawRuntimeInputSnapshotV1
module name
RawPreRootChildrenCompletionV1
helper receipts
App callable-Main outcome
RawDrainWitnessV1
RawFinalizationParitySealV1
RawPostprocessParitySealV1
ModulePostprocessScheduleV1
ModuleVerificationEvidenceV1::Raw
RawPostprocessProgressV1
```

The old `Raw { ledger, root }` evidence remains legacy/disconnected. COMMIT0
must not create a smaller RawDirect seal, reconstruct evidence from module
symbols, re-project the sealed ledger, or infer the route from inventory.

The aggregate is assembled exactly once while consuming the successful
POST0 owner. A borrowed validation view may exist privately, but it is not a
second stored authority.

## S0b — opaque physical handoff

Add one Builder-sibling terminal over `RawPostprocessedPhysicalV1`:

```text
borrowed validation of family/brands/progress/evidence
-> preflighted owner conversion
-> PreparedBuilderModuleSessionV1::into_external_commit()
-> RawExternalCommitModuleV1 (opaque RawPostprocessedModuleV1 carrier)
```

The new carrier must not expose:

```text
MirModule field
Deref / DerefMut
AsRef / AsMut<MirModule>
module_mut / into_module
caller-provided mutation closure
```

Do not call `prepare_module_session()` again. Do not widen
`MirCompiler::prepare_module_external_commit`, adapt to
`PostprocessedModuleInvocationV1`, or use the old Raw ledger/root-only
evidence variant.

## S0c — single compiler terminal and rejection owner

Implement only this compiler-visible transition:

```rust
RawPostprocessedInvocationV1::prepare_external_commit(self)
  -> Result<
       PreparedRawExternalCommitV1,
       RejectedRawExternalCommitInvocationV1,
     >
```

Preflight order is fixed:

```text
route variant <-> route evidence correspondence
Raw family and token/Builder/witness/parity brand equality
POST0 progress == ParitySealed
Raw schedule and Raw verification correspondence
manifest/callable/helper evidence correspondence
physical readiness already represented by PreparedBuilderModuleSessionV1
```

Every check borrows the complete owner. The rejection retains the exact
owner and a typed stage/cause; public inspection is limited to
`stage(&self)`, `error(&self)`, and `discard(self)`. No retry, resume,
fallback, rollback clone, evidence replacement, legacy downgrade,
`into_module`, or publication continuation is permitted.

Prepared success retains the complete evidence aggregate and a named opaque
physical product. It is not convertible to the existing publication owner
inside this row.

## S0d — focused proof and guard

Add or extend one reusable Raw lane guard rather than a per-field shell guard.
The guard must prove:

```text
prepare_external_commit definition                         = 1
RawPostprocessedInvocationV1 production consumer            = 1
PreparedRawExternalCommitV1 producer                        = 1
complete RawPostprocessEvidenceV1 producer                  = 1
RawPostprocessedPhysicalV1 external terminal                = 1
PreparedBuilderModuleSessionV1::into_external_commit use   = 1

bare MirModule / DerefMut / AsMut / into_module in new path = 0
old PostprocessedModuleInvocationV1 adapter                  = 0
old Raw { ledger, root } downgrade                           = 0
ledger reprojection / module inventory rescan                = 0
route inference from symbols                                 = 0
second readiness check                                       = 0

PreparedModuleExternalCommitV1::commit new Raw caller        = 0
MirCompiler::commit_prepared_module new Raw caller           = 0
MirCompileResult producer                                   = 0
live Builder mutation/public ingress/executor/JSON/CUT0      = 0
retry/resume/fallback/rollback/catch_unwind                  = 0
```

## Required fixtures

```text
success: empty Script and scalar Script
success: App callable-Main NotSelected
success: App callable-Main Selected
success: App with helper receipts
success: Raw verification Ok and reportable pre-transform Err

failure: crossed Script/App route evidence
failure: foreign family or token/Builder/witness/parity brand
failure: progress not ParitySealed
failure: schedule/verification mismatch
failure: helper/callable receipt evidence drift

retention: every failure retains the exact postprocessed owner,
opaque module, Builder readiness, witness/parities, route/runtime evidence,
verification evidence, and no state mutation occurs

one-shot: second prepare, retry, fallback, bare-module escape, and actual
commit caller remain zero
```

## Verification

```bash
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_external_commit_p0 --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_commit0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The full library suite is not a required gate for this row; existing broad
baseline failures remain separately tracked. Focused Raw POST0/FINAL0/DRAIN0
tests must remain green.

## Non-claims

```text
PreparedModuleExternalCommitV1::commit        = 0
MirCompileResult publication                 = 0
live Builder replacement                     = 0
public Raw ingress / executor wiring         = 0
AST-JSON / Program(JSON v0) behavior         = 0
old Raw finalizer/postprocess retirement     = 0
POST0 optimizer/policy changes               = 0
JSON bridge ownership                        = 0
atomic CUT0 activation                       = 0
```

## Proof budget / sunset

```text
ceremony_tier = T1 (selected bounded handoff; no new publication authority)
sunset_id = RAW-COMMIT-SUNSET-001
proof_inventory_before = POST0 route/witness/parity/opaque-carrier evidence
new_proofs = one complete RawDirect evidence aggregate and one opaque commit handoff
retired_or_merged_proofs = split POST0 evidence storage at COMMIT0 handoff
sunset_row = later Raw publication/retirement row
retire_when = RawDirect evidence is consumed by the sole existing publication authority and old Raw callers are zero
budget_repayment_evidence = one reusable Raw lane guard plus route matrix
```

## Internal order

```text
COMMIT-EVIDENCE0 -> COMMIT-PHYSICAL0 -> COMMIT-I0 -> COMMIT-G0
```

All modified/new source and check files must remain below 800 lines. The
production consumer remains forbidden until a later explicit publication
decision.

