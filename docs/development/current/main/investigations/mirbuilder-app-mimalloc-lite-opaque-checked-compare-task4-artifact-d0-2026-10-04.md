# mimalloc-lite opaque checked-compare task-4 artifact D0 (dominated-view use coverage)

Status: design stop — census recorded; Decision pending on the
  dominated-view use-coverage order (`.set` arg / `+` operand / `new`
  arg) and the Result-claim `home_prefix` contract.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARTIFACT-D0`
Related:
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-task4-d0-2026-10-03.md
  (parent; dominated-view series and the resolved Invoke emit census);
  mirbuilder-app-mimalloc-lite-opaque-checked-compare-normal-integer-d0-2026-10-03.md
  (grandparent; view contract and the task-4 `.set`/`+`/`new` scope
  statement);
  mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md (bundle
  inventory — slice 9 OPAQUE-SCALAR-POSITION keeps the `OpaqueHandle`
  rejection, slice 11 RESULT-NEW-OBJECT-ARGS names the `return`);
  mirbuilder-final-pipeline-ssot.md
  (`MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`).

## Frontier chain (observed 2026-10-04)

`apps/mimalloc-lite --emit-mir-json` now reaches only the designed
negative `unsupported terminator Invoke` — the generic MIR JSON lane
is exhausted by Decision
(`MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`: do not add `Invoke`
to `src/runner/mir_json_emit`; the lifecycle triplet serializes
through `published_backend_view/physical_program_json.rs`). The real
production lane is `--backend mir --emit-exe` (pure-first → lifecycle
V4 → LLVM C API → EXE, per `tools/smokes/.../mimalloc_lite_exe.sh`).
First observation on that lane:

```text
[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]
```

## Attribution (read-only worker census + temporary diagnostic, reverted)

- `HakoAllocPage.allocate/1` block 43 emits
  `Call{BirthConstructor HakoAllocHandle.birth/3, receiver:%130}` —
  the `return new HakoAllocHandle(me.page_id, block_id,
  requested_size)` at `page_heap_box.hako:102`. `lifecycle_bindings`
  for the owner is empty (`expected=[]`).
- The claim exists and is correctly owned:
  `LocalCommitV1::Result(NewResultCommitV1)` taken by
  `try_take_result_new_claim` (`raw_ordinary_new_claim.rs:511-535`,
  `ordinary_new_ledger.rs:211-268`) — but
  `prepare_result_new_emission` found
  `home_prefix = Err(PrefixNotCovered(Body(8)))`
  (`emission_prepare.rs:140-189`) → `NewEmissionProgress::
  RetainedUnavailable`.
- `Body(8)` = `me.requested_sizes.set(block_id, requested_size)`
  (L93): `requested_size` is an unannotated formal → installed
  `StoredLocal::Handle` → `OrdinaryObservation::Handle`, which
  `argument_subtree_neutral`
  (`home_new_prefix_field_call.rs:96-189`) deliberately rejects
  (bundle slice 9: "Keep rejection; select checked
  operation/borrow/Fault contract before widening").
- A retained claim emits via the raw lane (`lower_ordinary_raw_new_
  with_port_v1`, `ordinary_new_admission.rs:19-121`): raw
  `MirInstruction::NewBox` (not lifecycle-counted) plus
  `Call(BirthConstructor)` (lifecycle-counted,
  `invoke.rs:14-24`) — recording zero bindings by design. The
  coverage gate then correctly rejects: this freeze is the designed
  fail-closed terminal, not a bug.
- Pinned census test `page_heap_fixture_result_claim_census`
  (`brand_catalog_mixed_result_class_tests.rs:790-848`): 9 retained
  result claims; the `HakoAllocHandle` row is `construction()` +
  `argument_rows()` Ok, `home_prefix()` Err(`Body(8)`).
- `Emitted{bindings}` can never be empty
  (`record_new_emission`, `emission_prepare.rs:240-241`), so this is
  a retained claim, not a truncated record.

## Why the emit lane missed it

`--emit-mir-json` runs finishing validation with `artifact=false`
(`validate_after_compiler_finishing`) — `validate_artifact_lifecycle_coverage`
never executes, so "the whole app passes" on that lane ends at the
designed negative only. The production lane runs artifact coverage
per function; `allocate` is the first uncovered site.

## allocate's remaining dominated-view uses (parent-card scope)

`requested_size` (OpaqueHandle; view lent by `> me.block_size` at
L75, region dominated on the Normal edge):

- `.set` argument L93 — `Body(8)`, the direct blocker.
- `+` operand L96 (`me.requested_bytes + requested_size`).
- `new` constructor argument L102 (also bundle slice 11
  RESULT-NEW-OBJECT-ARGS).

Prefix statements whose coverage is undetermined: `.get` reads
L84-85 (`me.free_stack.get`, `me.use_counts.get`), `.set` L91-92,
`me.` field writes L83/88/94-96/99, `if` L87/98 — which of these
`home_prefix` counts as covered vs blocks on is the next census
question.

## Open design questions (Decision needed)

1. Use-kind admission order under the dominated view: `.set` arg /
  `+` operand / `new` arg are distinct admission arms in
  `borrowed_call_uses` and operand profiles; the parent card names
  all three as task-4 scope but ordering is undecided.
2. `home_prefix` coverage contract: what does `compute_home_prefix`
  require for `.get`/`.set`/`me.` writes/`if` statements — claimed
  rows, scalar writes, or free statements? Bounded answer needed
  before any slice can promise a named terminal.
3. Child-vs-root terminal asymmetry (worker secondary finding): the
  child `RetainedUnavailable` check inspects only `Ordinary` rows
  (`root_validation.rs:82-97` via `row.ordinary()`), so a `Result`
  row falls through to `artifact-unowned-lifecycle-site` instead of
  `artifact-source-unavailable`; the root checks both variants
  (`ordinary_new_local_commit.rs:611-621`). Token parity only — the
  outcome is fail-closed either way.
4. Smoke pin update: `mimalloc_lite_exe.sh` still greps the stale
  pinned terminal `emission-binding-drift`; the designed terminal is
  `unsupported terminator Invoke` again (progress marker).

## Smallest next slice proposal

The `.set`-arg view use is the first blocker (`Body(8)`), but the
`home_prefix` census (Q2) decides whether a bounded slice reaches a
named terminal without covering the whole prefix. Proposed first
slice: census `compute_home_prefix`'s covered set for `allocate` and
pin it — design information, not admission — then order the
view-use-kind slices.

## Non-claims

No `Invoke` JSON vocabulary, no `OpaqueHandle` widening, no `.get`/
`.set` named-array lane change, no new builtin-call authority, no
object-typed `Alias` read, no app EXE PASS or production switch claim.
