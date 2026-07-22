# CUT0-I0 ROOT0-CANON0 RECEIPT0 実行タスク

Status: **Active — LOWER0 closed; collector-issued receipt retention next**

Related:

- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Close the canonical receipt boundary without connecting production ingress.
The collector must issue the exact receipt that completion consumes, and the
canonical completion witness must retain that receipt by value.

対象はcanonical four routesのみである。

```text
APlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

Raw root receipts remain owned by ROOT0-RAW0. Recursive capability branding is
the separate RECURSIVE0 row. DRAIN0, finalization, external commit, fallback,
retry, and atomic CUT0 activation remain parked.

## Selected implementation

```text
draft admission preflight
  -> collector-issued branded receipt
  -> collector + receipt inseparable product
  -> canonical completion witness retains product by value
```

The positive path may use `collect_branded` for canonical single and a new
`collect_all_branded` terminal for callable batches. Receipt branding after
collection, receipt cloning, receipt reacquisition, and loose receipt
parameters are forbidden. The receipt's collector brand and the active
invocation brand must match before completion can be sealed.

Completion remains disconnected and route-specific. It must retain the actual
BRAND0 shell, collector, source continuation, and exact receipt without
re-observing `current_module` or reacquiring a header/catalog.

## Acceptance

```text
collector-issued receipt producer = 1 per route family
collector + receipt product is non-Clone
completion witness retains receipt by value
post-hoc receipt branding = 0
receipt Clone/Arc/reacquisition = 0
foreign collector/receipt brand = typed rejection before mutation
receipt mismatch leaves collector prefix unchanged
production capture/drain/finalizer/external commit = 0
all touched source/check files < 800 lines
```

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_lower0_guard.py
```

Add a focused receipt fixture for successful exact retention and foreign /
duplicate receipt rejection. Do not add production consumers or drain logic in
this row.

## Stop line

RECEIPT0 closes only collector-to-receipt provenance and by-value retention in
canonical completion. Recursive install receipts, source-derived drain
planning, finalization, external commit, and atomic CUT0 activation remain
separate rows.
