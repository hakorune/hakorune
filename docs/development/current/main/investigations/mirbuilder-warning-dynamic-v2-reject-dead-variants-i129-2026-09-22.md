---
Status: closed__2026-09-22__WarningDynamicV2RejectDeadVariants__DeletedAndVerified
Task: MIRBUILDER-WARNING-DYNAMIC-V2-REJECT-DEAD-VARIANTS-I129
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i128-2026-09-22.md
Implementation permission: true for deleting the four declaration-only variants
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I130
---

# Warning cleanup: unconstructed Dynamic V2 metadata reject variants

## Six-line brief

```text
Decision: delete four enum variants that have no issuer, consumer, or test.
Source authority + canonical issuer: DynamicV2AotCallMetadataRejectV1 and its
  existing project_dynamic_v2_aot_call_metadata owner.
Non-authority: warning text alone, the similarly named cursor reject enum,
  compatibility routes, or a new error taxonomy.
Fail-fast boundary: any non-declaration reference, focused red, new warning,
  or changed rejection mapping cancels the slice.
Smallest next slice: remove exactly four variants from one enum and verify the
  provider-admission focused tests plus stable guards.
Non-claims: no provider ABI change, no admission behavior change, no old-edge
  retirement, no warning suppression, and no deletion of the cursor variant.
```

## Census and delete set

The selected warning is emitted at
`src/box_callable/provider_admission/call_metadata.rs:34-44`. Exact search
across `src`, `tests`, `tools`, `lang`, `apps`, and tracked development docs
found only these declarations:

```text
DynamicV2AotCallMetadataRejectV1::MissingCallRole
DynamicV2AotCallMetadataRejectV1::DuplicateCallRole
DynamicV2AotCallMetadataRejectV1::MissingNormalResult
DynamicV2AotCallMetadataRejectV1::DuplicateNormalResult
```

No production caller, test assertion, match arm, guard, or documentation
depends on them. `DynamicV2RecipeOperationCursorRejectV1::MissingCallRole` is a
separate live variant and is explicitly excluded.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib box_callable::provider_admission
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record focused test count and lib/lib-test warning counts. No `#[allow]`, test
deletion, replacement error, or route change is allowed.

## Closeout

The four declaration-only variants were deleted from
`DynamicV2AotCallMetadataRejectV1`; the similarly named cursor variant and all
live provider rejection variants remain unchanged. The provider-admission
focused filter passed **6/6**. `cargo fmt --all -- --check` passed,
`cargo check --profile quick --lib -j4` passed with lib **1,690** warnings,
and `cargo test --profile quick --lib --no-run -j4` passed with lib-test **545**
warnings. The pointer guard and `git diff --check` passed. No `#[allow]`, test
deletion, replacement error, route change, or ABI change was introduced.
