# 296x-1676: Boxed Sum VariantMake Site Fact Normalization

Status: Complete
Date: 2026-06-24
Token: BOXED-SUM-VARIANT-MAKE-SITE-FACT-NORMALIZATION-001

## Decision

Normalize boxed-sum `VariantMake` site facts before changing C shim behavior.

```text
selected option:
  D

semantic split:
  enum instantiation hint
  actual payload presence
  boxed runtime ABI plan
  runtime payload storage
```

This slice does not add canonical MIR instructions and does not introduce
Option-specific backend behavior.

## Problem

`MetadataContextApi.current_parent_region/1` currently fails AOT when
constructing `Option<i64>::None`:

```text
reason=boxed_sum_abi_missing_site_payload_storage
op=variant_make
variant=Option::None
payload_type=Integer
has_payload=false
boxed_sum_abi_plan_id=null
```

`payload_type` is a producer-side instantiation hint. It is not proof that the
selected variant has a runtime payload. For a unit variant, actual payload
presence must win.

## Authority

```text
boxed_sum_abi_plan_id
  = runtime representation identity

tag
  = plan variant row selector

plan.variants[tag].payload_storage
  = runtime storage authority

has_payload
  = actual operand presence / constructor arity authority

payload_type
  = instantiation hint only
```

`boxed_sum_payload_storage` may remain serialized as a cache/assertion, but it
must match `plan.variants[tag].payload_storage`; it is not a second authority.

## Selected Scope

```text
BoxedSumVariantMakeSiteFacts:
  enum_name
  tag
  payload_presence = Absent | Present
  instantiation_hint = Option<MirType>

selected source shape:
  unit variant
  payload = None
  concrete contextual instantiation hint
```

Required behavior:

```text
Option<i64>::None
  -> plan shape Option|0:none,1:i64
  -> storage for tag 0 is none

Option<i64>::Some(20)
  -> same plan id as Option<i64>::None
  -> storage for tag 1 is i64
```

## Acceptance

```text
metadata_context_region_parent AOT green
None and Some use the same Option<i64> abi_plan_id
Option<i64>::None keeps payload_type as hint if needed
has_payload=false -> plan[tag].payload_storage=none
has_payload=true -> plan[tag].payload_storage!=none
Some(payload=%0) is treated as payload-present
Option<i64>::None and Option<String>::None resolve to distinct plans
ambiguous generic unit site fails closed
variant_binding.plan_id == value_representation abi_plan_id
payload_type backend proof = 0
Option-name backend branch = 0
runtime fallback = 0
```

## Non-Claims

```text
general multi-parameter generic sum specialization = 0
full generic monomorphization = 0
new backend route = 0
new canonical MIR instruction = 0
```

## Parked Follow-Ups

```text
rename payload_type into actual_payload_type / sum_instantiation_hint
MIR JSON root boxed-sum plan rebuild consolidation
multi-parameter enum type-argument mapping
JSON schema tightening for boxed-sum site fields
```

## Closeout

Implemented in the Rust boxed-sum site resolver and C AOT consumer path.

```text
VariantMake site facts:
  payload_type remains an instantiation hint
  has_payload / payload presence drives constructor arity
  plan.variants[tag].payload_storage drives runtime storage
  resolved BoxedSumSitePlan is consumed by MIR JSON emission

C AOT:
  VariantMake reads explicit has_payload from variant_binding
  raw payload register truthiness is not used
  VariantTag can read boxed-sum ValueRepresentationFact from operands
  boxed handle + local unit binding compare lowers through tag comparison
  print lowering also consumes existing ORG_STRING metadata
```

Evidence:

```text
cargo test -q boxed_sum
cargo test -q variant_make
cargo test -q value_representation_fact
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/rust_lifecycle_metadata_context_region_parent_derived_artifact_guard.sh
git diff --check
```

Result:

```text
metadata_context_region_parent AOT green
None and Some use Option|0:none,1:i64 where required
Some(payload=%0) remains payload-present
Option-name backend branch = 0
payload_type backend proof = 0
runtime fallback = 0
```
