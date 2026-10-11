# MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-R1

Status: closed; next `MIRBUILDER-ROOT-EXIT-STATIC-TWO-EXIT-S0`
Date: 2026-10-11
Scope: behavior-preserving canonical writer preparation for the existing root graph
Related: `mirbuilder-root-exit-canonical-writer-d0-2026-10-11.md`

## Contract and target

The sole root cleanup graph algorithm remains in
`ordinary_new_admission/selected.rs`. Its raw port preserves current Plain
and terminal Call production. Add the canonical adapter required by the
selected Static V2 `size_to_bin` caller; that caller's actual cutover and
two-exit ledger batch belong to S0. No new source classifier, Home order,
cleanup algorithm, or result issuer is authorized here.

Make block reservation fallible and register canonical blocks at reservation.
Restrict the port to root graph operations. Add canonical CFG operations for
the root cleanup Invoke, prepopulated terminal Call landing, and root Fault
terminal without weakening existing I64 Call/ordinary Fault operations.
Keep logical root binding order independent of physical write order. Use
canonical edge/predecessor creation. The adapter must not insert a raw
terminated block into a canonical session. The selected caller in S0 must
seal the completed graph through canonical CFG and SSA identity together;
R1 only prepares its open blocks and verifies CFG sealing in isolation.

## Acceptance

- Existing raw root Plain/Call/Fault focused positive and negative cases pass;
  physical MIR and binding order match the R0 baseline.
- A canonical root graph test checks block creation, Jump/Invoke predecessor
  caches, projection, Home order, sealing, and `finish` against a real
  canonical session. Existing canonical stale-cache and edge-after-seal
  negatives still reject.
- Focused new negatives reject an occupied/foreign landing, wrong frame,
  duplicate block, and unsupported generic terminator at the root port.
- Original `size_class_box.hako` first stop remains
  `root-exit-unconsumed`; no selected progress or EXE pass is claimed.
- Source files remain under the 800-line hard stop. No row-specific guard.

## Retire when

S0 switches the selected Static two-exit caller to this adapter and its
prepared root batch, verifies original-source acceptance, and removes any
exclusive temporary wrapper. Shared raw Plain/Call consumers remain until
their own canonical replacement exists.

## Closeout evidence

The raw writer still allocates the same ID and performs the same accepted
block/current/projection writes. Its new preflight rejects unsupported graph
shapes before mutation. The canonical adapter registers blocks immediately,
emits Jump and root Invoke edges through `CanonicalCfgSessionV1`, and retains
the clean Return for DraftSeal. The root Invoke requires one entry Fault-frame
definition, exact open landings, and an unsealed source. The ordinary I64
Invoke and direct-Fault writers were not weakened.

`cargo test --profile quick -p nyash-rust --lib root_graph_writer::tests`
passed 3/3 after module relocation and sealed-source validation (quick
build/link 5m19s, test 0.00s). The same binary passed canonical CFG 23/23,
root cleanup graph 5/5, and raw Map consumer 3/3. The Map family's selected
C/LLVM EXE test is pre-existing ignored/informational, not a changed-code red.
`rustfmt --check`, `git diff --check`, and the current-state pointer guard
passed. All changed source files are below 800 lines; the pre-existing
803-line `resolved_lowering/mod.rs` received no change.
`cargo check --profile quick -p nyash-rust --lib` passed in 24.23s. The
unselected adapter emits two new dead-code warning groups in the non-test
build; S0 is its named production consumer, and this warning debt must be
reviewed at that cutover. Existing unrelated warnings are baseline context.

A temporary physical-input probe built from the unchanged original
`size_class_box.hako` plus trivial Main passed 1/1: the first stop remained
`ordinary-new/local-commit/root-exit-unconsumed` (quick build/link 5m20s,
probe 0.02s). The probe was removed. It preceded the final sealed-source
preflight and module-declaration relocation, neither of which connects the
selected caller. R1 claims no selected root progress, canonical SSA close,
physical EXE pass, or old-edge retirement.
