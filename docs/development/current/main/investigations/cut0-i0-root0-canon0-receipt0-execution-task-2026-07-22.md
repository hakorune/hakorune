# CUT0-I0 ROOT0-CANON0 RECEIPT0 実行タスク

Status: **Closed — RECEIPT0 collector/receipt products and completion retention passed; RECURSIVE0 next**

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
python3 tools/checks/lib/cut0_i0_root0_canon0_receipt0_guard.py
```

Add a focused receipt fixture for successful exact retention and foreign /
duplicate receipt rejection. Do not add production consumers or drain logic in
this row.

## Stop line

RECEIPT0 closes only collector-to-receipt provenance and by-value retention in
canonical completion. Recursive install receipts, source-derived drain
planning, finalization, external commit, and atomic CUT0 activation remain
separate rows.

## Implementation result

Canonical single and callable-batch collection now produce non-Clone products
that own the branded physical collector and its collector-issued receipt. The
route-specific completion states consume those products by value; the exact
receipt is retained in the canonical root witness instead of being inspected
and dropped. Admission and duplicate failures return the branded collector
owner before any prefix mutation, while unbranded collectors are rejected by
typed error.

Evidence:

```text
RECEIPT0 focused fixtures: 3 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib: passed
RECEIPT0 guard: passed
SOURCE-BIND0/LOWER0 guards: passed
current-state pointer guard: passed
git diff --check: passed
```

The next executable row is `RECURSIVE0`: brand the recursive shell install
receipt and acyclic absence witness without enabling drain or production
canonical ingress.
