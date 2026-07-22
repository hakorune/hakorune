# CUT0-I0 ROOT0-CANON0 CANON-FIXTURE0 実行タスク

Status: **Active — RECURSIVE0 closed; four-route aggregate proof next**

Related:

- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-recursive0-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Build one disconnected aggregate proof for the four canonical source-bound
routes without connecting canonical ingress, drain, finalization, or external
commit. The fixture must exercise the already closed SOURCE-BIND0, LOWER0,
RECEIPT0, and RECURSIVE0 contracts together, so later DRAIN0 consumes a
single proven owner chain rather than independent unit boxes.

Routes:

```text
CanonicalAPlus
BindingSsaTrivial
BindingSsaAcyclic
BindingSsaRecursive
```

## Required matrix

```text
success:
  exact plan -> package -> active shell/collector -> receipt -> completion

foreign / mismatch:
  same-family foreign plan or source continuation is rejected before mutation

late collision:
  whole callable batch preflight leaves collector delta zero

synthetic identity:
  FunctionDraftKeyV1::Main and SyntheticConditionFn are rejected
  canonical physical symbol named condition_fn/N remains accepted

recursive parity:
  recursive source has one branded install receipt
  acyclic source has one branded absence witness
```

## Acceptance

```text
four route fixtures registered and focused test command is green
one aggregate source/shell/collector/receipt/completion chain per route
foreign pairing fails before Builder or collector mutation
canonical batch late collision leaves collector delta zero
canonical synthetic keys rejected; canonical condition_fn spelling accepted
recursive/acyclic witness family and brand remain co-sealed
production capture/drain/finalizer/external commit = 0
all touched source/check files < 800 lines
```

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_root_completion_canon_fixture0 --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_fixture0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_lower0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_receipt0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_recursive0_guard.py
```

## Stop line

CANON-FIXTURE0 closes only the disconnected four-route aggregate proof.
Completion-to-drain consumption, source-derived inventory projection in the
live executor, finalization, external commit, retry/fallback, and atomic CUT0
activation remain separate DRAIN0/CUT0 rows.
