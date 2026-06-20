# 296x-1387 MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001

Status: open
Date: 2026-06-20

## Purpose

Create the first narrow lifecycle projection pilot for the MirBuilder
`BindingContext` family.

The pilot verifies that the lifecycle vocabulary can describe a real
MirBuilder-owned structure without translating Rust lifetime syntax into Hako.

## Selected By

```text
296x-1386-RUST-TO-HAKO-LIFECYCLE-EMITTER-CONTRACT-000
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
docs/development/current/main/design/rust-lifecycle-facts-vocab-v0.md
docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md
docs/development/current/main/design/rust-to-hako-lifecycle-emitter-contract.md
```

## Scope

Only the `BindingContext` lifecycle shape:

```text
Rust BTreeMap<String, BindingId>:
  deterministic_order_required fact
  Hako plan: OrderedMapBox

Rust &self read methods:
  BorrowFact SharedRead
  plan: direct read or BorrowView

Rust &mut self mutation methods:
  BorrowFact UniqueWrite, escapes=false
  plan: owner method mutation

Rust BindingId Copy:
  CopyKind ImmediateValue / AggregateValue as appropriate

Rust BTreeMap memory-only Drop:
  DropFact TrivialMemory
  cleanup_policy=erase
```

Implementation may be docs/schema/probe first. If code is needed, it must stay
BindingContext-specific and must not become a general resolver.

## Acceptance

```text
binding_context_lifecycle_facts_fixture=1
binding_context_lifecycle_plan_fixture=1
ordered_map_projection_requires_deterministic_order_fact=1
memory_drop_erased_only_with_TrivialMemory=1
borrow_escape_unknown_denied=1
rust_lifetime_syntax_added=0
general_resolver_implemented=0
converter_direct_ownership_policy_added=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_generalize_to_VariableContext=1
do_not_implement_full_lifecycle_resolver=1
do_not_emit_verified_plan_to_Hako_yet=1
do_not_add_Rust_lifetime_syntax=1
do_not_choose_OrderedMapBox_from_BTreeMap_spelling_alone=1
do_not_erase_Drop_without_TrivialMemory=1
```

## Next

```text
296x-1388-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001
```
