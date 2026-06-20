# 296x-1464 RUST-LIFECYCLE-OWNERSHIP-PROJECTION-REFERENCE-001

Status: closed
Date: 2026-06-20

## Purpose

Document the practical answer to:

```text
Can the converter translate Rust ownership / borrow / Drop into .hako?
```

The answer is yes only through verified lifecycle projection:

```text
rustc facts -> HakoLifecyclePlan -> verifier -> converter/emitter
```

## Output

Reference manual:

```text
docs/reference/architecture/rust-to-hako-lifecycle-projection.md
```

Design SSOT pointer updated:

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
```

## Decision

```text
converter_can_emit_ownership_aware_hako=1
converter_is_policy_owner=0
rust_syntax_rewrite_model=0
rustc_lifecycle_facts_required=1
hako_lifecycle_plan_required=1
verifier_required=1
skeleton_route_lifecycle_parity_claim=0
```

## Task Breakdown

```text
1. Keep RustSubsetModule-v0 as structure / skeleton input.
2. Keep RustLifecycleFacts-v0 as the semantic sidecar.
3. Keep HakoLifecyclePlan-v0 as the Hako-owned projection result.
4. Require VerifierResult before lifecycle-aware emission.
5. Keep skeleton route and lifecycle-aware route separate.
```

## Acceptance

```text
reference_manual_exists=1
reference_manual_lists_owners=1
reference_manual_lists_stop_lines=1
design_ssot_links_reference=1
new_hako_lifetime_syntax_added=0
implementation_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_add_rust_lifetime_syntax=1
do_not_let_adapter_choose_hako_policy=1
do_not_claim_lifecycle_parity_on_skeleton_route=1
do_not_start_resolver_or_emitter_implementation=1
```
