---
Status: Landed
Date: 2026-04-27
Scope: Prune string-kernel semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/verification/string_kernel.rs
  - src/runner/mir_json_emit/plans.rs
---

# 291x-527: String-Kernel Root Export Prune

## Goal

Keep string-kernel plan vocabulary owned by `string_kernel_plan` instead of the
broad MIR root.

String-kernel types are semantic metadata. The root can keep orchestration
entry points (`derive`, `infer`, `refresh`), while verification and JSON
emitters import the owner module for the data vocabulary.

## Inventory

Removed root exports:

- `StringKernelPlan`
- `StringKernelPlanPart`
- `StringKernelPlanBorrowContract`
- `StringKernelPlanCarrier`
- `StringKernelPlanConsumer`
- `StringKernelPlanFamily`
- `StringKernelPlanLegality`
- `StringKernelPlanPublicationBoundary`
- `StringKernelPlanPublicationContract`
- `StringKernelPlanReadAliasFacts`
- `StringKernelPlanRetainedForm`
- `StringKernelPlanSlotHopSubstring`
- `StringKernelPlanTextConsumer`
- `StringKernelPlanVerifierOwner`

Migrated consumers:

- `src/mir/verification/string_kernel.rs`
- `src/runner/mir_json_emit/plans.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`

## Cleaner Boundary

```text
string_kernel_plan
  owns StringKernelPlan* vocabulary

mir root
  exports derive_string_kernel_plan
  exports infer_string_kernel_text_consumer
  exports refresh_function_string_kernel_plans
  exports refresh_module_string_kernel_plans
```

## Boundaries

- BoxShape-only.
- Do not change string-kernel plan derivation.
- Do not change verifier semantics.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `StringKernelPlan*` vocabulary.
- Verification and JSON consumers use `crate::mir::string_kernel_plan`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed string-kernel semantic vocabulary from the MIR root export surface.
- Kept string-kernel orchestration entry points available at the MIR root.
- Preserved verification behavior and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
