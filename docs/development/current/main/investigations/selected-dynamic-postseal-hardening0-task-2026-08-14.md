---
Status: Accepted; next fast row is DYN-PROD-BASELINE-R0
Date: 2026-08-14
Parent: docs/development/current/main/investigations/dynamic-v2-w6-production-activation-task-2026-08-13.md
Resume-after: docs/development/current/main/investigations/llvmlite-keep0-ret0-inventory-task-2026-08-14.md
Scope: selected Dynamic post-seal lifecycle, publication, and backend-owner hardening before G3 archive movement or main integration
---

# SELECTED-DYNAMIC-POSTSEAL-HARDEN0

This card accepts the post-W6 audit without reopening semantic planning,
Recipe, CheckedCallOut meaning, DraftSeal, or the selected production route.
The selected canonical core remains the authority; these rows make its linear
state, post-seal mutation boundary, artifact publication, and build-feature
ownership fail closed.

## Six-line brief

```text
Decision: close four post-W6 physical/lifecycle gaps before resuming llvmlite G3 archive work or integrating the branch.
Source authority + canonical issuer: linear metadata slots, the sealed selected metadata pair, canonical MIR, StaticAotArtifactPublicationTxnV1, and Cargo/DriverKind feature ownership.
Non-authority: Option::None, mutation_count=0, a receipt JSON claim without physical co-check, llvm-harness naming, llvmlite output, or stale migration prose.
Fail-fast boundary: scrubbed/partial metadata, any selected post-seal mutator, partial artifact visibility, actual-digest drift, or implicit Boundary-to-compat reachability rejects before fallback or launch.
Smallest next slice: DYN-PROD-BASELINE-R0 synchronizes the stale production caller guard/docs and classifies the outstanding completion red without changing behavior.
Non-claims: no semantic receipt, accepted source shape, Recipe/MIR change, new backend, fallback/retry, llvmlite archive move, external publication, deletion, or main integration.
```

## Audit verdict

The four P0 findings are accepted with these exact qualifications:

| Finding | Current evidence | Classification |
|---|---|---|
| clone-scrubbed selected metadata becomes ordinary | both linear slots map `Occupied -> Consumed` on clone; both `borrow()` methods map `Empty` and `Consumed` to `None`; selected census sees only `is_some()` | latent correctness P0; current live selected caller moves the module and censuses before clone, so no current reproduction was found |
| Method-ID injector runs after seal | runner censuses selected metadata, then lends `&mut MirModule` to the default-enabled injector | structural P0; the current pass is an explicit no-op returning zero, so no current bytes are mutated |
| executable is renamed before receipt publication | child commits the candidate executable to the final path, then writes/renames receipt JSON | publication P0; root refuses launch without a valid receipt, but artifact visibility and prior-final rollback are not atomic |
| Boundary production is gated by `llvm-harness` | selected path lives in `HarnessExecutorBox`, `selected_dynamic_nyrt_dir` is feature-gated, and `llvm = ["llvm-harness"]` | ownership P0; selected execution uses Boundary/ny-llvmc and does not spawn Python, but G3 cannot archive the harness boundary safely while these owners are mixed |

The feedback does not justify rebuilding MIRBuilder. It identifies post-cutover
hardening around an otherwise single canonical lane.

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
  -> SELECTED-DYNAMIC-LINEAR-SLOT-FENCE-R0
  -> SELECTED-DYNAMIC-POSTSEAL-MUTATION-FENCE-R0
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

## Row B2: SELECTED-DYNAMIC-POSTSEAL-MUTATION-FENCE-R0

Classification: BoxShape. The current Method-ID pass is a no-op, but selected
MIR must not cross even a nominal post-seal mutable seam.

```text
selected metadata pair
  -> read-only legacy-callsite scan
  -> Boundary path

ordinary module
  -> existing compatibility injector stage exactly once
```

The selected scan reuses the existing canonical legacy callsite classifier.
The relevant invalid shape is `Call { callee: None }`, not a retired
`MirInstruction::BoxCall`. A valid `Callee::Method` remains accepted. Selected
invalid shape rejects before JSON, child process, object, fallback, or retry.

Long term, retire the no-op injector wrapper/plan/report fields after a caller
census. If future ordinary physical normalization is required, it belongs to
the unpublished compiler postprocess before external commit; method meaning
belongs earlier in source/Facts/Recipe/canonical lowering.

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

Only after B0-B4 and integration evidence may
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

These do not block B0-B4 unless a touched file makes them necessary:

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

## Cross-row stop line

- Do not turn a slot observation into a new semantic receipt.
- Do not remove `Clone` repository-wide as an incidental fix.
- Do not use mutation-count zero as a selected post-seal fence.
- Do not route artifact/publication failures into ordinary or llvmlite paths.
- Do not let the root validator reconstruct provider, Recipe, CFG, or lease
  meaning.
- Do not add Python, runtime registry lookup, fallback, retry, or VM selected
  consumers.
- Do not archive/move llvmlite until Boundary ownership no longer depends on
  harness feature/module names.
- Do not touch unrelated dirty compiler files in any row.
