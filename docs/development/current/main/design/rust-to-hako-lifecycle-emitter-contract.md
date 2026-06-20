# Rust-to-Hako Lifecycle Emitter Contract

Status: SSOT
Scope: Converter/emitter contract for verified lifecycle plans.

## Purpose

The converter/emitter is the final rendering surface for verified
`HakoLifecyclePlan-v0`.

It does not choose ownership policy.

```text
input:
  verified HakoLifecyclePlan-v0

output:
  .hako source/skeleton
  or canonical MIR surface
```

## Required Inputs

```text
RustLifecycleFacts-v0:
  source lifecycle evidence

HakoLifecyclePlan-v0:
  resolver-selected plan

VerifierResult:
  positive verification that the plan satisfies the facts
```

Missing verification is a fail-fast error.

## Allowed Rendering

```text
Immediate:
  render scalar/local value use

AggregateLocal:
  render record/local aggregate shape

BorrowView:
  render verified non-owning access surface

TransferOwned:
  render verified move/consume/replace surface

LocalBox:
  render box construction and local use surface

OrderedMapBox:
  render OrderedMapBox construction/use only when selected by plan

HostResource:
  render verified cleanup / release owner surface

ArcCompat / CompatShim:
  render explicit compatibility boundary and diagnostics
```

## Forbidden Policy

```text
do not choose record vs box from Rust syntax
do not choose OrderedMapBox from BTreeMap spelling
do not erase Drop from absence of evidence
do not lower &mut to mutation without verified non-escape plan
do not lower returned/stored borrow as naked alias
do not lower Arc/Rc as ordinary box when Arc behavior is observed
do not invent LocalBox / CompatShim fallback for unknown facts
do not duplicate resolver decisions in emitter
```

## Fail-Fast Rules

```text
missing_plan:
  fail-fast

unverified_plan:
  fail-fast

unknown_fact_dependency:
  fail-fast

plan_kind_not_supported_by_emitter:
  fail-fast with plan kind and source context
```

## Separation From Current Skeleton Converter

The current RustSubset skeleton converter can continue to emit lossy skeletons
from `RustSubsetModule-v0`.

Lifecycle-aware emission is a later route and requires:

```text
RustLifecycleFacts-v0
HakoLifecyclePlan-v0
VerifierResult
```

Until then, the skeleton converter must not pretend to preserve ownership,
borrow, or Drop semantics.

## Stop Lines

```text
do not implement emitter behavior in this contract row
do not add Rust lifetime syntax
do not start BindingContext lifecycle pilot from emitter contract
do not allow fallback ownership policy
do not make diagnostics the source of lifecycle truth
```
