# MirBuilder Gate 1 — current design and acceptance

Status: per-exit Home flow S0 landed; constructor-argument evidence D0 is next
Date: 2026-09-29
Scope: MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0; compact Gate-1 frontier.
Related: CURRENT_STATE.toml; workstream row H; RULES.md;
  ownership-home-model-ssot.md; own-home-callable-abi-d0-design-task-2026-08-09.md.

## Current decision

The 2026-09-29 review at `339674c77b` invalidates the blanket
`AllSixRedsAwaitNamedParkedFamilies` conclusion from `053b659637`.
Gate 1 is still unsatisfied. The user selected organization and design;
this turn closes the entry contract and selects the next bounded S0 without
executing it. One read-only worker audited the exact authority and consumers;
the primary integrated the decision below. The original
`MIRBUILDER-GATE1-MULTI-RETURN-RESULT-NEW-D0` is decomposed into entry admission,
per-exit flow/consumption, and later argument/result contracts.

```text
Decision: adapt the sole Home ABI owner to lend current-batch entry demands;
  full call-site result ABI and per-exit Home flow remain separate contracts.
Source authority + canonical issuer: exact resolver callable batch/common
  parameter catalog -> CallableHomeAbiIssuerV1 -> scoped source entry-flow loan.
Non-authority: receiver spelling, MIR types, absent result annotation, an empty
  cleanup vector, and a retained Construction terminal.
Fail-fast boundary: foreign/incomplete entry rejects; captures stay unavailable;
  unresolved result never issues Unit or a complete call-site Home ABI.
Smallest next slice: MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0, replacing only
  receiver-presence => EntryDemandMissing for the exact admitted entry cohort.
Non-claims: no multi-return flow, constructor argument/result widening,
  field destruction, selected-C/backend activation, app PASS or Gate-1 completion.
```

The canonical ABI D0's complete-call-ABI rule is preserved with a narrow
[entry-flow loan amendment](own-home-callable-abi-d0-design-task-2026-08-09.md#source-entry-flow-projection-2026-09-29).
Updating that owner is required because it specified only a complete ABI
aggregate. The new entry-only loan stays inside the owner aggregate, keeps the
result unresolved and cannot reach a call-site resolver. No new receiver ownership semantics are invented.

## Corrected frontier ownership

This census covers the **recorded 11-entry EXE suite's first terminals ->
known source/transport owners and downstream dependencies**. It includes the
six recorded red members; it excludes a new suite run, new dynamic terminal
ordering, production caller-zero, and all-repository candidate exhaustion.
No entry below is a newly measured runtime result.

| Member | Recorded first terminal | Next responsibility / boundary |
| --- | --- | --- |
| mimalloc_lite | `artifact-unowned-lifecycle-site` | Selected D0: receiver-entry demand and per-return Home flow for retained return-new claims. Argument representation, mixed return ABI and destruction remain distinct later dependencies. |
| boxtorrent_mini | `unsupported terminator Invoke` | Its smoke still uses generic JSON ingress. Physical-route acceptance migration is a separate harness decision; field-write `new`/DropPlan is downstream, not its observed first terminal. |
| binary_trees | `root-call-entry-unavailable` | Unreleasable root Home; owning-field destruction and untyped storage stay outside this D0. |
| allocator_stress | `NamedArray(TextSourceMissing)` | Named-array/call-result argument provenance. A separate owner is not by itself evidence of an external wait. |
| json_stream_aggregator | `route-not-front-selected` | Root `me` loop source coverage and existing locator consumer need a bounded connection decision. The selected-C arbitrary-UserBox park does not close canonical MIR design. |
| typed_object_untyped_field_min | `return_type_strategy` panic | Known compiler baseline defect reproduced at `31c4bc072e` in the recorded receipt; not an intentional rejection contract. |

The selected-C park remains exactly
`MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0`, whose manifest keeps
canonical MIR routes. See
[manifest](mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml).
Existing canonical locator evidence is in
[S3](mirbuilder-exe-acceptance-declared-instance-loop-locator-s3-2026-09-26.md).
Neither that receipt nor the present static review proves current json-stream
EXE success. Do not change this app or select that sibling during this D0.

Receiver demand, CFG Home flow, destination semantics and terminal DropPlan
have distinct owners in
[Home task order](hakorune-home-ownership-task-2026-08-04.md):
`OWN-HOME-CALLABLE-ABI-D0`, `OWN-HOME-FLOW-CFG0-S0`,
`OWN-FIELD-CONTAINER-DEST-D0`, and terminal finalization.
The latter two are not reopened by diagnosing the former two. A downstream
field dependency does not remove an upstream internal design task.

## Selected source boundary

Target: the already-inventoried nine retained result-new sites in the
mimalloc-lite imported allocator bodies, including
`HakoAllocPage.allocate/1`'s `return new HakoAllocHandle(...)` and
`HakoAllocHandleResult` constructors. Reuse the existing inventory; do not
start another whole-suite or repository census to rediscover this target.
The finite source inventory in `lang/src/hako_alloc/memory/page_heap_box.hako`
is 1 + 3 + 5 = 9 Return-new sites (not nine HandleResult constructors):

| Callable | Source lines | Result-new sites |
| --- | --- | --- |
| HakoAllocPage.allocate/1 | 102 | one HakoAllocHandle; other returns are null |
| HakoAllocHeap.allocateResult/1 | 216, 221, 224 | three HakoAllocHandleResult |
| HakoAllocHeap.reallocResult/2 | 302, 306, 310, 315, 318 | five HakoAllocHandleResult |

The current implementation task admits declaration entry; it does not promise
available flow for all nine sites or erase unsupported field/call operations.

- Membership: `ordinary_new_coseal_issue.rs::issue_ordinary_source_cohort_v1` uses
  exact construction and parent `Return { value }` source membership.
  A trailing `Value` segment alone never establishes Return position.
- Source flow: `home_new_prefix.rs::scan_new_home_flow` and
  `home_new_prefix_scan.rs::scan_statement_flow` share the current issuer.
  A receiver/capture installs `EntryDemandMissing`; unsupported statements
  retain `PrefixNotCovered` (they do not override the earlier issue).
- Return observation uses `completion.explicit_site()`;
  `ExplicitReturns` yields no single site. Nested branch Returns are not
  traversed by the top-level walk. Result claims can therefore be retained
  with `SourceMismatch` even while exact Return membership exists.
- `emission_prepare.rs` turns a failed prefix into `prior_homes=None` and
  unavailable emission. Observing additional Return sites cannot by itself
  fix entry demands, branch state, cleanup or artifact admission.
- The common declaration parameter catalog owns parameter representation,
  not receiver or result meaning. Receiver classification must come from
  the canonical Home ABI owner, never an `if receiver { Handle }` patch in
  the scanner. Missing combined result authority must not become Unit.

Selected chain to finish designing:

```text
same parser/declaration/batch + common parameter contracts
  -> sole Home ABI owner: exact source entry demands
  -> scan_new_home_flow: path-specific prior Homes and Completion/Fault exits
  -> ordinary_new_coseal_issue: per-site result prefix + argument contract
  -> existing result claim ledger -> selected emission/return validation
```

Do not pair independent receipts later by names or owner keys. The entry and
flow evidence must belong to the same source invocation before co-seal.
A retained `Value(Construction)` describes a result; it is not evidence that
all paths have valid cleanup or that a mixed null/new callable returns Handle.

## Next implementation — MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0

Change:
  The existing `home_abi` issuer accepts current-batch entry authority and
  lends it through `issue_ordinary_source_cohort_v1` ->
  `verify_function_completion_with_new_homes_and_argument_observations_v1`
  -> `scan_new_home_flow` / `PrefixLocalFlow`. Remove the unconditional
  receiver-presence rejection only for that verified loan in the same slice.
Contract:
  Keep the common parameter catalog alive at `issuer.rs` (currently reduced
  to owned parameter rows before ordinary-new issuance); consume/project it
  once without reissuing parameter meaning. Bind the entry loan to parser,
  batch, declaration, owner, instance mode and exact Receiver binding. The
  existing classifier supplies receiver Handle; parameters preserve both
  `home_demand()` and their existing representation kind. Entry Handles add
  no owned Home/release. Result remains unresolved; complete ABI publication
  is unavailable. Captures and unsupported body statements retain rejection.
Done:
  An admitted straight-line instance method with an existing scalar/local-new
  body has a source prefix and normal/fault cleanup that excludes receiver
  and borrowed parameters. Exercise the real package issuer/consumer, not
  just classifier units. Negative cases cover missing/foreign evidence,
  wrong owner/mode/binding, duplicate/missing parameters, captures and result
  publication attempts; existing static and complete I64/Unit paths retain
  their checks. Run focused Home/ordinary-new tests and the existing
  `mirbuilder_qualified_route_scope_guard.sh`; update owner README/reference
  when implementing. Record actual successor terminal, not assumed EXE PASS.
Stop:
  If this needs result inference, receiver consumption, capture admission,
  new parameter representation or branch cleanup, return to the named D0.
  Do not route missing evidence through old raw emission or another issuer.

| Entry state / transition | Required behavior |
| --- | --- |
| Exact instance + explicit capability + complete parameter relation | Owner issues aggregate-bound entry loan; consumer installs only verified bindings. |
| Exact entry, unresolved result | Source entry flow may consume the loan; full call-site Home ABI stays unavailable. |
| Missing capability / capture demands | Preserve named entry unavailability; never supply empty obligations. |
| Foreign batch/declaration/owner/site or duplicate/missing rows | Reject before flow/physical effects; no key-only recombination. |
| Static/non-instance declaration | Existing route unchanged; no instance entry loan. |
| Unsupported body / unused selected site | Existing prefix/residual rejection remains; entry readiness is not body readiness. |

Production scope is the package's exact ordinary instance declarations with
existing parameter kinds and no capture demand; future Take/consuming receiver
forms are excluded. All other callers retain their present entry checks.
The slice removes one obsolete source decision, not the shared scanner/owner.
Tests after implementation and selected-decision removal are outputs; they are
not demanded before coding. No broad suite rebuild is required for design.

## Landed — MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0

Landed route:
`issue_normal_callable_semantic_package_v1` keeps the live common parameter
catalog and calls
`CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1` once; the
batch-facing projection lives in
`normal_callable_semantic_package/instance_entry_home.rs` (sole authority
stays `home_abi.rs`). The returned `VerifiedInstanceEntryHomeCatalogV1` is
looked up per batch slot inside `issue_ordinary_source_cohort_v1` and lent to
`verify_function_completion_with_new_homes_and_argument_observations_v1`,
`issue_new_home_prefixes_with_arguments_v1`, and the `child_new_ready` probe.
`scan_new_home_flow` replaces only the receiver-presence
`EntryDemandMissing` when a loan is present; `PrefixLocalFlow::install_entry_home`
re-verifies loan owner, receiver `Receiver` declaration binding, and complete
unique parameter rows, installing the receiver as a borrowed self-rooted
`Handle`. Capture demands and static/top-level declarations mint no loan.
Evidence: `instance_entry_home_tests.rs` 7/7 — positive exact loan admits the
`new` site prefix; negatives cover missing evidence, foreign cohort loan,
foreign owner metadata, wrong receiver binding, duplicate/incomplete rows,
and static non-admission. Result publication is type-absent (the loan has no
result relation). Focused suite: 192 pass / `main_f1` red reproduces on
`339674c77b`-era baseline (unrelated `normal_source_plan` debt). Guards:
`current_state_pointer_guard.sh` ok, `mirbuilder_qualified_route_scope_guard.sh`
ok (new pins + boundary registrations). Successor: the named
`MIRBUILDER-GATE1-PER-EXIT-HOME-FLOW-D0` below; no EXE PASS is claimed.

## Following task — MIRBUILDER-GATE1-PER-EXIT-HOME-FLOW-D0

After entry S0, settle one contract spanning source flow and exit consumption.
Existing `completion.explicit_sites()` and exact If regions supply identities;
they do not supply ownership joins. `RootHomeFlow` has one terminal list,
result-new Fault continuations currently require root-body scope, and
`prepare_root_home_exit` accepts one `explicit_site()` with owner-keyed progress.
Those three assumptions must be replaced together for the admitted per-exit
shape; an all-Returns observation patch alone is not sufficient.

Select a finite no-loop/no-capture source shape with complete condition/branch
coverage, explicit branch-state joins, terminated-path handling and reverse
live-Home order at each Return and outward Fault. Prove exact crossed scopes
for nested faults. Preserve divergent/unavailable state instead of taking a
union or empty list; field ownership and unclassified calls stay unavailable.
Each physical exit consumes its site-bound obligation once; result commits
must match their own Return, with duplicate/missing/residual checks. Do not
activate source availability ahead of this consumer.

The D0's execution brief must fix the admitted subtree grammar and join rules,
then name the single-terminal source/fault/exit branches removed in its S0.
First plan a responsibility split of the 779-line `home_new_prefix_scan.rs`
(prefix traversal/state versus terminal observation); keep source files below
800 without mixing semantic expansion into the mechanical split.

After per-exit S0, select constructor-argument evidence (null/field/call-result)
and mixed null/new result ABI as distinct contracts. Keep field destruction,
tagged storage and selected-C parked. Re-measure the selected source invocation
at each semantic boundary; run the fixed EXE suite at its acceptance boundary,
not once per design edit. The corrected frontier table supplies later candidates
without treating another owner's internal gap as an external wait.

## Accepted — MIRBUILDER-GATE1-PER-EXIT-HOME-FLOW-D0

Census basis: read-only worker census integrated with owner reading; the
true single-terminal authorities are (1) `RootHomeFlow.terminal`
(`home_map_flow.rs:9-24`), (2) the scan's `terminal: Option<&SourceStmtSiteV1>`
gate plus `break` (`home_new_prefix_scan.rs:45,515`), (3) single
`Option<TerminalRelationV1>` per owner, (4) `issue_result_new_fault_continuation_v1`'s
`roots.body_pair().scope()` equality (`function_control.rs:133-146`), and
(5) owner-keyed `root_exits` progress with `explicit_site()==site` equality
(`ordinary_new_local_commit/root_home.rs:136-185`).

```text
Decision: one per-exit Home-flow contract replaces all five single-terminal
  assumptions together — exits keyed by explicit site, If branches walked
  with per-path state, ordered-intersection joins, named divergence.
Source authority + canonical issuer: `verify_function_completion_v1`
  (`explicit_sites()` + coverage rows) supplies exit identity;
  `core.if_regions` / `VerifiedResolvedFunctionIfControlV1` supply exact
  branch scope pairs and authorized-return membership; `scan_new_home_flow`
  remains the sole issuer of the per-exit `RootHomeFlow`.
Non-authority: `explicit_site()` callers never re-derive exits; MIR/physical
  stages never re-infer branch joins; `crossed_scopes` stays fail-closed
  empty for existing consumers; unclassified calls and field ownership
  stay unavailable.
Fail-fast boundary: `PrefixNotCovered` per uncovered site/subtree as today;
  divergent joined homes -> named `HomeFlowBranchDivergent` retaining
  per-branch state (never union, never empty substitution); duplicate
  exit prepare -> existing freeze; missing/residual exit obligations ->
  named failure at completion; unproven fault ancestry -> `SourceMismatch`.
Smallest next slice: BoxShape split of `home_new_prefix_scan.rs` (779 ->
  traversal/Local arms ~310 in place; terminal arm -> new
  `home_new_prefix_terminal.rs` ~470 via `observe_terminal_statement`
  taking running state by reference; zero semantic change), then per-exit
  S0 as the semantic slice.
Non-claims: no loops, no captures, no implicit-end exits
  (`ExplicitUnitSetWithImplicitEnd`/`ImplicitVoid` keep
  `TerminalNotCovered`); no `crossed_scopes` activation; no field
  destruction, tagged storage, Selected-C, constructor-argument widening;
  no EXE PASS or Gate-1 completion.
```

### Admitted subtree grammar (bounded, recursive)

```text
Statement := Local                     (existing arms unchanged)
           | Return                    iff site ∈ completion.explicit_sites()
           | If{cond, then, else?}     iff a verified `ResolvedIfRegionBundleV1`
                                        row exists and each branch body
                                        consists only of Statement
```

Any other statement or subtree keeps named unavailability for that region.
Return sites outside `explicit_sites()` are already rejected upstream by
`if_control`'s `authorized_return_sites` (`analyzer.rs:144`); the flow
treats a foreign Return as unavailable, never silently admitted. Condition
subtrees contribute descendant-map observations only — no `new` claims and
no Home minting under conditions.

### Branch states, joins, terminated paths

Each path carries live-home order plus the existing locals/maps state. At an
admitted `If` both branch bodies are walked from the same entry snapshot:

- A branch ending in a completing `Return` is a terminated path — its live
  homes in reverse order seal that exit's obligation; it does not join.
- Fall-through branches join by ordered intersection: a home survives only
  if live on every fall-through path in the same order. Missing `else`
  joins as the entry snapshot.
- Divergent surviving homes record `HomeFlowBranchDivergent` retaining the
  per-branch states — no union, no empty-list substitution.

### Per-exit obligations and consumption

- `RootHomeFlow.terminal` becomes an exits map keyed by `SourceStmtSiteV1`
  over `explicit_sites()`; each entry is that Return's reverse live-Home
  order or its named unavailability.
- `TerminalRelationV1` becomes per-exit; result-new co-seal runs per Return
  site and `result_prefixes` stay keyed by `OwnedExprSiteV1`, co-sealing
  only under the Return that owns the site.
- `prepare_root_home_exit(owner, site)` keeps its signature but checks
  `explicit_sites()` membership and consumes obligation `(owner, site)`
  exactly once — duplicate prepare stays a freeze;
  `root_home_exit_is_complete` requires every `explicit_sites()` member
  prepared/emitted, so missing or residual obligations are named failures.
- All `explicit_site()==site` drift checks listed by the census
  (direct_call_lifecycle, coseal, terminal_access, terminal_field_return,
  root_instance_call, finalized_root_handoff, map.rs) rewrite to
  membership/per-exit lookups in S0 — the same slice that changes the
  producer, never ahead of it.

### Fault scopes

`NewFaultContinuationV1.source_scope` takes the scope containing that
Return's `new` site; for branch returns the ancestry must be proven a
descendant of `roots.body_pair().scope()` through the if-region
`ScopeRegionPair`s, recording the crossed scope sequence
innermost→outermost on the continuation. Unproven ancestry ->
`SourceMismatch`. `ResolvedCleanupObligationsV1.crossed_scopes` stays empty;
its six fail-closed consumers are unchanged.

### Split plan — `home_new_prefix_scan.rs` (779 lines)

`MIRBUILDER-GATE1-PER-EXIT-HOME-FLOW-S0-SPLIT` — mechanical BoxShape
split, first commit of the pair:

- `home_new_prefix_scan.rs` keeps the loop head, per-statement dispatch,
  non-`Local` catch-all and the `Local` arms (≈310 lines).
- New `home_new_prefix_terminal.rs` receives the terminal arm (scan
  :45-516, ≈470 lines) as `observe_terminal_statement(...)` taking the
  running state (`locals`, `homes`, accumulators) by reference.
- Register the child in `mirbuilder_qualified_route_scope_guard.sh`;
  no semantic change rides this split.

Landed: `home_new_prefix_scan.rs` 779 -> 329 (loop/dispatch/Local arms),
`home_new_prefix_terminal.rs` 481 (`observe_terminal_statement` takes the
running state by reference; the caller still owns `break`). Registered
`NEW_PREFIX_TERMINAL` in the scope guard. Focused evidence:
`ordinary_new_coseal` 58/58, `instance_entry_home` 7/7; the wider focused
set is 635 pass / 16 red and every red reproduces at `4d659f78a0` without
the split — `raw_invocation_port_*` ×8 (`raw-invocation/missing-
expression-source-receipt` freeze), `artifact_child_rejects_retained_
unavailable`, `array_source_binding` (order-dependent), `production_
skip_while` ×2, `main_f1`, `birth_receiver`, `main_static_child`,
`qualified_call_map` — all recorded baseline debt, none touching the
extracted code path.

## Landed — MIRBUILDER-GATE1-PER-EXIT-HOME-FLOW-S0

Per-exit Home flow replaces the single-terminal authority end to end:

- `RootHomeFlow` carries `exits: BTreeMap<SourceStmtSiteV1,
  Result<RootHomeExitV1, ..>>` plus `uncovered_implicit_exit`;
  `all_exits_ready()` is the only readiness predicate; `terminal_homes()`
  stays a strict sole-exit compat accessor that fails on zero/multiple
  exits or an uncovered implicit end. Each exit row carries its homes and
  the `covered_calls` path snapshot for per-exit binding groups.
- The scan takes `exit_sites`/`uncovered_implicit_exit`, walks `If`
  branches via `home_new_prefix_branch.rs` (fork/join of
  `PrefixLocalFlow`, `path_calls`, `homes`; disagreement is
  `HomeFlowBranchDivergent`), and writes every terminal relation into a
  `(statement site -> relation)` map.
- Consumers consume `(owner, site)`: ledger relations are site-keyed,
  `*_at` accessors are the only production lookup; sole-row helpers are
  `#[cfg(test)]`-gated and return `None` on non-singletons. Emit paths
  forward `SourceStmtSiteV1` from `current_source_site_v1()`; terminal
  value progress is per-exit `BTreeMap`.
- `direct_call_lifecycle` requires `all_exits_ready()` on both caller and
  callee, finds the exact call-site relation (`call.call_site() ==
  site.site()`), and `uniform_call_result_kind` seeds from the first
  relation — empty stays `I64`, mixed classes reject.

Two regressions found and fixed inside the slice:

- `uniform_call_result_kind` seeding from the scalar default made Map
  callees compare `Map` against `I64` -> `LifecycleSourceMismatch`
  (`map_result_local_call_installs_map_class_and_map_row`,
  `call_returned_map_get_issues_relation_and_installs`).
- `SourceStmtSiteV1` carries no owner identity, so a site-only probe of
  the root relation map misrouted a child owner's same-`Body(N)` exit to
  the root value lane (`literal-source-drift` in
  `map_consumer_tests` ×2). `terminal_relation_is_indexed` now decides
  the value lane by where the relation was retained, mirroring
  `terminal_relation_for_owner_at`'s index-first lookup.

Focused evidence: the `map_ terminal_ brand_catalog direct_call home_
map_consumer ordinary_new` set is 674 pass / 7 red, identical to
detached-worktree HEAD (`3a9b98b75b`) — `main_f1`,
`global_call_route_plan` ×2, `birth_receiver`,
`qualified_call_map_argument` (`BorrowedEntryEscape`, reproduced at
parent), `mir_corebox_router_unified::map` ×2. Full lib run matches the
same worktree comparison: in-scope lanes' failure sets are identical;
remaining diffs rerun green in isolation (parallel flake).
`mirbuilder_qualified_route_scope_guard` PASS (new child files
registered); `mir_call_canonical_corridor_guard` fails identically at
HEAD (`new_expression.rs` pin — baseline debt, file untouched).
`current_state_pointer_guard` PASS.

Not claimed: constructor-argument evidence, Gate-1 EXE rerun, any
physical emission beyond the existing selected admission path.

Next design row: `MIRBUILDER-GATE1-CONSTRUCTOR-ARGUMENT-EVIDENCE-D0` —
constructor-argument evidence (null/field/call-result), per the
"Following task" contract above.

## Preserved contract boundaries

- Generic `mir_json_emit` rejects lifecycle Invoke. Invoke, normal-result and
  Fault transport use `hako.published-lifecycle-physical-program.v2` through
  the existing published backend view and lifecycle V4 route. No generic
  reader receives a permissive Invoke arm to fix an app smoke.
- Artifact observation requires source completion; claimed-but-unavailable
  child construction rejects before lifecycle coverage, including NoBirthZero.
  Every lifecycle-required instruction still needs its exact ledger binding.
  `RetainedUnavailable` is never permission to publish an artifact.
- The current physical object profile requires `PlainI64NoHook` and I64 fields.
  Names, slot tags, absent hooks and process exit cannot prove releasability.
  Owning-field release and tagged dynamic storage remain separate dependencies.
- Result-new claims own exact Return membership, prior-Home Fault cleanup,
  Invoke-shaped construction and exactly one Return of the emitted object.
  Argument/field-position constructions do not inherit this authority.
- Direct and claim-local lexical Handle calls consume the callee's sealed
  Construction result plus class claim. Received objects install as one owned
  caller Home; release accounting is per normal/Fault path. Opaque, rebound,
  parameter/nested receivers and unsupported result contracts do not gain a
  new path. Root-result ABI is separate.
- `BinaryTreesBench.run(): i64` (`5fd8f3fd17`) follows the explicit result
  contract precedent `MiWorkload.run(): i64` (`b16c3548ac`). This agrees with
  the selected source contract; it is not evidence for the unannotated input.

## Evidence retained from the previous card

### Fixed EXE suite — recorded 2026-09-29, not rerun here

Receipt source: `574d90ffc5`, plus explicit-tool resolution correction
`c82b7a415a`; complete pre-compaction record at `339674c77b:this-file`.
Profile: dev binary with LLVM 18 `opt-18` / `llc-18` / `clang-18`.
The old receipt did not pin a binary digest/build SHA for the full run; these
are record/fix commits, not a new exact-build execution claim.

Result: **5 PASS / 6 FAIL**. Passing entries were
`typed_object_newbox_min`, `typed_object_birth_min`,
`typed_object_birth_param_min`, `typed_object_method_min`, and
`real_apps_exe_boundary_probe`; the six red terminals are listed above.

Two repaired gaps in that receipt: retained result commits complete after
`ExpressionCompleted` without a nonexistent local install; explicit LLVM
fallback resolves version 18 first. Pending expression remains incomplete.
Do not replace suite failure with “six designed stops”: one is a baseline
panic and generic-ingress rejection does not observe downstream EXE behavior.

### Reviewer remediation — recorded at 339674c77b

The stale `birth_site_index_covers_field_assign_sites_while_return_position_claims`
pin and moved `install_inventoried_call_result` pin were corrected.
Six oversized parents were split; all resulting files are below 800 lines.
There are eight new child files and three previously unregistered parents in
the boundary list (not the reported nine plus three).

Commit-recorded checks: cargo check lib/tests clean; focused lifecycle 32/32,
coseal 58/58, lexical instance 15/15, recursive child 7/7, root catalog 56/57.
The remaining root-catalog failure
`array_source_binding_survives_actual_compiler_finishing_with_optimization`
reproduced in the same group on parent `053b659637` according to that commit.
This turn does not independently rerun Cargo or refresh the baseline receipt.

### Landed history tombstone

Full designs, rejected alternatives and old terminal order are in Git;
`git show 339674c77b:docs/development/current/main/investigations/mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`
recovers the complete prior card. No archive copy or new card is created.

| Commit | Landed responsibility / evidence |
| --- | --- |
| `d3a259a01c` | StringIndexOf/1 with receiver/needle text proof; focused source tests 11/11. |
| `2a68ddc5a7` | Proven flat call-free LoopCond source coverage; existing sole physical route. |
| `b16c3548ac` | MiWorkload declared result contract. |
| `660dafc7b1` | Claim-proven parameter/local instance receiver source edges. |
| `901df40d18` | Body StringLen text evidence and selected-static bucket preservation. |
| `43b706e07e` | Uniform field-write class provenance for receiver claims. |
| `179c6c67c7` | Uniform return-new result-class claims and call-result receivers. |
| `5fd8f3fd17` | Non-condition carrier coverage, root-call unavailable finishing/seal distinction, binary-trees result annotation. |
| `4e89bbb551` | Generic Invoke rejection preserved; untyped-field smoke uses physical route. |
| `e054920b05` | Unreleasable artifact boundary; mimalloc smoke physical route. |
| `c8c930d8e6` | Child retained-unavailable rejection before lifecycle coverage. |
| `30ffce464c` | Exact Return-position new claims and artifact bindings. |
| `1dad229932` | Direct Handle result ABI; lifecycle focused 29/29. |
| `493e55de4e` | Claim-local lexical Handle result; lifecycle focused 32/32. |
| `574d90ffc5`, `c82b7a415a` | Recorded suite and retained-result / explicit-tool fixes. |
| `339674c77b` | Guard pins and six source splits; focused results above. |

## Organization closeout

Current scope: this card, CURRENT_STATE, the canonical Home ABI D0 (its
projection contract changes), existing pointer guard, and the one
reviewed trailing blank line in `ordinary_new_coseal_issue.rs`. No workstream,
reference, index or restart-mirror history expansion. The guard now enforces
the existing 1,000-line budget for all three active-document pointers; this
closes an enforcement gap and introduces no new policy or semantic authority.
Validation: pointer guard and qualified-route scope guard PASS; `bash -n` and
`git diff --check` PASS. The amended pointer guard first rejected the original
1,414-line card. Eight full-guard runs in temporary copies then verified normal
separate pointers, each pointer at 1,000/1,001 lines, and an oversized shared
active/design target. All eight passed; the repository was not mutated by them.
The review's single trailing blank line is removed. Cargo/compiler/EXE checks
were not rerun; existing Rust baseline reports above remain historical.
The read-only worker's entry-authority/per-exit findings are integrated into the
selected S0 and following D0; no new observer census or semantic receipt landed.
