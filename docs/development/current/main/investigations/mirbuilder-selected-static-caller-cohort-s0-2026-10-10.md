# Selected Static caller cohort S0

Status: closed S0
Date: 2026-10-10
Scope: MIRBUILDER-SELECTED-STATIC-CALLER-COHORT-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-closed-app-static-selection-s0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-explicit-exe-static-selection-s0-2026-10-10.md

## Contract

Prerequisite: `8b4339ecfb` confines closed-App physical selection to
explicit EXE requests. The reusable package issuer retains the complete
source cohort; this selected incoming view applies only after the EXE scope
was chosen. The earlier uncommitted implementation was replayed onto the
corrected branch; the original checkout remains untouched for handoff.

Decision: `QualifiedStaticCallClaimIndexV1` remains the complete source-call
authority. The catalog's same-brand omitted parser identities, checked
against the resolved batch and selected map by
`validate_cataloged_source_co_seal_v1`, issue the exact omitted caller-owner
set. The existing incoming walk consumes that set once; it performs no new
source scan and leaves the claim index intact. Unknown, foreign, mismatched,
or non-Static omitted identities fail before incoming projection. All-source
requests have an empty omitted set.

Keep the complete original Static claim index and declarations. Derive the
exact omitted App Static caller batch slots from the already co-sealed
source-backed catalog and resolved batch. The ordinary incoming walk must
exclude only those caller slots from executable observations. It must still
reject an unknown or unmatched unselected batch declaration and every
unsupported selected caller. No new source scan, physical selection map,
transport packet, or Static seed policy is allowed.

The same-brand omitted loan needs parser identity, canonical Static key, and
batch slot correspondence. The original `accepts -> class_id` row remains in
`QualifiedStaticCallClaimIndexV1`; `BorrowedIncomingInventoryV1` is the
selected executable caller view. Any current method called a complete
original cohort must be renamed or documented as selected-only.

## Acceptance

- On unchanged mimalloc-lite, retain all 15 Heap source callers and the
  selected `Heap -> LayoutBox.class_id` edge. Retain the original
  `LayoutBox.accepts -> class_id` claim row, but it contributes no executable
  incoming/context veto. The selected class_id seed may advance only if all
  other existing conditions pass.
- Adding a real incoming call to `LayoutBox.accepts` reselects it and restores
  its CurrentOwner/context veto. A selected unsupported caller continues to
  veto; foreign or mismatched identity fails closed. Script/library behavior
  remains total.
- Run focused positive/negative tests and the required family guard, then
  probe unchanged mimalloc-lite to record the actual first stop. A zero-test
  filter or prior EXE probe is not acceptance.

Non-claims: no executable tagged actual, result Completion, Page edge, Loop
Mul, old-edge retirement, or MirBuilder goal completion.

## Evidence

The earlier uncommitted implementation was replayed on the EXE-scope
correction at `753c439b99`. Two fixture-only `@rune Public` adaptations were
removed because reusable package requests now keep all source methods.
The unchanged imported mimalloc source test passed 1/1: 15 Heap callers stay
in the source inventory; selected `LayoutBox.class_id` sees one qualified
incoming caller and candidate integer agreement, while the original
`accepts -> class_id` claim remains. Static source-domain 11/11, selected
actual 1/1, closed-App omission negatives 2/2, and paired physical
selection 1/1 passed. The package filter returned 995 passed and exactly
the three known baseline failures. One attempted source-domain filename
filter selected zero tests and was not counted; the exact test-name family
filter then ran 11/11. The current-owner condition test separately pins the
selected caller's context veto; the closed-App source-inventory negatives pin
reselection when a real incoming call appears, so no new duplicate test was
added.

The quick `hakorune` CLI build took 2m53s (SHA-256
`34c6f561e9a2b20e783b774783d6daf1de9441048edb89cadbb43138d2bad8ce`).
The unchanged mimalloc-lite EXE probe used `--backend mir --emit-exe
<out> --emit-exe-nyrt <lifecycle-kernel/release>` with
`NYASH_DISABLE_PLUGINS=1`, `HAKO_BACKEND_COMPILE_RECIPE=pure-first`, and
`HAKO_BACKEND_COMPAT_REPLAY=none`. It exited 1 in 0.05s at the same
`ordinary-new/borrowed-entry/source-only-object-actuals` first-stop; no EXE
was written. This S0 resolves the `LayoutBox.class_id` candidate's omitted
CurrentOwner veto but does not identify or solve the remaining executable
actual frontier. Page actuals and other caller/source conditions need a new
first-exclusion observation before construction continues.
