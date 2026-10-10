# MIRBUILDER-BOOL-CURRENTOWNER-ENTRY-D0

Status: MIRBUILDER-BOOL-CURRENTOWNER-ENTRY-S0 closed; physical Static coverage remains open
Date: 2026-10-11
Scope: unchanged `SizeClassBox.accepts_usize/1 -> accepts/1` borrowed entry
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-mixed-cohort-root-entry-d0-2026-10-11.md
  - docs/development/current/main/investigations/mirbuilder-bool-literal-terminal-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source.rs
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source_drafts.rs

## Current stop and predecessor

The predecessor mixed cohort has a verified package checkpoint at
`024af5d8aa` with pointer repair at `f974f16d82`. Its original-source
positive, missing-sibling route/lending negative, and 12-test Static packet
family pass. Its physical acceptance remains **open**. With protected Eq WIP
copied only into a disposable worktree, the original SizeClassBox published
probe leaves the former `size_to_bin_usize/1` `call-entry-missing` stop and
fails while building `accepts/1` at
`[ordinary-new/borrowed-entry/entry-values-missing]`. This is before the
physical ABI callback, so it does not prove the mixed root Call emitted.
The diagnostic result is `/tmp/hako-static-mixed-physical-final-20261011.log`.

The original `accepts_usize(size: usize)` calls `me.accepts(size)`. Its
CurrentOwner source has an `ExactBool` result, whereas
`seed_static_transport_owners_v1` currently admits only a qualified source
or a narrow CurrentOwner `ExactI64` local initializer. The existing result
solver owns `ExactBool`; the mixed packet and emitted ValueIds do not own
the caller's entry contract.

There is a second dependency to resolve. In
`finish_ingress_from_drafts_v1`, the transport-owner fixed point removes an
owner whose opaque formal forwards to a destination outside
`transport_owners`. The mixed `size_to_bin` destination is currently retained
as `source_only_definitions`; merely seeding `accepts` could still prune it.
This must be checked against the exact inventory and handled by the same
canonical source/entry issuers, with no parallel route or fabricated entry.
The current `prepare_static_source_actuals_v1` also stages original
CurrentOwner candidates only for zero-argument or I64 result sources, so
the Bool incoming actual needs an explicit checked source arm. A source-only
entry route may keep both owners outside the global transport fixed point,
but it must prove its complete incoming actual and borrow the existing
entry issuer rather than pretending the I64 seed applies.
The raw incoming census also marks the return-call context unsupported;
admission must prove this exact return shape without suppressing that veto
for unrelated calls.

## Decision

```text
Decision: keep accepts source-only and complete one exact Bool CurrentOwner
  entry cohort at the existing borrowed-entry issuer. Do not seed it into
  the global transport-owner fixed point.
Source authority + canonical issuer: original CurrentOwner incoming inventory,
  resolved BodyShape singleton Return/value relation at the original call
  site, ExactBool result disposition, declared USIZE caller formal/signature,
  target opaque formal/origin/checked input, Completion and target signature;
  existing borrowed-entry issuer records values.
Non-authority: mixed packet as a substitute for accepts entry, caller ValueId,
  I64-only CurrentOwner transport, and physical Bool result.
Fail-fast boundary: wrong site/target/formal/ABI/result, incomplete incoming
  cohort, unsupported non-Return context, missing entry source, or attempt
  to consume entry-only proof as a Static packet/Bool Invoke.
Smallest next slice: BOOL-CURRENTOWNER-ENTRY-S0 stages one source-only Bool
  scalar actual, completes an entry-only proof, then feeds the existing
  borrowed-entry owner view and ordered-actual issuer.
Non-claims: no Bool physical return/Invoke, no whole-source publication, no
  mixed S0 closeout, no mimalloc-lite EXE success.
```

S0 acceptance: the unchanged-source `accepts` entry obtains a source-backed
value through the existing borrowed-entry issuer and reaches the next named
Bool boundary; a wrong Return/value site, target, result, formal, ABI,
missing incoming row or non-Return context refuses before entry. The
entry-only phase cannot be lent as a Static packet. Reuse the existing
borrowed-entry and result tests and the original-source probe; add only
independent missing conditions. A red whole-source probe at the next
physical Bool frontier is expected, not a PASS.

## Read-only audit result

The canonical incoming inventory already retains the CurrentOwner
`ExactBool` source and the matching `accepts_usize -> accepts` row. No second
source scan is needed. The narrow implementation direction is an entry-only
completed Bool single-caller cohort while `accepts` remains source-only;
adding it to the global transport fixed point would require promoting the
mixed `size_to_bin` destination as well and would mix responsibilities.

Three existing seams need one integrated Decision before construction:

- `prepare_static_source_actuals_v1` must stage this exact Bool CurrentOwner
  scalar actual from the original row; the present gate is I64-only.
- The raw inventory records non-initializer Static contexts as unsupported.
  The exact `return me.accepts(size)` site needs source-shape certification;
  unrelated return/callback contexts must retain the veto.
- `checked_borrowed_entry_incoming` requires an executable actual, and
  `incoming_targets()` has an I64-only CurrentOwner guard. An entry-only
  completed proof must satisfy entry checking without granting a Bool Static
  packet or Invoke. The checked entry owner view must consume that proof.

The Return/value-site proof is the resolved `BodyStatementShapeV1::Return`
value site plus its `BodyShapeRelationV1` Value child, joined to the exact
original CurrentOwner call site. It does not infer a return ABI. The
entry-only phase may pass the borrowed-entry ordered-actual correspondence
check only when its completed source proof is present; the generic physical
packet lender and Bool Invoke keep their existing refusal. The predecessor
mixed S0 physical acceptance stays open throughout.

## S0 evidence in progress

The exact `return me.accepts(size)` call never reaches the Home expression
walk's actual observer. For this one source-only Bool cohort, the entry
finisher borrows the existing incoming row and the declared `usize` caller
formal, checks that the resolved argument site names that binding, then
passes a scalar candidate to the existing Static source-actual issuer.
The completed phase is accepted only by the borrowed-entry ordered-argument
checker and is refused by the physical Static packet lender. The earlier
attempt to add this site to `static_source_sites` was removed: that set also
selects physical call handling and is not the Bool entry authority.

At the WIP revision before the responsibility-only module split,
`cargo check --profile quick -p nyash-rust --lib` passed, the original-source
focused Bool entry test passed 1/1, the borrowed-entry family passed 17/17,
and the Static packet family passed 12/12. The post-split `cargo check` also
passed. The post-split `borrowed_formal_entry::` focused family passed 58/58
under the quick profile, including the original-source positive and existing
negative cases (`/tmp/hako-bool-entry-final-family-20261011.log`).

The unchanged SizeClassBox source with only a test `Main` suffix was probed
in disposable worktree `/mnt/workdisk/hako-bool-entry-physical-probe-20261011`
at base `f9cc86cf86` plus copied Bool WIP and the separately protected Eq
overlay. Command: `CARGO_TARGET_DIR=/mnt/workdisk/hako-field-array-get-i64-target
CARGO_BUILD_JOBS=4 cargo test --profile quick -p nyash-rust --lib
imported_size_class_accepts_publishes_ordered_static_integer_eq`.
Build/link took 7m55s; the test ran in 0.02s. The physical probe is still
red, but its first stop moved from `accepts/1 borrowed-entry/entry-values-missing`
to install `IncompleteOrdinaryNewCoverage` (`/tmp/hako-bool-entry-physical-20261011.log`).
This proves the previous entry stop was passed. The generic coverage failure
does not by itself identify the remaining Bool physical obligation or prove
the mixed root Call emitted. It needs a separate owner census before the next
physical slice is selected; the predecessor mixed physical acceptance is open.

The first dedicated negative was red: a Bool call in a local initializer
issued a package even though the new entry must require the exact Return
value site (`/tmp/hako-bool-entry-negative-20261011.log`). The existing
inventory's initializer context exception was broader than this entry
contract. The inventory and entry finisher now call one shared exact
Return/value relation predicate; the finisher checks it whether the actual
was observed by Home or sourced directly. This change-induced red is fixed:
the post-fix borrowed-entry family passes 59/59, including the two new
rejection cases and the unchanged-source positive
(`/tmp/hako-bool-entry-final-family-2-20261011.log`). The final-revision
physical probe repeated the same `IncompleteOrdinaryNewCoverage` first stop
at the final WIP source revision (`/tmp/hako-bool-entry-physical-final-20261011.log`;
quick build/link 5m54s, test 0.02s). It is an informational next frontier,
not a green whole-source acceptance. Disposable-only diagnostics identified
`local_call_bindings_consumed=false` as the lone failing top-level ledger
condition (`/tmp/hako-bool-entry-coverage-diag-20261011.log`). Recorded owner
slots 4, 5 and 7 have Static local-call groups but no exit entries; source
order identifies them as `bin_size`, `size_to_bin` and `good_size`. Owner
slot 10 (`accepts`) has an emitted exit entry containing its local-call
groups (`/tmp/hako-bool-entry-coverage-owner-20261011.log`). Diagnostic
prints exist only in the disposable worktree.

S0 closes the one source-backed Bool borrowed entry and its fail-fast
boundary. It does not close the mixed cohort's physical acceptance, Static
local-call consumption, whole-source publication or mimalloc-lite EXE.
