---
Status: Accepted; next fast row is DYN-PROD-BASELINE-R0
Date: 2026-08-14
Parent: docs/development/current/main/investigations/dynamic-v2-w6-production-activation-task-2026-08-13.md
Resume-after: docs/development/current/main/investigations/llvmlite-keep0-ret0-inventory-task-2026-08-14.md
Scope: selected Dynamic admission, post-seal lifecycle, publication, Boundary reachability, and MirBuilder safety hardening before G3 archive movement or main integration
---

# SELECTED-DYNAMIC-POSTSEAL-HARDEN0

This card accepts the post-W6 audit without reopening Recipe, CheckedCallOut
meaning, Completion, or DraftSeal semantics.  A broader worker audit found
that the selected infrastructure is not yet an end-to-end executable route:
the runner stops at its PyVM nonconsumer fence, final module verification is
not a strict commit barrier, and Boundary confuses the zero-argument launch
entry with the metadata-bearing selected helper.  Upstream declaration-mode
admission and one safe lexical-scope owner also require correction.  The
selected canonical core remains the authority; these rows make every boundary
from admission through launch fail closed.

## Six-line brief

```text
Decision: close the audited admission/safety/post-seal/Boundary gaps before resuming llvmlite G3 archive work or integrating the branch.
Source authority + canonical issuer: resolved declaration mode, FunctionOwned lexical scope, linear metadata slots, sealed canonical MIR plus strict final verifier, exact launch/helper identities, StaticAotArtifactPublicationTxnV1, and Cargo/DriverKind feature ownership.
Non-authority: raw AST mode/header re-observation, a raw-pointer lifetime comment, Option::None, mutation_count=0, ambient verifier env, entry-name fallback, receipt JSON without physical co-check, llvm-harness naming, or llvmlite output.
Fail-fast boundary: unsupported Dynamic mode, scope-close failure, scrubbed/partial metadata, any selected post-seal mutation or verifier weakening, launch/helper identity drift, partial artifact visibility, or implicit Boundary-to-compat reachability rejects before external commit, fallback, or launch.
Smallest next slice: DYN-PROD-BASELINE-R0 synchronizes the stale production caller guard/docs and classifies the outstanding completion red without changing behavior.
Non-claims: no semantic receipt, accepted source shape, Recipe/MIR change, new backend, fallback/retry, llvmlite archive move, external publication, deletion, or main integration.
```

## Audit verdict

The original four P0 findings remain accepted.  The expanded worker audit
adds the following independently reproduced P0 boundaries:

| Finding | Current evidence | Classification |
|---|---|---|
| clone-scrubbed selected metadata becomes ordinary | both linear slots map `Occupied -> Consumed` on clone; both `borrow()` methods map `Empty` and `Consumed` to `None`; selected census sees only `is_some()` | latent correctness P0; current live selected caller moves the module and censuses before clone, so no current reproduction was found |
| Method-ID injector runs after seal | runner censuses selected metadata, then lends `&mut MirModule` to the default-enabled injector | structural P0; the current pass is an explicit no-op returning zero, so no current bytes are mutated |
| executable is renamed before receipt publication | child commits the candidate executable to the final path, then writes/renames receipt JSON | publication P0; root refuses launch without a valid receipt, but artifact visibility and prior-final rollback are not atomic |
| Boundary production is gated by `llvm-harness` | selected path lives in `HarnessExecutorBox`, `selected_dynamic_nyrt_dir` is feature-gated, and `llvm = ["llvm-harness"]` | ownership P0; selected execution uses Boundary/ny-llvmc and does not spawn Python, but G3 cannot archive the harness boundary safely while these owners are mixed |
| selected Boundary dispatch is unreachable | the selected arm constructs a fatal PyVM-rejection error and immediately exits; the later `try_execute_selected_dynamic` call is dead for every selected input | live route P0; PyVM remains retired from selected production, but its fence currently blocks Boundary too |
| selected final verification is not a commit barrier | normal finish retains a pre-transform verifier `Err`, mutates the module afterward, commits it, and the LLVM adapter returns only `.module` | publication P0; JSON/Boundary are not verifier substitutes |
| selected verifier policy is ambient | `NYASH_STAGEB_DEV_VERIFY=0`, `NYASH_VERIFY_ALLOW_NO_PHI=1`, or `NYASH_MIR_NO_PHI` can skip/weaken checks used by the selected lane | verification P0; selected production needs an env-independent strict policy |
| Boundary launch and helper identities disagree | Rust selects the module by exactly one metadata-bearing helper, while C and the artifact descriptor inspect the zero-argument `main`/`ny_main` entry | physical identity P0; a four-argument helper may not be aliased to the runtime launch entry |
| Dynamic admission is not declaration-mode bounded | package admission probes every resolved declaration although A-prime/lowering accepts only `StaticBoxMethod`; Instance/TopLevel exact shapes can be selected or fail before ordinary ownership | upstream acceptance P0; mode comes only from the resolved batch row |
| lexical scope guard is a safe unsound API | `LexicalScopeGuard` stores a lifetime-free raw `*mut MirBuilder`, dereferences it in safe `Drop`, and hides KeepAlive close errors | MirBuilder safety P0; replace it with an escape-proof fallible scope transaction |

The feedback does not justify rebuilding MIRBuilder. It identifies post-cutover
hardening around an otherwise single canonical lane.

`dynamic_v2_aot_activation_authority_guard.sh` is green at this HEAD, but it
checks the existence/count of the downstream selected call rather than control
dominance; it therefore does not prove that Boundary is reachable.  Local
structural green remains non-production evidence until the execution-shaped
negative/positive tests above exist.

## Immediate baseline debt

One stable guard is red at this HEAD and at both `HEAD^` and the G0-G2 close
commit `8c9b6956d9`:

```text
bash tools/checks/dynamic_v2_text_scan_admission_authority_guard.sh
-> unexpected caller: src/mir/builder/normal_callable_semantic_loan_port.rs
```

This is accepted allowlist drift: the guard still encodes the pre-W6
definition-plus-test caller set and does not admit the landed single production
caller. `dynamic_v2_aot_activation_authority_guard.sh` and the current pointer
guard remain green. Do not waive the red; repair its exact caller contract in
the first row.

Owner documentation is also stale. In particular,
`src/mir/builder/resolved_lowering/README.md` still describes End, profile
close, DraftSeal, publication, and production selection as closed. The W6 card
also names a `completion` failure without the exact parent command/SHA/result
needed to classify it as baseline debt.

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
lexical_scope production unsafe / raw pointer lifetime claim = 0 / 0
success / Err / panic restore count                           = 1 / 1 / 1
injected KeepAlive close failure                             = typed error
scope chronology / Recipe / CFG / accepted source delta      = 0
```

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

The bounded series first lands a production-shaped `main + selected helper`
fixture, then one C owner censuses the exact helper and sends that function to
the existing CheckedCallOut physicalizer, and finally proves object -> link ->
receipt -> zero-argument launch end to end.  Missing/duplicate metadata,
metadata on the launch entry, missing/duplicate/nonzero-argument main, helper
arity drift, generic CheckedCallOut fallback, or a descriptor sourced from
main all reject before publication.  Metadata copying and by-name reselection
are forbidden.

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

1. `...-BUNDLE-PREPARE-I0`
   - prepare executable and receipt bytes inside one candidate directory;
   - all child fallible work completes before publication;
   - attempt identity prevents stale/PID-only receipt reuse.
2. `...-BUNDLE-COMMIT-I0`
   - publish by exactly one same-filesystem directory rename;
   - collision/rename failure leaves no final bundle and preserves prior
     artifact; commit has no subsequent child write.
3. `...-ROOT-CONSUME-R0`
   - require exact expected bundle and regular executable path;
   - hash the actual executable and compare with receipt;
   - co-check input, descriptor, site/ABI/wire/PlanStamp, and exact symbol
     census rather than merely nonzero values;
   - fix success/error receipt/bundle cleanup policy before launch.

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

`LLVM-BOUNDARY-EXECUTOR-S0` first performs a behavior-neutral physical split:
`BoundaryExecutorBox` and Boundary NyRT/process helpers leave
`harness_executor` and llvmlite helpers. `LLVM-BOUNDARY-COMPAT-FEATURE-R0` then
atomically moves the selected production caller to `llvm-boundary`, removes the
old selected `llvm-harness` cfg edge, gates every Python owner under
`llvmlite-compat`, and updates G0/G2/G3 inventories and guards.

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

## Main integration and G3 resume

`MAIN-INTEGRATION-EVIDENCE-R0` runs in a clean detached worktree and records
the exact integration SHA, history policy, focused W6/G0/G1/G2 tests, stable
guards, `cargo check --lib`, and diff check. The branch is not silently
squashed because existing receipts refer to intermediate commits.

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

These do not block the P0 DAG unless a touched file makes them necessary:

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
6. `DYN-SOURCE-FACT-ISSUER-ONCE-R0`
   - issue the source-backed Dynamic callable once in package admission;
   - derive I6/I7 call rows through a private scoped borrow instead of
     reissuing the same Facts from the resolved input.
7. `DYN-CATALOG-PHYSICAL-HEADER-PROJECTION-R0`
   - carry one private catalog-owned header projection into A-prime;
   - remove raw AST name/parameter/return/attrs/uses re-observation.
8. `DYN-CALLOUT-NORMAL-RESULT-TYPE-R0`
   - have the canonical normal-result issuer co-seal ValueId, site shape,
     MIR type, physical representation, and value-ledger publication;
   - specifically close the missing I6 handle type publication.
9. `DYN-DRAFTSEAL-EVIDENCE-CONSUME-R0`
   - decide and guard the explicit collector consumption or intentional
     nonauthority retirement of Completion and `FunctionDraftSealReceiptV1`;
   - forbid their silent drop from being treated as publication evidence.
10. `DYN-ACTIVATION-DISPOSITION-TYPESTATE-R0`
    - remove the negative-only `RejectBeforeEffect` token from the successful
      activation chain, or consume it in an explicit unpublished-to-ready
      typestate transition before Builder open.
11. `CHECKED-CALLOUT-PHYSICAL-ID-ISSUER-R0`
    - make Site/Entry/Outcome/Lease IDs private and mint them once;
    - consumers borrow issued IDs instead of reconstructing raw `0/1` values.
12. `DYN-CALLOUT-BOUNDARY-CFG-OPERAND-PARITY-R0`
    - carry the verified canonical site/result/landing/End census to Boundary;
    - reject operand, landing, predecessor, projection, or End-placement drift.
13. `DYN-CALLOUT-WIRE-FAILSTOP-R0`
    - trap on I6 zero handle, unknown Fault codes, Suspended, malformed wire,
      or transport errors; only known semantic Fault reaches MIR Fault.
14. `DYN-LEASE-PUBLISH-ATOMICITY-R0`
    - issue handle plus lease identity in one host-handle owner transition;
    - restore the handle on every identity/token publication failure.
15. `LOCAL-SSA-CHECKED-TERMINAL-R0`
    - migrate production callers away from `unwrap_or(original ValueId)` and
      `LegacyFacade -> Ok(original)` fail-open wrappers;
    - keep any remaining compatibility fallback explicitly fenced.
16. `TEXT-SCAN-PROVIDER-CONTRACT-TYPED-CARRIER-R0`
    - generate the ProviderSlot role/profile/lifecycle carrier from its Hako
      source and remove hand-written Rust mirrors;
    - keep CoreMethod result/effect and neutral C ABI under their existing,
      separate authorities.
17. `DYN-BOUNDARY-PLATFORM-FENCE-R0`
    - document Linux x86_64 as the currently supported selected target and
      reject other targets before output until their backend rows exist.
18. `MIRBUILDER-LINE-BUDGET-CENSUS-R0`
    - warn at 760 and fail at 800 using one reusable census;
    - split only owners touched by an accepted row, never as a mass refactor.

## Cross-row stop line

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
