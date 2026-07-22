# CUT0-I0 ROOT0-CANON0 RECURSIVE0 実行タスク

Status: **Closed — RECURSIVE0 branded capability provenance complete**

Related:

- `cut0-i0-root0-canon0-receipt0-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-source-binding-consultation-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Close the canonical callable capability boundary. A recursive source must
install exactly one branded, non-Clone shell receipt; an acyclic source must
produce a branded absence witness. Source disposition, shell projection, and
completion must agree before any later drain plan can be consumed.

対象はcallable batchのacyclic/recursive routeのみである。

```text
BindingSsaAcyclic
BindingSsaRecursive
```

Raw, A+, trivial, source re-binding, DRAIN0, finalization, external commit,
fallback, retry, and atomic CUT0 activation remain parked.

## Selected implementation

```text
exact callable source continuation
  -> source-driven branded shell constructor
  -> install marker exactly once
  -> branded RecursiveCapabilityInstallReceiptV1

acyclic source
  -> no marker
  -> branded AcyclicCapabilityAbsenceWitnessV1
```

The caller cannot pass `required: bool` or a raw capability value. A duplicate
install, family mismatch, missing recursive marker, or unexpected acyclic
marker returns a typed rejected owner before completion or drain mutation.
The completion product retains the install/absence witness by value; drain
does not re-observe shell metadata to select a route.

## Acceptance

```text
recursive install receipt carries invocation brand and recursive family = 1
acyclic absence witness carries invocation brand and acyclic family = 1
recursive install terminal = 1
duplicate install = typed rejection
acyclic marker install = typed rejection
source/shell/family capability co-seal = 1
drain-time route re-selection = 0
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

Add focused acyclic/recursive install fixtures and a static census guard.
Do not connect production canonical ingress or DRAIN0 in this row.

## Closeout evidence

```text
src/mir/builder/module_lowering_shell.rs
  -> recursive and acyclic witnesses retain invocation brand and family
  -> branded shell terminal injects its own brand and installs the marker once

src/mir/builder/canonical_root_completion.rs
  -> source-driven capability install is co-sealed with completion
  -> foreign brand/family witnesses fail before completion mutation

focused fixtures:
  recursive_install_returns_exact_brand_and_family_once
  acyclic_install_returns_branded_absence_witness
  acyclic_route_rejects_a_preexisting_recursive_marker
```

Evidence passed:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_root_completion_recursive0_p0 --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_recursive0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_source_bind0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_lower0_guard.py
python3 tools/checks/lib/cut0_i0_root0_canon0_receipt0_guard.py
```

The disconnected legacy recursive publisher remains explicitly allowlisted;
no production canonical capture, drain, finalizer, or external commit was
added. CANON-FIXTURE0 is the sole next row.

## Stop line

RECURSIVE0 closes only capability install provenance. Source-derived drain
planning, completion-to-drain consumption, finalization, external commit, and
atomic CUT0 activation remain separate rows.
