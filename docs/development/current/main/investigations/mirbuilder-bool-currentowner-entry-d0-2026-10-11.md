# MIRBUILDER-BOOL-CURRENTOWNER-ENTRY-D0

Status: design_stop; source/fixpoint authority under audit
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

## Decision brief under audit

```text
Decision: pending one integrated source/fixpoint/entry mapping.
Source authority + canonical issuer: original CurrentOwner incoming inventory,
  ExactBool result disposition, declared USIZE formal, target opaque formal,
  Completion and physical signature; existing borrowed-entry issuer.
Non-authority: mixed packet as a substitute for accepts entry, caller ValueId,
  I64-only CurrentOwner transport or physical Bool result.
Fail-fast boundary: wrong site/target/formal/ABI/result, incomplete incoming
  cohort, unproved forwarding destination, or missing entry source.
Smallest next slice: to be selected after the read-only fixpoint audit.
Non-claims: no Bool physical return/Invoke, no whole-source publication, no
  mixed S0 closeout, no mimalloc-lite EXE success.
```

Audit acceptance: identify the exact original incoming and forwarding graph,
which owner enters the fixed point under which proof, whether the existing
borrowed-entry source/actual issuer can consume it, and a positive plus
wrong-identity/missing-source negative that reaches the next named Bool
boundary. Do not start semantic construction while this mapping is open.

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

The issuer remains the existing borrowed-entry path. The next design step
is to name the exact Return/value-site proof already issued by resolved
source/Completion, and specify how the entry-only phase is consumed by
entry checking while physical call/result remain refused. Once that mapping
is fixed, select one S0 with original-source entry positive, wrong
site/target/result/formal/ABI and missing-row negatives, and the next named
physical Bool frontier. The predecessor mixed S0 physical acceptance stays
open throughout.
