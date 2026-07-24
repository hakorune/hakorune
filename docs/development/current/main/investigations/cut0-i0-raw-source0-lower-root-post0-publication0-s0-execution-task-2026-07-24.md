# RAW-SOURCE0 LOWER ROOT — POST0-PUBLICATION0-S0 execution task

Status: **Closed — PUBLICATION0-S0 implementation and focused proof green**
Date: 2026-07-24
Decision: **RAW-PUBLICATION-prime-r1**
Predecessor: `cut0-i0-raw-source0-lower-root-post0-publication-consultation-2026-07-24.md`

The publication consultation is closed. This row consumes the already-closed
`PreparedRawExternalCommitV1::{Script, App}` owner and performs the first
actual RawDirect live-Builder publication through one shared private
publication kernel.

This row does not project `MirCompileResult`, wire public Raw ingress, change
AST-JSON or Program(JSON v0), activate an executor, retire the old Raw chain,
or activate CUT0.

## Current inventory closeout

The worker audit was performed against clean `public-main` HEAD
`7a41d51ad7`.

```text
COMMIT0-S0                                      = closed
PreparedRawExternalCommitV1 terminal            = absent
RawExternalCommitModuleV1 bare extraction       = absent
PreparedBuilderExternalCommitV1 live assignment = existing one-line primitive
public compile_with_source RawDirect caller      = 0
old Raw finalization non-test caller             = 0
src/mir files at or above 800 lines              = 0
canonical_root_completion.rs                     = 738 lines
```

The normal AST production ingress still reaches legacy `build_module` through
`compiler/mod.rs`, `runtime/mirbuilder_emit.rs`, and the AST-JSON host
provider. The old Raw finalization bridge still contains the Main-only
`condition_fn`/`main` inventory and bare-module construction, but it remains
non-test-caller-zero. Neither concern belongs in PUBLICATION0.

PUBLICATION0 implementation is now in progress: the shared assignment
kernel, target-quiescence helper, opaque Raw published carrier, typed
Script/App result, and focused publication/evidence fixtures are present.
Public Raw ingress, `MirCompileResult` adaptation, executor wiring, old-chain
retirement, and CUT0 remain disconnected as required by this card.

The CI command below currently reports tracked-manifest drift:

```bash
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
```

That drift covers unrelated Failure/Outcome source movements and new evidence
sites. It must be refreshed through
`failure-outcome-semantic-manifest-maint0-execution-task-2026-07-24.md` in a
separate commit before PUBLICATION0 closeout. Do not mix the generated
manifest delta with the publication authority implementation.

## Decision lock

```text
Q1  Legacy and RawDirect retain separate payload/result types. Both use one
    private sealed publish_once kernel, which is the only non-test caller of
    the low-level live Builder replacement primitive.

Q2  RawExternalCommitModuleV1 remains opaque through target preflight. The
    publication kernel consumes it directly into RawPublishedModuleV1; no
    bare MirModule crosses the compiler boundary.

Q3  Success is RawPublishedInvocationV1::{Script, App}. It retains the token,
    opaque published module, complete RawPostprocessEvidenceV1, and a
    publication seal issued only after the actual Builder assignment.

Q4  ModuleVerificationEvidenceV1::Raw { pre_transform: Err(..) } is
    reportable publication success evidence. PUBLICATION0 has no verifier
    rejection producer.

Q5  One shared Builder quiescence checker validates only transactional target
    lanes. A successful prepared publication holds the mutable target borrow;
    no fallible work remains after it is issued.

Q6  MirCompiler::publish_raw_direct is the sole compiler-internal RawDirect
    consumer. Public adapters, ingress, executor, JSON, fastmem, selfhost, and
    CUT0 callers remain zero.

Q7  Legacy Raw publication surfaces stay disconnected until a measured
    retirement row. RAW-PUBLICATION-SUNSET-001 records their deletion
    condition; canonical publication remains unaffected.
```

## PUBLICATION-KERNEL0

Introduce one private shared publication vocabulary:

```rust
struct PreparedPublicationV1<'target, P> {
    target: &'target mut MirBuilder,
    builder: PreparedBuilderExternalCommitV1,
    payload: P,
    _seal: PreparedPublicationSealV1,
}

trait SealedPublicationPayloadV1 {
    type Published;

    fn finish(
        self,
        receipt: BuilderPublicationReceiptV1,
    ) -> Self::Published;
}
```

The only assignment kernel is:

```rust
fn publish_once<P: SealedPublicationPayloadV1>(
    prepared: PreparedPublicationV1<'_, P>,
) -> P::Published;
```

`PreparedBuilderExternalCommitV1::commit` remains the low-level owner of:

```rust
*current = self.session.candidate;
```

It is strengthened to return a non-forgeable
`BuilderPublicationReceiptV1 { brand, family }`. Existing legacy publication
is routed through `publish_once` without changing its public result contract.
RawDirect keeps a separate payload and typed result.

Production law:

```text
live Builder assignment implementation = 1
non-test direct caller of low-level commit = publish_once only
Legacy payload result = MirCompileResult
RawDirect payload result = RawPublishedInvocationV1
```

Do not add a second Raw assignment terminal or force RawDirect evidence into
the legacy bare-module/lifetime representation.

## PUBLICATION-TARGET0

Extract the current candidate readiness checks into one behavior-neutral
Builder quiescence helper:

```rust
fn check_builder_external_commit_quiescence(
    builder: &MirBuilder,
) -> Result<(), BuilderCommitReadinessErrorV1>;
```

It checks exactly:

```text
current_module              = none
current function            = none
current block               = none
function-owned state        = closed
current slot registry       = none
compilation context         = none
recursion depth             = 0
```

`ModuleBuilderInvocationSessionV1::prepare_module_session` reuses this helper
for its candidate. Raw publication reuses it for the live target. Persistent
configuration, import/plugin configuration, and unrelated stable Builder
state are not required to be empty.

The Raw prepared owner must retain `&mut MirBuilder` from successful preflight
until `publish(self)`:

```rust
PreparedRawPublicationV1<'target>
  -> publish(self)
  -> RawPublishedInvocationV1
```

This closes the target-preflight/assignment TOCTOU gap without a generation
counter, module clone, or rollback copy.

Failure retains the exact `PreparedRawExternalCommitV1`:

```rust
RejectedRawPublicationInvocationV1 {
    owner,
    stage,
    error,
}
```

Allowed exits are `stage(&self)`, `error(&self)`, and `discard(self)` only.

## PUBLICATION-RAW0

Add a Builder-side consuming opaque transition:

```text
RawExternalCommitModuleV1
  -> RawPublishedModuleV1
```

Neither carrier exposes:

```text
MirModule field
module_mut / into_module
Deref / DerefMut
AsRef / AsMut<MirModule>
caller-provided mutation closure
clone / rollback
```

Compiler success remains route-specific:

```rust
enum RawPublishedInvocationV1 {
    Script(RawScriptPublishedInvocationV1),
    App(RawAppPublishedInvocationV1),
}
```

The common core retains by value:

```text
ModuleInvocationTokenV1
RawPublishedModuleV1
RawPostprocessEvidenceV1
RawPublicationSealV1
  BuilderPublicationReceiptV1
```

Do not re-project the ledger, rescan the module inventory, infer the route from
symbols, recount helper/callable receipts, or rebuild verification evidence.
COMMIT0 already sealed those relations.

Raw verification law:

```text
Raw pre-transform Ok  -> publish with exact evidence
Raw pre-transform Err -> publish with exact reportable evidence
Canonical evidence    -> typed preflight rejection
```

## PUBLICATION-I0

Add exactly one compiler-internal entry:

```rust
impl MirCompiler {
    pub(in crate::mir) fn publish_raw_direct(
        &mut self,
        prepared: PreparedRawExternalCommitV1,
    ) -> Result<
        RawPublishedInvocationV1,
        RejectedRawPublicationInvocationV1,
    >;
}
```

The method performs only:

```text
borrow-only target/evidence preflight
-> PreparedRawPublicationV1 holding the target borrow
-> private infallible publish_once
-> typed RawPublishedInvocationV1
```

It must not contain a direct Builder assignment, module extraction,
`MirCompileResult` projection, legacy downgrade, public compile routing, or
fallback.

## PUBLICATION-G0

Add one focused guard and fixtures.

Required structural assertions:

```text
live Builder assignment implementation                         = 1
private publish_once production implementation                 = 1
PreparedBuilderExternalCommitV1::commit non-test direct caller = 1

PreparedRawExternalCommitV1 production consumer                = 1
RawPublishedInvocationV1 producer                              = 1
BuilderPublicationReceiptV1 producer                           = 1

target quiescence check before Raw owner consumption           = 1
fallible operation after PreparedRawPublicationV1 issue        = 0

RawExternalCommitModuleV1 bare compiler handoff                = 0
RawPublishedModuleV1 mutable/bare accessor                     = 0
module clone / rollback                                        = 0

manifest reprojection / module inventory rescan                = 0
route inference / helper-callable recount                      = 0
Raw verifier Err rejection producer                            = 0

RawPublishedInvocationV1 -> MirCompileResult producer          = 0
public ingress / executor / JSON / fastmem / selfhost / CUT0   = 0
old Raw publication new caller                                 = 0
retry / resume / fallback / catch_unwind                       = 0
```

Required fixtures:

```text
success: Script, Raw verification Ok
success: Script, reportable Raw verification Err
success: App callable-Main NotSelected
success: App callable-Main Selected
success: App with helper receipts

failure: live target current module/function/block open
failure: live target function state/slot registry/context/depth open
failure: non-Raw family
failure: foreign token/Builder/evidence brand
failure: canonical verification evidence

retention: every failure preserves the exact prepared Raw owner
atomicity: every failure leaves the live Builder unchanged
success: live Builder replacement count = 1
success: receipt brand/family equals published token/evidence
success: route/runtime/receipts/witness/parities/verification retained
```

## Files

Prefer new small siblings:

```text
ADD
  src/mir/compiler/publication_kernel.rs
  src/mir/builder/builder_publication_target.rs
  src/mir/builder/raw_root_physical/publication_terminal.rs
  src/mir/compiler/raw_root_publication.rs
  src/mir/compiler/raw_root_publication_p0.rs
  tools/checks/lib/
    cut0_i0_root0_raw_source0_lower_root_post0_publication0_guard.py

EDIT narrowly
  src/mir/builder/module_invocation_session.rs
  src/mir/compiler/external_commit.rs
  src/mir/compiler/raw_root_external_commit.rs
  src/mir/builder.rs
  src/mir/compiler/mod.rs
  CURRENT_STATE.toml
  docs/tools/check-scripts-index.md
```

Do not add PUBLICATION0 implementation to the existing 437-line
`postprocess_terminal.rs` or grow `compiler/mod.rs` beyond registration and a
thin method export. Every modified/new source and check file must stay below
800 lines.

## Verification

```bash
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_publication_p0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q external_commit_p0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_session_p0 --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_publication0_guard.py
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_commit0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The semantic-manifest check must be green before PUBLICATION0 closes, but its
generated refresh is a separate maintenance commit. Broad unrelated baseline
failures are not hidden or normalized by this row.

## Follow-on migration queue

The compiler-wide cleanliness assessment is recorded as this ordered queue:

```text
PUBLICATION0-S0/G0
  sole internal Raw publication and typed evidence

PUBLIC-ADAPTER0
  RawPublishedInvocationV1 -> MirCompileResult projection only

INGRESS0
  public compiler ingress, AST-JSON, runtime Program(JSON v0) policy

CUT0
  measured all-family activation and direct build_module caller census

RETIREMENT0
  old Raw finalization/run_raw/ledger-root evidence/hard-coded inventory
  plus caller-zero dead-code scaffolding cleanup

E2E0
  full end-to-end route and regression gate
```

`SEMANTIC-MANIFEST-MAINT0` is repository hygiene and remains separate from
this semantic queue. `LINE-BUDGET-MONITOR0` is enforced by each row guard;
there is no current emergency split because all `src/mir` files are below 800
lines.

## Non-claims

```text
RawPublishedInvocationV1 -> MirCompileResult adapter = 0
public Raw ingress / compile_with_source cutover      = 0
runtime/executor/AST-JSON/Program(JSON v0) wiring     = 0
old Raw chain deletion                                = 0
canonical publication behavior change                = 0
fastmem/selfhost production consumer                  = 0
CUT0 activation                                       = 0
typed panic retention                                 = 0
```

## Proof budget / sunset

```text
ceremony_tier = T2 (new live publication authority and result boundary)
sunset_id = RAW-PUBLICATION-SUNSET-001
proof_inventory_before = typed RawDirect prepared owner, opaque module, complete evidence
new_proofs = one target-held prepared publication, one assignment receipt, one typed published result
retired_or_merged_proofs = no Raw proof retired in this row
sunset_row = RETIREMENT0 after PUBLIC-ADAPTER0 / INGRESS0 / CUT0
retire_when = old Raw finalization, run_raw, and ledger-root-only evidence have non-test caller zero
budget_repayment_evidence = one shared publication kernel, one Raw lane guard, one measured retirement ledger
```

## Internal order

```text
SEMANTIC-MANIFEST-MAINT0 (separate prerequisite commit)
-> PUBLICATION-KERNEL0
-> PUBLICATION-TARGET0
-> PUBLICATION-RAW0
-> PUBLICATION-I0
-> PUBLICATION-G0
```
