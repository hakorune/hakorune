# MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-R1

Status: selected execution slice
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
canonical edge/predecessor and SSA sealing. The adapter must not insert a
raw terminated block into a canonical session.

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
