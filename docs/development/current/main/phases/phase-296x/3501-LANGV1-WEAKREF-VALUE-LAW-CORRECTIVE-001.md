# 3501 - LANGV1-WEAKREF-VALUE-LAW-CORRECTIVE-001

## Status

Done. The lifecycle value-law corrective is green and the Weak field contract
prerequisite is closed.

## Goal

Make current WeakRef runtime behavior match the accepted lifecycle law without
adding Weak field storage, carriers, MIR operations, or backend support.

```text
same target token:
  equal while Alive, Dead, or Freed
different target tokens:
  unequal even when both are Dead/Freed
WeakRef versus unified absence:
  unequal
upgrade Alive:
  BoxRef
upgrade Dead/Freed:
  unified absence
```

## Owner Boundary

```text
owner:
  lifecycle WeakRef value law
authority:
  stable target token
  InstanceBox usable lifecycle state
  unified absence law
non-authority:
  Arc strong count or successful Weak::upgrade alone
  VMValue representation, field storage, or MirType
```

This card must not introduce `WeakFieldContractOwner` or modify grammar.

## Implementation Scope

1. Introduce or expose one stable WeakRef target-token comparison primitive.
2. Make equality compare target tokens rather than upgrade outcomes.
3. Remove `dead WeakRef == Void` compatibility from Canonical semantics.
4. Make upgrade reject logically Dead/Freed InstanceBox targets even if an Arc
   can still be upgraded.
5. Restrict `WeakRef.Load` input to WeakRef or unified absence; primitive and
   strong Box values reject.
6. Keep `Void -> Void` only as the total empty-slot composition case required
   by the accepted R1 law.
7. Add focused VM-reference unit tests without adding runner routes or smoke
   families.

## Stable Tags

```text
[type/weakref_load_invalid_input]
[type/weakref_target_token_missing]
[type/weakref_lifecycle_state_drift]
[type/weakref_equality_upgrade_authority_forbidden]
```

## Fixture Matrix

```text
same_live_target_equal
same_dead_target_equal
different_live_targets_unequal
different_dead_targets_unequal
dead_weakref_not_equal_void
alive_upgrade_returns_box
dead_upgrade_returns_void_even_if_arc_remains
freed_upgrade_returns_void
void_load_returns_void
primitive_load_rejects
strong_box_load_rejects
```

## Acceptance

```text
cargo test --features vm-reference weak_ref
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Use the narrowest existing test target if the exact filter differs. Do not add
a new product or selfhost VM gate.

## Non-Claims

```text
weak_field_contract_activation = 0
weak_field_runtime_consumer_complete = 0
ownership_kernel_activation = 0
new_absence_state = 0
empty_weakref_language_value = 0
automatic_box_to_weak_conversion = 0
runtime_check_elision = 0
llvm_weak_field_contract_support = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

```text
3502 - LANGV1-WEAK-FIELD-CONTRACT-OWNER-001
```

## Closeout

```text
weakref_target_token_owner = runtime::weak_ref_value
same_dead_target_equality = 1
different_dead_target_equality = 0
dead_weakref_equals_void = 0
logical_dead_upgrade_returns_void = 1
weakref_load_void_total_case = 1
weakref_load_nonweak_reject = 1
weak_field_contract_activation = 0
vm_product_route_widened = 0
```

Validation:

```text
cargo test --features vm-reference weak_load --no-fail-fast
cargo test --features vm-reference weak_ref --no-fail-fast
cargo test --features vm-reference \
  eq_vm_uses_weak_target_tokens_after_drop --no-fail-fast
```
