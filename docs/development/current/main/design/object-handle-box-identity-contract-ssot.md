---
Status: SSOT
Decision: accepted
Date: 2026-06-14
Scope: ObjectHandle / BoxIdentity contract for ARC-RETIRE-003.
Related:
  - docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
  - src/runtime/object_identity.rs
---

# ObjectHandle / BoxIdentity Contract

## Decision

`ObjectHandle` is the future ownership-substrate object token.
`BoxIdentity` is the generation-aware identity of that token.

```text
object_handle_contract_defined=1
object_handle_zero_is_invalid=1
box_identity_generation_defined=1
weak_object_handle_generation_check_required=1
object_identity_root_visibility_defined=1
object_identity_fini_owner_defined=1
typeabi_identity_truth_count=0
arc_hot_path_retirement_started=0
summary=accepted
```

This contract is representation-only in the current slice. It does not replace
`Arc<dyn NyashBox>` yet.

## Layering

```text
Current storage:
  Arc<dyn NyashBox>

Current public host ABI:
  u64 host handle, where 0 means null / void

New contract:
  ObjectHandle(raw_nonzero_u64)
  BoxIdentity(ObjectHandle, ObjectGeneration)
  WeakObjectHandle(BoxIdentity)

Future substrate:
  object table / arena / side table
```

## Invariants

```text
ObjectHandle:
  opaque
  non-zero
  stable only while the owning table entry is live
  no caller may infer table index or object kind from raw bits

ObjectGeneration:
  0 = legacy unversioned handle
  >=1 = versioned object-table generation

BoxIdentity:
  handle + generation
  detects reused slots when generation is versioned

WeakObjectHandle:
  stores BoxIdentity
  upgrade must validate handle and generation
```

## Ownership Fields

The contract separates identity from ownership policy.

```text
ObjectIdentityKind:
  Builtin
  UserBox
  Plugin
  HostBridge
  Unknown

RootVisibility:
  StrongRoot
  WeakOnly
  Borrowed
  Unrooted

FiniOwner:
  None
  Scope
  ObjectDrop
  Plugin { type_id, instance_id, fini_method_id }
  Host
```

## Plugin Mapping

Plugin identity is not the same as an internal Box slot.

```text
PluginInstanceIdentity:
  type_id
  instance_id
  fini_method_id

Plugin route truth:
  PluginLoader route resolver
  BoxCallableRegistry route snapshot

Plugin object identity:
  ObjectHandle / BoxIdentity contract
```

Do not mix plugin `method_id`, plugin `instance_id`, internal method slot, and
object handle raw values.

## Type ABI Boundary

TypeAbiCatalog can project descriptors for tools and planning. It must not
become object identity truth.

```text
typeabi_identity_truth_count=0
typeabi_hot_lookup_count=0
box_callable_dispatch_truth_count>0
```

## Current Implementation

The current implementation is a small Rust contract module:

```text
src/runtime/object_identity.rs
```

It defines:

```text
ObjectHandle
ObjectGeneration
BoxIdentity
WeakObjectHandle
ObjectIdentityKind
RootVisibility
FiniOwner
BuiltinIdentity
PluginInstanceIdentity
ObjectIdentityDescriptor
object_identity_contract_report_fields()
```

## Non-Goals

```text
do not replace Arc in this slice
do not rewrite Box trait surface
do not change host handle ABI
do not change plugin ABI
do not move identity truth into TypeAbiCatalog
do not add a cycle collector prerequisite
do not change active exact-AOT optimization lane
```

## Next Seam

The first implementation seam after this contract is `ARC-RETIRE-004`:
host handle table internals.

```text
external_host_abi_changed=0
borrowed_access_preserved=1
object_handle_contract_used_by_host_handles=1
host_handle_identity_generation=legacy_unversioned
identity_snapshot_available=1
host_handle_backing_arc_replaced=0
```
