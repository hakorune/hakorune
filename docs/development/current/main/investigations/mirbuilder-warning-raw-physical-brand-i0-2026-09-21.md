---
Status: closed__2026-09-21__WarningRawPhysicalBrand__DeletedAndVerified
Task: MIRBUILDER-WARNING-RAW-PHYSICAL-BRAND-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i108-2026-09-21.md
Implementation permission: true for the production-zero raw physical brand accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I110
---

# Warning cleanup: unused raw physical brand accessor

## Six-line brief

```text
Decision: delete CompletedRawRootBatchPhysicalV1::brand; the physical owner
already validates invocation brands during sealing and no caller reads this
convenience accessor.
Source authority + canonical issuer: src/mir/builder/raw_root_physical/
root_batch_terminal.rs; selected from the I108 quick-profile warning census.
Non-authority: warning suppression, cargo-fix, a new brand projection, or a
change to raw physical sealing and drain semantics.
Fail-fast boundary: any named caller, raw-root focused red, or warning-count
mismatch rejects the row.
Smallest next slice: remove one accessor and run raw-root focused tests plus
sequential warning checks.
Non-claims: no raw route switch, parser expansion, fallback, old-edge deletion,
or LegacyCallV0 retirement.
```

## Caller census

I108 reproduced lib **1,700** and lib-test **551** warnings. The method
`CompletedRawRootBatchPhysicalV1::brand` has no production caller. One raw-root
environment test used it only as a convenience assertion; that test now
verifies the existing `prepare_raw_drain` handoff instead. No source, tool, or
tracked-document caller remains, and all brand equality checks remain in the
sealing and finalization paths.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::compiler::raw_root_decl_access_p0
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filter must execute nonzero tests and pass. The warning refresh must
reduce lib warnings from **1,700** to **1,699** and keep lib-test at **551**;
no `#[allow]` or unrelated raw-root changes are permitted.

## Closeout evidence

The unused accessor was deleted and its single test-only convenience assertion
was replaced by the existing `prepare_raw_drain` handoff check. The focused
raw-root suite passed **15/15**. Sequential validation passed:

```text
cargo check --profile quick --lib -j4      -> lib warnings 1,699
cargo test --profile quick --lib --no-run -j4 -> lib-test warnings 551
cargo fmt --all -- --check                 -> pass
git diff --check                            -> pass
bash tools/checks/current_state_pointer_guard.sh -> pass
```

No production caller, new allowance, fallback, or physical-route change was
introduced. The next action is a baseline refresh before selecting another
warning cohort.
