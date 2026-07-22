# CUT0-I0 ROOT0-CANON0 CANON-FIXTURE0 実行タスク

Status: **Design Stop — compiler/builder owner bridge is undefined**

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

## Design-stop finding

The requested aggregate cannot honestly be implemented from the current
owners. `src/mir/compiler/source_bound_package.rs` owns
`CanonicalInvocationTokenV1`, `CanonicalInvocationBrandV1`,
`LoweredCanonicalPlanV1`, and the only SOURCE-BIND0/LOWER0 terminals. The
disconnected completion scaffold in
`src/mir/builder/canonical_root_completion.rs` owns a different
`ModuleInvocationTokenV1`/`ModuleInvocationBrandV1` and accepts only a
test-factory token plus a separately supplied plan. No production caller
connects the two chains.

The following census is therefore a hard boundary, not a missing fixture:

```text
compiler package -> builder completion bridge        = 0
builder completion production callers                = 0
compiler token -> builder token conversion            = 0
post-hoc brand rewrap that preserves provenance       = impossible to accept
```

Adding four tests with independently minted builder tokens would prove two
parallel disconnected boxes, not the required
`source -> token -> package -> LOWER0 -> shell/collector -> receipt ->
completion` chain. Synthetic-root rejection is also not reachable through the
current canonical active collector because that terminal derives the canonical
key internally; the loose collector API still accepts `Main` and
`SyntheticConditionFn` for the legacy/raw use case.

The worktree remains code-clean at this design stop. Do not add the aggregate
fixture, a test-only bridge, or a second identity authority until the attached
bridge consultation is decided.
