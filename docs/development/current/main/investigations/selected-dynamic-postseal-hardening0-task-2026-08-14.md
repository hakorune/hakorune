---
Status: Hardening implementation tail closed for task selection; LiveBlocker=0 and CutoverBlocker=0; one latest-HEAD detached integration return gate remains before explicit Loop M8 S6C retarget; latent P1/P2, G3, and live cutover are parked
Date: 2026-08-14
Parent: docs/development/current/main/investigations/dynamic-v2-w6-production-activation-task-2026-08-13.md
Resume-after: docs/development/current/main/investigations/llvmlite-keep0-ret0-inventory-task-2026-08-14.md
Scope: selected Dynamic admission, post-seal lifecycle, publication, Boundary reachability, and MirBuilder safety hardening before G3 archive movement or main integration
---

# SELECTED-DYNAMIC-POSTSEAL-HARDEN0

This card accepts the post-W6 audit without reopening Recipe, CheckedCallOut
meaning, Completion, or DraftSeal semantics.  A broader worker audit found
that the selected infrastructure was not yet an end-to-end executable route:
the Rust runner and Boundary now share a strict PyVM/verification and
launch/helper identity fence, while object/link/receipt/launch evidence was open.
The candidate route now proves those stages; live publication remains open under
the B3 bundle owner.  Upstream declaration-mode admission and one safe lexical-scope owner
also required correction.  The selected canonical core remains the authority;
these rows make every boundary from admission through launch fail closed.

## Six-line brief

```text
Decision: stop the Dynamic hardening tail: live/cutover blocker count is zero; park latent P1/P2 findings, rerun the latest-HEAD detached integration gate once, then return explicitly to the Loop product frontier.
Source authority + canonical issuer: the landed selected semantic package, canonical session/DraftSeal, Boundary artifact fence, and the unchanged selected candidate/integration gates.
Non-authority: grep-only suspicion, no-caller LocalSSA wrappers, future provider/platform generalization, LOC/style findings, llvmlite output, or another audit score.
Fail-fast boundary: only a named live/candidate reproducer, exact selected-cutover gate failure, or reachable UB/corruption/irreversible effect may reopen this card before Loop.
Smallest next slice: MIRBUILDER-LATEST-HEAD-INTEGRATION-R0 in a clean detached worktree; classify every red, then retarget CURRENT_STATE to JOINIR-LOOP-M8-LOOPV0-SCANS-S6C.
Non-claims: no LocalSSA/provider/platform/line-budget implementation, live cutover, G3 archive/delete, fallback/retry, Loop implementation, or second writer lane.
```

## Closed audit boundary

The landed chain now covers declaration mode, lexical safety, clone scrubbing,
post-seal immutability, strict verification, runner reachability, launch/helper
identity, attempt-unique artifact publication, Boundary/compat feature
ownership, physical census/wire parity, and atomic lease publication. Git
history and the focused tests own their detailed evidence. The C ABI remains a
thin `uint64_t lease_token -> uint32_t status` bridge to the Rust lease owner;
it owns no lease, CFG, wire, or semantic meaning. New findings use the
hardening admission rule below and do not create another card or guard family.

### GUARD-SURFACE-CONSOLIDATION-D0

Decision: consolidate the guard surface as one BoxShape organization task;
new per-finding guard files are frozen.
Source authority + canonical issuer: the existing family guard, its focused
behavior test, and the manifest-backed check inventory.
Non-authority: grep-only caller counts, mutation-count zero, historical prose,
or a generated always-green wrapper.
Fail-fast boundary: an unclassified check, missing owner, or guard migration
that changes behavior remains retained and blocks closeout.
Smallest next slice: read-only inventory and owner/disposition census; migrate
only after selected P0 barriers and integration evidence are green.
Non-claims: no compiler behavior, semantic receipt, fallback, source move, or
proof deletion.

This is the single organization task for the findings below.  It is a
read-only inventory first; it does not open a new semantic row, create a
per-finding shell script, or delete proof.

```text
live P0 behavior/authority  -> owning existing family guard + focused test
source/ABI/CFG parity        -> callslot/AOT/VM family guard
post-seal/verification       -> in-place replacement + pointer guard
artifact/platform/archive    -> manifest-backed evidence, after P0 close
historical wrappers          -> archive/delete only after caller census
```

Initial source-backed census (2026-08-14, read-only) records 3,654 tracked
`tools/checks` paths: 3,283 shell, 175 Python, 67 Rust, 34 JSON, 17 TOML,
and the remaining support/fixture paths.  The public shell surface is 3,009
entries, including 1,567 `k2_wide_*` paths and 241 manifest implementation
paths.  `guard_rows.toml` currently has 102 rows across `pilot` (102),
`hako-alloc-closeout` (74), `quick-static` (14), and `pure-first-route` (1);
the profile counts overlap by design.  Exact shell/Python content duplicates
are zero, so blind deduplication is not an accepted disposition.

The existing manifest inventory also exposes a separate baseline contract
drift: its hako-alloc closeout rows expect executable wrappers/implementations,
but many tracked paths are mode `0644`, and the inventory guard itself is not
executable.  This is retained as `unknown_retain`/owner-documentation debt;
mass chmod or wrapper deletion is explicitly out of this D0.  The next
sub-row must classify every tracked path against the six allowed dispositions,
starting with the indexed stable entries and manifest-backed families, then
record unknown rows rather than infer deletion from a grep or executable bit.

The pre-cut index census had 573 human-table rows and a required
machine-readable compatibility block with 2,017 historical names (16 also
appeared in the table).  The loaded guard manifest had 102 rows/command paths
across overlapping profiles.  The D0 inventory therefore treated the table
and compatibility block as separate authorities and retained all unclassified
paths.

The first generator receipt is now reproducible with
`python3 tools/docs/guard_surface_inventory.py --check`: 3,654/3,654 tracked
paths are represented exactly once: 489 `stable_public_entry` and 315
manifest-backed rows split into 103 `family_manifest_case` and 212
`focused_behavior_test`, plus 2,850 `unknown_retain` rows; index and manifest
source gaps are both zero.  No row is assigned `historical_archive` or
`delete_after_equivalent_coverage`.  The optional JSON output is diagnostic
only and is not a second authority or a retirement approval.

`GUARD-PUBLIC-ENTRY-CUT-R0` now keeps 19 reusable human-facing entries in
`docs/tools/check-scripts-index.md` (118 lines total), while the compatibility
block is byte-for-byte unchanged.  The post-cut inventory reruns as 3,654
unique rows: 19 `stable_public_entry`, 103 `family_manifest_case`, 212
`focused_behavior_test`, and 3,320 `unknown_retain`; source gaps remain zero.
No tracked script moved or was deleted, and no check command changed.

The inventory records one owner, one caller/profile, one evidence kind, and
one disposition for every tracked check.  `stable_public_entry`,
`family_manifest_case`, `focused_behavior_test`, `historical_archive`,
`delete_after_equivalent_coverage`, and `unknown_retain` are the only allowed
classes.  Unknown rows remain retained; a green grep, mutation count, or
`Option::None` is never execution evidence.  Migration/retirement waits until
the selected P0 DAG is green and never touches unrelated dirty compiler files.

`dynamic_v2_aot_activation_authority_guard.sh` is green at this HEAD, but it
checks the existence/count of the downstream selected call rather than control
dominance; it therefore does not prove that Boundary is reachable.  Local
structural green remains non-production evidence until the execution-shaped
negative/positive tests above exist.

### Audit normalization and task DAG (2026-08-14)

The latest worker/pro review (including C ABI, PyVM, verifier, artifact,
archive, and guard-surface audits) is fully absorbed here; it creates no new
card and no new per-finding guard.  The execution order is:

```text
closed: mode fence + lexical transaction + linear slots
  -> closed: post-seal mutator fence + strict final verification
  -> closed: selected runner dominance (PyVM remains retired)
  -> closed: C dual launch/helper view + CheckedCallOut physicalizer + candidate object/link/receipt/launch
  -> closed: B3 attempt-unique artifact bundle publication/consumption and path-bound launch fence
  -> closed: B4 llvm-boundary vs llvmlite-compat feature ownership
  -> closed: selected post-seal mutation fence + strict verification retention + selected runner reachability
  -> CURRENT CLOSEOUT: MIRBUILDER-LATEST-HEAD-INTEGRATION-R0
  -> RETURN: JOINIR-LOOP-M8-LOOPV0-SCANS-S6C (T2 design stop)
  -> parked G3 oracle/archive/deletion DAG
```

The parked P1 bundle owns source-fact/header projection, callout effect and
CFG/wire parity, physical-ID issuance, lease rollback, LocalSSA fail-closed
terminals, activation disposition typestate, ProviderSlot typed carrier,
platform fencing, stale-name retirement, and the reusable line census.  The
guard organization task owns inventory/manifest migration and wrapper
retirement only after the selected P0 DAG is green.  PyVM is not restored:
explicit requests reject, absence skips the retired stage, and selected
Boundary is reached once; `mutation_count == 0`, green grep, and `None` are
never execution evidence.  The C ABI remains only the thin lease-status
bridge.  No fallback, retry, VM consumer, source move, deletion, or main
integration is claimed by this task map.

## Immediate baseline debt

One stable guard is red at this HEAD and at both `HEAD^` and the G0-G2 close
commit `8c9b6956d9`:

```text
bash tools/checks/dynamic_v2_text_scan_admission_authority_guard.sh
-> unexpected caller: src/mir/builder/normal_callable_semantic_loan_port.rs
```

This was accepted allowlist drift: the guard encoded the pre-W6
definition-plus-test caller set and did not admit the landed single production
caller. B0 now admits the resolved-lowering facade plus exactly one definition,
one focused test caller, and `normal_callable_semantic_loan_port.rs` as the
named production caller; a second production caller remains fatal.
`dynamic_v2_aot_activation_authority_guard.sh` and the current pointer guard
remain green.

Owner documentation is also stale. In particular,
`src/mir/builder/resolved_lowering/README.md` had described End, profile close,
DraftSeal, publication, and production selection as closed. B0 updates the
current graph to distinguish the landed candidate/unpublished handoff from
still-closed live publication and Boundary execution. The focused completion
red is now classified below with its exact parent reproduction.

## Ordered hardening DAG

```text
DYN-PROD-BASELINE-R0
  -> LEXICAL-SCOPE-SAFE-TRANSACTION-R0
  -> DYN-ADMISSION-MODE-FENCE-R0
  -> SELECTED-DYNAMIC-LINEAR-SLOT-FENCE-R0
  -> SELECTED-DYNAMIC-POSTSEAL-IMMUTABILITY-D0
       -> SELECTED-DYNAMIC-POSTSEAL-IMMUTABILITY-R0
       -> SELECTED-DYNAMIC-STRICT-VERIFIER-POLICY-R0
       -> LLVM-NORMAL-COMPILE-VERIFICATION-FENCE-R0
  -> SELECTED-DYNAMIC-RUNNER-DOMINANCE-R0
  -> DYN-BOUNDARY-SELECTED-HELPER-IDENTITY-D0
       -> ...-PRODUCTION-SHAPE-FIXTURE-R0
       -> ...-PHYSICALIZER-R0
       -> ...-END-TO-END-R0
  -> DYNAMIC-V2-STATIC-ARTIFACT-BUNDLE-PUBLICATION-D0
       -> ...-BUNDLE-PREPARE-I0
       -> ...-BUNDLE-COMMIT-I0
       -> ...-ROOT-CONSUME-R0
  -> LLVM-BOUNDARY-COMPAT-OWNERSHIP-D0
       -> LLVM-BOUNDARY-EXECUTOR-S0
       -> LLVM-BOUNDARY-COMPAT-FEATURE-R0
  -> MAIN-INTEGRATION-EVIDENCE-R0
  -> resume LLVMLITE-ORACLE-COVERAGE-D0

DYN-PROD-BASELINE-R0
  -> GUARD-SURFACE-CONSOLIDATION-D0       # read-only inventory may proceed

all selected P0 rows + accepted guard D0
  -> GUARD-FAMILY-MANIFEST-MIGRATION-R0
  -> GUARD-HISTORICAL-RETIRE-R0
```

No row may be combined with another semantic family or with G3 source
movement. Source files are split at 760 lines and hard-stop at 800; the three
existing near-limit owners (`builder.rs` 794, `recursive_child_lowering.rs`
794, and `function/metadata.rs` 787) receive no new responsibility without a
prior physical split.

Do not activate the runner reachability fix before the post-seal immutability,
strict verifier, and compile-result fences are closed.  Making a dead route
reachable before those barriers would expose an unverified candidate rather
than repair production.

The guard inventory/design sibling does not block P0 implementation.  Its
file movement, wrapper retirement, and quick-profile recut wait until the
selected P0 barriers are green so guard cleanup cannot hide a live failure.

## Row B0: DYN-PROD-BASELINE-R0

Classification: BoxShape / evidence closeout. Behavior and accepted forms are
unchanged.

Change:

1. Update the TextScan admission guard to require exactly one definition, one
   focused test caller, and the one named production caller. Reject every
   second production caller.
2. Replace stale pre-W6 wording in the guard/check index and current owner
   README sections with the landed production graph.
3. Run the focused `completion` command in a clean worktree. If green, remove
   the stale known-red statement. If red, run the same command at the exact
   recorded parent and record SHA, command, and both outcomes before labeling
   it baseline debt.
4. Do not edit the eleven unrelated compiler files currently dirty in the
   shared worktree.

Acceptance:

```text
TextScan admission definition / focused caller / production caller = 1 / 1 / 1
second production caller negative                               = reject
TextScan admission guard / AOT guard / pointer guard            = green
owner README current graph                                       = landed route
completion red                                                   = green or exact parent-classified
route / Recipe / MIR / fixture changes                           = 0
```

Closeout evidence (2026-08-14):

```text
guard: bash tools/checks/dynamic_v2_text_scan_admission_authority_guard.sh = ok
guard: bash tools/checks/dynamic_v2_aot_activation_authority_guard.sh      = ok
guard: bash tools/checks/current_state_pointer_guard.sh                     = ok
completion clean HEAD b0629d7b7b: cargo test -q --lib completion = 107 passed, 1 failed
completion parent b69f5e11fe: cargo test -q --lib completion = 107 passed, 1 failed
shared failure: canonical_physical_completion_p0::compiler_bridge_drains_a_plus_single_route
  -> ReturnValueTypeMissing(ValueId(12)); baseline debt, not a B0 regression
```

## Safety preemption: LEXICAL-SCOPE-SAFE-TRANSACTION-R0

Classification: BoxShape safety correction.  Scope chronology and accepted
source forms do not change.

Change:

1. Replace the lifetime-free `*mut MirBuilder` Drop guard with one
   escape-proof scoped callback/transaction owner.
2. Close KeepAlive emission and scope restoration as a fallible transition;
   a normal-path KeepAlive failure is returned rather than discarded.
3. Restore state exactly once on success, typed error, and panic unwind.  No
   safe caller may let the scope owner outlive its Builder.

Done:

```text
lexical_scope.rs production unsafe / raw pointer lifetime    = 0 / 0
success / Err / panic restoration tests                      = 1 / 1 / 1
injected KeepAlive close failure                             = typed error
production callers migrated to scoped transaction           = 4
scope chronology / Recipe / CFG / accepted source delta      = 0
```

Evidence: `cargo check -q --lib`, lexical-scope focused tests (6 passed), and
block-driver focused tests (6 passed).  The legacy Drop guard is isolated in a
`#[cfg(test)]` compatibility module; production lowering has no raw pointer or
Drop-owned scope.  The existing UnsafeOrFFI inventory guard remains a known
baseline-red check because its parent task-order fixture is stale; that red is
not attributed to this row.

Stop if an escape-proof API cannot preserve current callers without changing
scope or KeepAlive meaning; return to a bounded ownership design instead of
adding another raw-pointer guard.

## Admission preemption: DYN-ADMISSION-MODE-FENCE-R0

Classification: bounded Dynamic admission correction.  It removes a false
selected classification; it does not add an accepted Dynamic shape.

Change:

1. Use the resolved batch row's declaration mode as the sole mode authority.
2. Lend only selected `StaticBoxMethod` rows to Dynamic admission.
3. Keep Instance/TopLevel rows under ordinary ownership without probing the
   Dynamic source/parameter contract and without raw-AST mode checks.

Done:

```text
current static skip_while fixture                         = selected
same-shape Instance / TopLevel Dynamic admission          = 0 / 0
dynamic-instance-route / MissingDynamicParameterContract  = not reached
ordinary fallback newly introduced                       = 0
```

Stop if ordinary ownership cannot be preserved from the existing resolved
mode.  Do not teach A-prime or the emitter to lower Instance/TopLevel as an
incidental repair.

Evidence (2026-08-14): the package issuer now checks the existing resolved
`ResolvedCallableDeclarationModeV1` row before Dynamic admission and lends only
`StaticBoxMethod`.  The parser-scan static fixture remains selected; the
instance-shaped scan fixture remains `ValidUnselected`; the mixed top-level +
selected fixture remains one complete batch with only the selected static row
entering Dynamic.  The existing TextScan admission guard now fixes this mode
gate in the sole package issuer.  Focused package tests: 18 passed; `cargo
check -q --lib` and the selected/AOT/current pointer guards remain green.

## Row B1: SELECTED-DYNAMIC-LINEAR-SLOT-FENCE-R0

Classification: BoxShape. It exposes existing lifecycle state without issuing
a new semantic product.

Use one private observation vocabulary:

```text
LinearSlotObservation<'a, T>
  Empty
  Occupied(&'a T)
  Scrubbed

FunctionMetadata selected-pair observation
  Empty + Empty       -> Ordinary
  Occupied + Occupied -> Selected borrowed pair
  any Scrubbed        -> fatal lifecycle reject
  every partial pair  -> fatal lifecycle reject
```

The slot observation stays private to the metadata/census boundary. Downstream
code does not receive two freely pairable raw slot states. JSON emission and
route census must share the aggregate pair observation; a scrubbed selected
clone may never silently omit both keys and enter an ordinary route.

Required tests:

```text
ordinary clone                 -> Ordinary
occupied selected pair         -> Selected
selected module/result clone   -> fatal reject, ordinary fallback not reached
receipt-only/admission-only     -> fatal partial-pair reject
one or both Scrubbed            -> fatal lifecycle reject
second install/take             -> existing one-shot rejection preserved
```

Guard the sole production census caller and forbid selected-route module/result
clone before census. Keep ordinary compatibility clone behavior unchanged.

Closeout evidence (2026-08-14): the fixture now distinguishes the two linear
states: `Empty -> Empty` and `Occupied -> Scrubbed`; the production slot and
pair observer remain unchanged. The focused slot tests (2), `cargo check -q
--lib`, fmt, AOT/text/physical-input/pointer guards, and diff check are green.
B1 closeout is restored; the next design row is the queued disposition
typestate, with selected production still 0/old=1 and no fallback/retry.

## Post-seal design: SELECTED-DYNAMIC-POSTSEAL-IMMUTABILITY-D0

The compiler currently installs the selected DraftSeal/receipt and then runs
the Legacy module finish schedule.  Optimizer, boundary refresh, optional RC
insertion, semantic metadata refresh, and callsite canonicalization all take a
mutable module; the pre-transform verifier result can remain `Err`, and the
LLVM adapter discards it by returning only `.module`.  Ambient compatibility
variables can also skip or weaken verifier checks.

This D0 must fix one ordering before implementation:

```text
all legal unpublished physical normalization
  -> selected DraftSeal / metadata install
  -> selected function immutable
  -> env-independent strict whole-module verification
  -> external commit
```

The implementation series is:

1. `SELECTED-DYNAMIC-POSTSEAL-IMMUTABILITY-R0`
   - no optimizer/RC/refresh/canonicalizer mutable loan reaches a sealed
     selected function;
   - ordinary compatibility functions retain only their explicitly admitted
     pre-publication schedule.
2. `SELECTED-DYNAMIC-STRICT-VERIFIER-POLICY-R0`
   - selected CFG, predecessor, definition, dominance, PHI, metadata, receipt,
     and site-plan parity are always checked;
   - `NYASH_STAGEB_DEV_VERIFY`, `NYASH_VERIFY_ALLOW_NO_PHI`, and
     `NYASH_MIR_NO_PHI` cannot weaken this selected barrier.
3. `LLVM-NORMAL-COMPILE-VERIFICATION-FENCE-R0`
   - consume the final verification result before external Builder commit and
     before route census/JSON/object/child effects;
   - forbid adapters that return `Ok(compile_result.module)` while silently
     dropping verification evidence.

Any injected selected mutation or final verification error must yield external
commit/backend/launch counts `0/0/0`.  Do not issue a second semantic receipt;
the existing canonical MIR and verifier are the authorities.

Closeout evidence (2026-08-14): selected finish now skips generic post-seal
mutators, uses an env-independent strict verifier, and consumes verification
before external commit.  The LLVM adapter preserves `MirCompileResult`; selected
routes consume `into_verified_module()` and bypass Method-ID injection while
ordinary routes remain unchanged.  Focused schedule/strict-policy tests,
`cargo check --lib`, fmt, AOT/text/pointer guards, and `git diff --check` are
green; selected production remains `0/old=1`.  B2 runner dominance is next.

## Row B2: SELECTED-DYNAMIC-RUNNER-DOMINANCE-R0

Classification: BoxShape production-route correction.  PyVM remains retired
from selected production; its nonconsumer fence must not terminate the
Boundary route.

```text
selected metadata pair
  -> selected PyVM request? typed reject
  -> read-only legacy-callsite scan
  -> Boundary path exactly once

ordinary module
  -> existing explicit PyVM compatibility decision
  -> existing compatibility injector stage exactly once
```

The selected branch must occur before `PyVmExecutorBox`, object-output, the
ordinary harness, and mock fallback.  It never constructs a fatal error merely
to pass through the ordinary PyVM match.  The selected scan reuses the existing
canonical legacy callsite classifier and bypasses `MethodIdInjectorBox`
entirely.

The relevant invalid shape is `Call { callee: None }`, not a retired
`MirInstruction::BoxCall`. A valid `Callee::Method` remains accepted. Selected
invalid shape rejects before JSON, child process, object, fallback, or retry.

Required execution evidence:

```text
selected + PyVM not requested  -> Boundary exactly 1
selected + PyVM requested      -> typed reject; PyVM/Boundary/object/child 0
selected Boundary failure      -> ordinary harness/mock fallback 0
selected MethodId mutator loan -> 0
```

Long term, retire the no-op injector wrapper/plan/report fields after a caller
census. If future ordinary physical normalization is required, it belongs to
the unpublished compiler postprocess before external commit; method meaning
belongs earlier in source/Facts/Recipe/canonical lowering.

Closeout evidence (2026-08-14): selected now skips PyVM when unrequested,
rejects an explicit PyVM request, scans the existing legacy-callsite classifier
before backend effects, and reaches the single selected Boundary caller without
ordinary harness/mock fallback.  The Rust pre-backend identity fence adds
exactly one zero-argument launch, one distinct four-argument selected helper,
the production-shaped dual-function JSON fixture, and nine focused
  negative/positive tests; C dual-view/physicalization is now landed and
  end-to-end launch remains open.  Focused route/identity tests, LLVM feature and
default `cargo check --lib`, AOT/text/pointer guards, and diff check are green;
selected production remains `0/old=1`.

## Boundary design: DYN-BOUNDARY-SELECTED-HELPER-IDENTITY-D0

The production module has two different physical identities and must keep
them distinct:

```text
launch entry
  = exact one zero-argument main / ny_main

selected callable
  = exact one metadata-bearing ParserScanLoopBox.skip_while/4 helper
```

Rust currently chooses the selected route from the helper metadata, while the
Boundary C validator and artifact descriptor inspect the launch entry.  The
helper cannot be renamed or aliased to `ny_main`, because the runtime invokes
`ny_main()` with zero arguments.

The bounded series first landed a production-shaped `main + selected helper`
fixture, then one C owner censused the exact helper and sent that function to
the existing CheckedCallOut physicalizer, and finally proved object -> link ->
receipt -> zero-argument launch end to end.  The fixture, C dual-view/
physicalizer, helper-aware descriptor projection, and candidate launch test are
landed; only B3 live bundle publication remains.  Missing/duplicate metadata,
metadata on the launch entry, missing/duplicate/nonzero-argument main, helper
arity drift, generic CheckedCallOut fallback, or a descriptor sourced from
main all reject before publication.  Metadata copying and by-name reselection
are forbidden.

The first child `...-PRODUCTION-SHAPE-FIXTURE-R0` is landed as a test-only
dual-function JSON shape fixture plus identity negatives.  The
`...-PHYSICALIZER-R0` child now owns one borrowed C dual view, forwards the
already-selected helper into the existing physicalizer, preserves ordinary
entry behavior, and rejects helper/launch identity drift before object output.
`...-END-TO-END-R0` owns link/receipt/launch evidence.  No launch-to-helper
arguments, fake zero-argument stub, metadata copy, or alias is emitted.

## Row B3: DYNAMIC-V2-STATIC-ARTIFACT-BUNDLE-PUBLICATION-D0

The accepted target is an attempt-unique same-filesystem bundle:

```text
candidate bundle/
  program
  receipt.json
    -> all fallible serialization/write/hash/descriptor/census work
    -> one directory rename
published bundle/
  program
  receipt.json
    -> root hashes actual program and validates the receipt
    -> launch fence
```

`StaticAotArtifactPublicationTxnV1` remains the sole child issuer/committer.
The root validator is a consumer and physical co-check, not a second artifact
authority. Fixed executable plus compensating two-file renames may be kept only
if the row explicitly calls the result rollback-capable rather than atomic;
the directory-generation design is preferred because it closes the crash
window with one visibility transition.

### B3 implementation rows

1. `...-BUNDLE-PREPARE-I0` (closed)
   - **closed:** prepare executable and receipt bytes inside one invisible,
     attempt-unique candidate directory; all child fallible work completes
     before publication and stale/PID-only reuse is not accepted.
2. `...-BUNDLE-COMMIT-I0` (closed)
   - publish by exactly one same-filesystem directory rename;
   - collision/rename failure leaves no final bundle and preserves prior
     artifact; commit has no subsequent child write.
3. `...-ROOT-CONSUME-R0` (closed for fence/cleanup; live cutover open)
   - require exact expected bundle and regular executable path;
   - hash the actual executable and compare with receipt;
   - co-check input, descriptor, site/ABI/wire/PlanStamp, and exact symbol
     census rather than merely nonzero values;
   - issue one path-bound fence and consume it through one launch/cleanup
     terminal; selected production remains new=0/old=1.

Negatives include receipt write/rename failure, executable mutation, valid
64-hex but wrong digest, fictitious `1/1/1/1` census, stale attempt, path drift,
bundle collision, and launch count zero after any reject.

## Row B4: LLVM-BOUNDARY-COMPAT-OWNERSHIP-D0

This is backend build/physical-owner SSOT, not MIR semantic SSOT. Freeze this
target feature graph before moving llvmlite sources:

```text
llvm-boundary
  Boundary C ABI, ny-llvmc, selected artifact receipt/execution

llvmlite-compat
  Python harness, --driver harness, explicit oracle/compat jobs

llvm
  compatibility alias for llvm-boundary only

llvm-harness
  temporary deprecated umbrella during migration only;
  production code must not cfg on it after recut
```

`LLVM-BOUNDARY-EXECUTOR-S0` is closed as a behavior-neutral physical split:
`BoundaryExecutorBox` and the selected Boundary process helper now live apart
from `harness_executor`. `LLVM-BOUNDARY-COMPAT-FEATURE-R0` is now closed:
the selected caller uses `llvm-boundary`, Python owners use `llvmlite-compat`,
and `llvm-harness` remains only as a migration umbrella.

Acceptance:

```text
selected Boundary caller                                      = 1
production cfg(feature = "llvm-harness")                      = 0
Boundary subtree Python/llvmlite/harness-env references       = 0
llvm alias includes llvmlite-compat                            = 0
featureless --driver harness                                  = typed pre-effect reject
explicit compat feature + driver Python spawn                 = 1
selected fallback / retry                                     = 0 / 0
```

## Queued post-B4 P0 hardening

These are one shallow queue, not new semantic routes or new guard families.

```text
SELECTED-DYNAMIC-POSTSEAL-MUTATION-FENCE-R0 [closed 2026-08-14]
Decision: selected sealed MIR bypasses MethodIdInjector and every later mutable optimizer/RC/canonicalizer seam.
Source authority + canonical issuer: selected receipt/admission census plus the existing unpublished postprocess owner.
Non-authority: mutation_count=0, runner clones, legacy “BoxCall” wording, or post-publication repair.
Fail-fast boundary: selected unresolved legacy Call or any post-seal mutable touch rejects before JSON/backend spawn.
Closeout: the selected finish branch is statically proven to return before every generic post-seal mutator; selected verification is consumed before external commit; selected Method-ID injection remains bypassed; ordinary compatibility is unchanged.
Non-claims: no method resolution, new receipt, fallback, retry, or production cutover.

SELECTED-DYNAMIC-STRICT-VERIFY-GATE-R0 [closed 2026-08-14]
Decision: selected compilation consumes verification results under an env-independent strict policy before any backend effect.
Source authority + canonical issuer: MirCompileResult verification result and existing selected final verifier.
Non-authority: `NYASH_STAGEB_DEV_VERIFY`, no-PHI compatibility knobs, JSON emission, or child receipt text.
Fail-fast boundary: verification error, dominance/PHI drift, or module metadata mismatch blocks object, Boundary, PyVM, and launch.
Closeout: `MirVerifier::new_strict()` is selected-only, compatibility env switches do not weaken it, and `normal_default_pipeline` consumes the result before external commit; focused strict-policy, compiler, runner, and authority-guard evidence is green.
Non-claims: ordinary compatibility verifier policy, semantic Recipe changes, or fallback/retry.

SELECTED-DYNAMIC-RUNNER-REACHABILITY-R0 [closed 2026-08-14]
Decision: selected input skips retired PyVM and reaches the sole Boundary executor; explicit PyVM is typed pre-effect reject.
Source authority + canonical issuer: selected metadata census and Boundary executor owner.
Non-authority: early fatal skip arms, `SMOKES_USE_PYVM`, harness feature names, or ordinary mock fallback.
Fail-fast boundary: selected route never emits PyVM error/child/object fallback before Boundary dispatch.
Closeout: execution order is guard-proven as census -> strict result -> PyVM decision -> object reject -> Boundary dispatch; the selected dispatch region contains only the Boundary owner, while ordinary compatibility retains its existing harness/fallback path. Focused stage/legacy-callsite tests and AOT authority guard are green; selected production remains new=0/old=1.
Non-claims: PyVM deletion, llvmlite archive movement, or new selected MIR shape.
```

## Guard surface consolidation task

Status: queued design sibling.  Immediate rule: new task-specific top-level
guard scripts are frozen; accepted P0 rows extend an existing reusable family
guard or add a real focused behavior test.

### Measured baseline

The 2026-08-14 tracked census is:

```text
tools/checks tracked files                         = 3,654
tracked shell scripts                              = 3,283
guard-named scripts                                = 2,899
tracked shell lines                                = 345,902
dev_gate quick steps                               = 66
check-scripts index lines / stable entries         = 118 / 19
compatibility ledger entries                       = 2,017
scripts with no literal src/ owner (heuristic)     = 1,793
inventory family rows / focused behavior rows     = 103 / 212
exact duplicate script contents                    = 0
```

This is a topology problem rather than a disk-size problem.  Many scripts
encode one historical card token or exact prose row, while the daily public
surface and product behavior are much smaller.  Exact duplicate count zero
means the cleanup must extract parameters into manifests; blind deduplication
will not work.

### GUARD-SURFACE-CONSOLIDATION-D0

Classification: design/census only.  It changes no compiler behavior and
deletes no proof.

Decision inputs:

1. Generate one source-backed inventory row for every tracked check with:
   owner family, public callers/profiles, active-card dependency, behavior vs
   source-authority vs codegen vs documentation-only class, unique evidence,
   and proposed disposition.
2. Classify each row exactly once:

```text
stable_public_entry
family_manifest_case
focused_behavior_test
historical_archive
delete_after_equivalent_coverage
unknown_retain
```

3. Treat production graph, source authority, codegen parity, and executable
   behavior as evidence.  A historical card's status/prose is not product
   authority.
4. Fix the exact current P0 guards in their owning rows.  Do not count a green
   grep/caller census as execution reachability evidence.

Done:

```text
inventory coverage / duplicate classification = 100% / 0
unknown rows                                    = explicitly retained
deletion / move / quick-profile behavior change = 0 / 0 / 0
selected P0 guard additions                     = new top-level scripts 0
```

### Implementation series after selected P0 close

#### GUARD-PUBLIC-ENTRY-CUT-R0 (queued design)

```text
Decision: shrink only the human-facing navigation; retain the machine-readable
compatibility ledger and every unclassified check.
Source authority + canonical issuer: check-scripts-index's stable table,
guard_rows.toml/proof_apps.toml manifests, and the source-backed inventory
generator; these are observed, not replaced by a new guard.
Non-authority: filename counts, grep-only caller guesses, executable mode, or
historical prose may not authorize retirement.
Fail-fast boundary: missing owner/disposition, manifest/index drift, or an
unmapped compatibility name keeps the row unknown_retain and blocks removal.
Smallest next slice: emit one generated owner/disposition inventory and use it
to propose a <=50-entry human index without changing any check behavior.
Non-claims: no script move/delete, manifest semantics change, new guard,
compiler change, fallback, archive publication, or llvmlite retirement.
```

#### GUARD-FAMILY-MANIFEST-MIGRATION-R0 (NoSafeSlice; parked)

```text
Decision: migrate only manifest-backed families through the existing generic
runner; keep wrapper names stable until one equivalent behavior owner exists.
Source authority + canonical issuer: guard_rows.toml and its includes,
proof_apps.toml and its includes, plus run_row_guard.sh/run_proof_app.sh.
Non-authority: filename prefixes, executable mode, historical card prose, or
grep-only caller counts cannot authorize a wrapper move or deletion.
Fail-fast boundary: duplicate/missing manifest IDs, command-path drift, missing
owner/profile, or mode mismatch remains an explicit retained baseline error.
Smallest next slice: classify the 103 family rows and 212 proof-app rows, then
convert one hako-alloc family only after its positive/negative behavior tests
and wrapper parity are observed.
Non-claims: no script deletion, new guard, compiler behavior, fallback,
archive publication, or llvmlite retirement.
```

The current manifest inventory is intentionally a named baseline: 102 loaded
rows (74 `hako-alloc-closeout`) report 44 public closeout wrappers outside the
manifest and widespread non-executable wrapper/implementation modes.  This
does not authorize chmod, wrapper deletion, or manifest invention; the family
row remains open until one owner-backed parity batch is selected.
The first executable closeout candidate also fails on a required child guard
whose mode is `0644`; this confirms that wrapper/implementation mode alone is
not a complete migration proof.  The child dependency closure must be named
before any mode normalization, and the current family stays retained.
The one closeout row whose wrapper/implementation/proof runner are already
executable (`hako-alloc-id-brand-first-pilot-closeout`) instead stops on its
semantic next-row selection (`MIMAP-145A` is not `selected current`).  It is
therefore not a safe migration candidate either.
An inventory-backed run of all six rows whose wrapper and implementation are
already executable found no green candidate: one has a child dependency mode
failure, two have stale README/owner evidence, two have missing manifest IDs,
and the ID-brand row has the stale next-row status.  The family migration is
therefore retained as a design boundary, not silently converted into chmod or
manifest repair work.

```text
Decision: retain all six candidates as unknown_retain with a NoSafeSlice reason;
do not add a seventh manifest disposition.
Source authority + canonical issuer: existing guard_rows/proof_apps manifests
and their generic runners; inventory only observes their closure.
Non-authority: executable bits, filename prefixes, grep counts, or stale prose.
Fail-fast boundary: child-mode, owner/README, manifest-ID, or next-row drift
blocks candidate admission and keeps the family retained.
Smallest next slice: park this family and enter QUICK-PROFILE-RECUT-R0 design.
Non-claims: no chmod, wrapper move/delete, manifest repair, compiler change,
fallback, archive publication, or production cutover.
```

The quick-profile BoxShape is landed in `bae8ec26e9`: source command rows remain
66/66, `--list` exposes exactly ten groups, and `--list-steps` exposes the same
66 detailed rows. Naming/varmap reds remain parent-baseline debt.
The historical-retire census is NoSafeSlice: all 3,654 rows have non-empty
owner/caller/evidence classification, so no zero-caller family can be retired
without inventing authority; rows remain in their existing dispositions.

#### MIRBUILDER-LINE-BUDGET-CENSUS-R0 (closed 2026-08-14 BoxShape)

```text
Decision: split only the 797-line unified call facade's two public compatibility
entrypoints into a child module; preserve API, order, and owner semantics.
Source authority + canonical issuer: tracked Rust source and existing reusable
line guards; the census reports but never authorizes a split.
Non-authority: compression, blank-line removal, grep-only ownership, or prose.
Fail-fast boundary: touched owner at/above 800, missing split target, or a new
responsibility without a prior owner keeps the row open.
Closeout: `emit_global_unified` and `emit_value_unified` now live in the private
child; the facade is 751 lines, API/caller parity is preserved, and focused
check/guard evidence is green (the all-tests failure reproduces at the parent).
Non-claims: no mass refactor, semantic change, deletion, fallback, archive,
llvmlite retirement, or production cutover.
```

1. `GUARD-PUBLIC-ENTRY-CUT-R0`
   - keep only stable daily/family launchers in the human index;
   - move historical lookup to a generated inventory plus git history.
2. `GUARD-FAMILY-MANIFEST-MIGRATION-R0`
   - migrate `k2_*` and `rust_lifecycle_mirbuilder_*` per-row shells first;
   - reuse `run_row_guard.sh`/one generic runner and data-only manifests;
   - keep behavior tests as tests rather than converting them to grep rows.
3. `GUARD-QUICK-PROFILE-RECUT-R0`
   - expose roughly 10--15 named groups instead of 66 individual public
     steps, while retaining the selected behavior and authority checks inside
     those groups;
   - remove retired llvmlite daily dependencies only under the accepted G3
     ownership boundary.
4. `GUARD-HISTORICAL-RETIRE-R0`
   - archive or delete only zero-caller wrappers with equivalent manifest,
     source, test, or git-history coverage;
   - unknown-owner and unique behavior rows remain.
5. `GUARD-SURFACE-CLOSEOUT-R0`
   - record before/after entry, file, line, quick-time, and failure-signal
     counts;
   - prove current P0 negatives still fail and daily gates stay green.

The initial editorial target is at most about 50 human-facing stable entries,
not a semantic hard cap.  The D0 inventory owns the exact retirement count.
No mass deletion, blanket skip, generated always-green wrapper, or guard-only
replacement for a behavior test is allowed.

## Main integration and G3 resume

`MAIN-INTEGRATION-EVIDENCE-R0` runs in a clean detached worktree and records
the exact integration SHA, history policy, focused W6/G0/G1/G2 tests, stable
guards, `cargo check --lib`, and diff check. The branch is not silently
squashed because existing receipts refer to intermediate commits.

Closeout evidence (2026-08-14, clean detached worktree at
`145921e7d5097d1c10b8fe376fffcf1b8d8d07d9`):

```text
status / history policy                         = clean / no squash
cargo check --lib                                = green
normal_callable_semantic_package                = 18 passed
dynamic_full_body_recipe                        = 33 passed
selected_dynamic_physical_emitter               = 5 passed
completion                                      = 107 passed, 1 parent-baseline red
selected W6/AOT/physical-input/callslot/VM/precutover guards = green
llvm census / route identity guards              = green
dynamic_v2_aot_activation_authority_guard        = green (archive absent is informational)
mirbuilder_inplace_replacement_guard             = parent-baseline red: direct Hakorune fixture absent
git diff --check                                  = green
selected production                             = new 0 / old 1; live cutover remains closed
```

The completion failure is the already recorded
`canonical_physical_completion_p0::compiler_bridge_drains_a_plus_single_route`
(`ReturnValueTypeMissing(ValueId(12))`), reproduced at both this SHA and
`1c57a95d61`.  The in-place guard red is likewise reproduced at both SHAs and
comes from the unavailable generic legacy executable fixture, not the selected
Boundary route.  These are named baseline/environment debt; they are not
silently converted to green evidence.  The executable-bit omission in the W6
smoke was corrected in `145921e7d5`, after which the AOT authority guard passed.
The integration row is therefore closed as evidence classification, not as a
claim that the baseline debt or live production cutover is complete.

That SHA is a classified baseline, not proof for the latest branch head.
`MIRBUILDER-LATEST-HEAD-INTEGRATION-R0` is the one predeclared return gate:
rerun the same detached family at the exact current SHA, classify every red
against its parent, and record selected `new=0 / old=1`. It changes no code or
route. When it closes, this hardening card stops and `CURRENT_STATE.toml`
retargets explicitly to `JOINIR-LOOP-M8-LOOPV0-SCANS-S6C`.

Only after every P0 row in the ordered DAG and integration evidence may
`LLVMLITE-ORACLE-COVERAGE-D0` resume. Its acceptance is refined to:

```text
119 candidates classified
six replay dependencies included
32 root opcodes plus every dispatch/leaf family -> replay_case | archive_only
O1-O6 use fixed MIR JSON bytes and independently-issued expected results
O7 is unsupported_nonconsumer_fence, not a positive oracle
consumer identity = 48 unique owner+selector edges; 50 source row IDs retained
```

## Parked P1/P2 cleanup

These do not block the product frontier. `LiveBlocker = 0` and
`CutoverBlocker = 0` at this census: a row may reopen only when a named
production/candidate reproducer reaches effect/publication, the exact selected
cutover consumes its owner and the unchanged gate fails, a caller changes
from zero to nonzero, or a touched authority/ABI/identity owner invalidates the
closed proof. Grep-only suspicion, naming/LOC/style, future families/platforms,
and another audit score remain parked.

1. `DYN-CALLOUT-EFFECT-R0`
   - project each site effect from its retained exact CoreMethod row;
   - keep whole-function effect as a separate union authority;
   - make Boundary C require metadata/MIR effect parity and known bits.
2. `CALLOUT-CENSUS-REUSE-R0`
   - lend the verified function census site/result view to AOT projection;
   - remove the second physical MIR scan without creating another plan table.
3. `DYN-ASSEMBLY-SEAM-R0`
   - remove the production inspection callback;
   - retain one `#[cfg(test)]` inspection wrapper over shared private logic.
4. `DYN-PRODUCTION-NAMES-R0`
   - rename only genuinely stale live W6/I8/canary identifiers;
   - retain true I8 evidence names and historical receipts.
5. `DYN-W6-SHELL-RETIRE-R0`
   - move the live artifact fence to a stable owner;
   - delete or test-gate old prepared W6 aggregates only after caller zero.
6. `DYN-SOURCE-FACT-ISSUER-ONCE-R0` (closed 2026-08-14 BoxShape)
   - package admission and route-neutral catalog/fixture owners issue once;
     call-row derivation borrows the exact product and rejects foreign owner
     before ledger traversal; focused coseal tests, batch guard, and check pass.
7. `DYN-CATALOG-PHYSICAL-HEADER-PROJECTION-R0` (closed 2026-08-14 BoxShape)
   - catalog row -> one linear package-loan projection -> A-prime; raw
     AST/root re-observation removed; focused tests and authority guard pass.
8. `DYN-CALLOUT-NORMAL-RESULT-TYPE-R0` (accepted 2026-08-14 BoxShape)
   - Decision: I6 `StringValue` + `opaque_handle` uses the existing `string_handle -> MirType::Box("StringBox")` projection; co-seal type, EndAuthorizedHandle lease representation, and ledger row; retain I7 Integer/ImmediateI64.
   - Authority: CheckedCallOut plan + route projection + canonical SSA issuer + private ledger; reject shape/type/representation/PlanStamp/landing or duplicate drift.
   - Slice: canonical typed publication helper, selected positive/negative tests, and one existing authority-guard extension; no new guard family.
9. `DYN-DRAFTSEAL-EVIDENCE-CONSUME-R0` (closed 2026-08-14 BoxShape)
   - Decision: Completion and `FunctionDraftSealReceiptV1` are proof-only DraftSeal evidence; retire them explicitly before collector handoff, never reissue them as publication authority.
   - Authority: `ResolvedFunctionCompletionConsumptionV1` and `PreparedFunctionDraftSealPlanV1` issue/verify; `CompletedFunctionDraftV1` owns the exact-once retirement terminal.
   - Slice: private `consume_non_authority_evidence()` terminal, focused positive/negative handoff tests, and one existing authority-guard extension; no new semantic receipt or guard family.
10. `DYN-ACTIVATION-DISPOSITION-TYPESTATE-R0` (closed 2026-08-14 BoxShape)
    - `RejectBeforeEffect` now moves once into a private unpublished-session
      fence, consumed immediately before Builder open; the session stores no
      disposition and no executable/backend/runtime meaning is issued.
    - focused emitter/package tests, cargo check, fmt, existing AOT/text/physical-input/pointer guards, and diff check are green; selected production remains new=0/old=1 with fallback/retry=0.
11. `CHECKED-CALLOUT-PHYSICAL-ID-ISSUER-R0` (closed 2026-08-14 BoxShape)
    - Private tuple fields and explicit `as_u32`/`from_test`/`from_wire`/`from_admitted` boundaries now keep the canonical pair as the Site/Outcome issuer, admitted facts as the Entry/lease projection source, and JSON as transport ingress.
    - Selected capability, emitter, lifecycle, profile close, receipt, printer, and test callers no longer mint or inspect raw physical IDs; the existing AOT authority guard fixes the owner/accessor and raw-consumer census without a new guard family.
    - `cargo check --lib`, checked-callout (14), selected-emitter (5), provider-admission (6), fmt, guard, pointer, and diff checks are green; selected production remains new=0/old=1.
12. `DYN-CALLOUT-BOUNDARY-CFG-OPERAND-PARITY-R0` (closed 2026-08-14 BoxShape)
    - Decision/authority: one non-Clone/HRTB canonical CFG/SSA census view feeds AOT/JSON/Boundary; no plan rebuild or second CFG authority. View carries exact sites, source/Normal/Fault blocks+predecessors, operands, projection, effect, ABI/wire/shape/slots/PlanStamp, and End cuts.
    - Reject/acceptance: orphan/duplicate site, plan/terminator/projection, operand/landing/predecessor/End/receipt-set drift is rejected before effect; existing AOT projection consumes the view once, JSON/Python carry the facts, Boundary C and canonical smoke/negative fixtures pass. No opcode/schema/new C ABI/cutover/fallback/VM/PyVM/llvmlite; selected production remains new=0/old=1.
13. `DYN-CALLOUT-WIRE-FAILSTOP-R0` (closed 2026-08-14 BoxShape)
    - Fixed header/Rust wire remains sole authority; C1 now traps I6 zero host payload and Fault outside 1..8 while preserving I7 zero and known semantic Fault landing. No ABI/enum/new guard/fallback/retry/VM/cutover.
    - Rust 11 + Python 11 wire tests, C1 physicalizer smoke, wire/AOT guards, cargo check/fmt, pointer guard, and diff check are green; selected production remains new=0/old=1.
14. `DYN-LEASE-PUBLISH-ATOMICITY-R0` (closed 2026-08-14 BoxShape)
    - Decision/authority: private host-handle child owns the lock-scoped `(handle, generation identity)` allocation; collision/exhaustion preserves the old entry and drops only the matching new identity. Rust lease/host tests, kernel TextScan tests, C1/AOT/pointer guards, check/fmt, and diff checks are green.
    - Nonclaims: no raw `drop_handle` rollback authority, C ABI/schema, strict-leaf semantics, fallback/retry, production cutover, or VM/llvmlite route.
15--18. PARKED: LocalSSA failure-terminal migration is T2 but has no selected
    or cutover caller; typed ProviderSlot carrier is future SSOT cleanup;
    non-Linux selected platforms are future-family work; line-budget census is
    touch-triggered cleanup only. None may insert a row before S6C.

## Cross-row stop line

- Do not add a new task-specific top-level guard while an existing family
  guard, focused test, or manifest row can own the evidence.
- Do not turn a slot observation into a new semantic receipt.
- Do not remove `Clone` repository-wide as an incidental fix.
- Do not use mutation-count zero as a selected post-seal fence.
- Do not repair runner reachability before selected post-seal immutability and
  strict final verification are real commit barriers.
- Do not let compatibility environment variables weaken selected verification.
- Do not copy helper metadata onto `main` or alias a four-argument helper to
  the zero-argument runtime launch symbol.
- Do not route artifact/publication failures into ordinary or llvmlite paths.
- Do not let the root validator reconstruct provider, Recipe, CFG, or lease
  meaning.
- Do not add Python, runtime registry lookup, fallback, retry, or VM selected
  consumers.
- Do not archive/move llvmlite until Boundary ownership no longer depends on
  harness feature/module names.
- Do not touch unrelated dirty compiler files in any row.
