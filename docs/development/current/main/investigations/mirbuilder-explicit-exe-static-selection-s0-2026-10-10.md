# Explicit EXE Static selection S0

Status: closed S0
Date: 2026-10-10
Scope: MIRBUILDER-EXPLICIT-EXE-STATIC-SELECTION-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-closed-app-static-selection-s0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-selected-static-caller-cohort-s0-2026-10-10.md

## Decision

Source authority + canonical issuer: the runner's selected `emit_exe` branch
owns EXE intent. `NormalCompileRequestV1` carries that intent through the
normal lifecycle to the existing callable-package issuer. The sealed source
call inventory remains the sole authority for reachability inside the
explicitly closed App executable.

Non-authority: a parsed `Main.main`, App mode, published MIR view, or
backend-specific consumer does not prove EXE intent. Package/test, MIR JSON,
OBJ, VM, and other non-EXE requests retain all source declarations by
default. A typed request flag is set only by the runner's EXE branch.

Fail-fast boundary: closed selection applies only when the sealed source has
an App Main and the same complete call inventory. A Script EXE stays total.
Ordinary package issuance does not omit
source methods or suppress their incoming obligations. No retry into the old
route follows a failed closed selection.

Smallest next slice: correct the physical selection scope, leaving the
selected-caller cohort work uncommitted and separate. Refactor the oversized
issuer by moving the direct-call loan helper without changing behavior, then
thread explicit EXE intent and revalidate both total package and closed EXE
paths. The prior S0 selected from `Main.main` alone; this scope error caused
package tests inspecting uncalled methods to lose their rows.

Non-claims: no new caller classification, parameter transport, Page edge,
Loop Mul, result Completion, or generic dead-code elimination.

## Acceptance

- Existing package tests that inspect uncalled App Static methods retain
  their rows, including `map_call_argument_flow_tests`.
- EXE mode omits the unreachable `LayoutBox.accepts/1` on unchanged
  mimalloc-lite while retaining its original declaration and source claim.
- MIR JSON and non-EXE package consumers remain total. Compare broad package
  tests with the pre-selection baseline and classify every red.
- Run focused positive/negative tests and the current-state pointer guard.

## Evidence

Pre-selection baseline at `6f90996ea3`: package filter 995 passed, 3 failed.
The three reds are `birth_receiver_non_escape_rejects_unproven_uses_before_row_publication`,
`main_static_child_port_consumes_all_role_rows_once`, and
`qualified_call_map_argument_reaches_the_named_capability_boundary`.
`map_call_argument_flow_tests::call_argument_map_issues_call_slot_destination_row`
passed. The checkout at `960880c040` with uncommitted selected-caller work
showed 891 passed and 107 failed in the same filter. The exact committed-tip
contribution remains to be measured. Corrected quick lib test build passed;
the explicit closed-App/total-source paired lifecycle test passed 1/1, and
the unchanged imported mimalloc source test passed 1/1 after selecting its
EXE scope explicitly. The package filter returned 995 passed and the same
three baseline failures (0 new reds). `map_call_argument_flow_tests` passed.
Current-state pointer guard and `git diff --check` passed.

The quick `hakorune` CLI (SHA-256
`e8ebb0e177fdb159f7a511c09216e3856ec38e0bd9e71eac3dbca2c5bbd3e03a`)
emitted MIR JSON for a small App containing two uncalled `Helpers` methods;
both `Helpers.live/1` and `Helpers.dead/1` appear. The unchanged
`apps/mimalloc-lite/main.hako` EXE probe exited 1 at the prior
`ordinary-new/borrowed-entry/source-only-object-actuals` boundary, with no
EXE written. The quick CLI build took 4m10s; the unchanged-source probe took
0.05s. The command used `NYASH_DISABLE_PLUGINS=1`,
`HAKO_BACKEND_COMPILE_RECIPE=pure-first`, `HAKO_BACKEND_COMPAT_REPLAY=none`,
`hakorune --backend mir --emit-exe <out> --emit-exe-nyrt
<lifecycle-kernel/release> apps/mimalloc-lite/main.hako`. This S0 restores
artifact scope; it does not solve that next
transport frontier. The synthetic App EXE attempt hit missing `ny-llvmc`
after MIR compilation; it is an informational backend-toolchain observation,
not acceptance evidence for this selected lifecycle route.
