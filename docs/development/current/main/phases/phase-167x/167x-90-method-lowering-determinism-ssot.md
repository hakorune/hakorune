# 167x-90: user-box direct method sealing SSOT

Status: SSOT
Date: 2026-04-12
Scope: remove the current direct-route drift where user-box method lowering shape depends on missing instance-method receiver metadata and on duplicated finalize ownership inside the MIR builder.

## Goal

- keep instance methods on the same finalize/metdata owner as static lowered methods
- seed direct receiver `Box(...)` type facts for instance methods
- keep box member lowering deterministic in MIR builder
- keep the current known-receiver keeper shape stable for `Counter.step_chain`
- fix the root cause in builder traversal, not in backend/helper fallback

## Diagnosis

The observed flake is:

- the same `bench_kilo_micro_userbox_counter_step_chain.hako` direct route sometimes lowers `Counter.step_chain/0` as canonical known-receiver `Method`
- and sometimes lowers it as `Global("Counter.step/0")`

The initial suspicion was raw `HashMap` traversal order, but the direct keeper break was actually narrower:

- instance-method lowering had an inline Step 5 path that bypassed `finalize_function()`
- that path did not run the shared type propagation / metadata sealing path
- `setup_method_params()` tracked `me` origin but did not seed `MirType::Box(<box>)`
- `callsite_canonicalize` consumes `function.metadata.value_types`, so direct known-receiver rewrite could miss the receiver box fact and fall back to `Global("Counter.step/0")`
- deterministic member traversal is still worth keeping, but it is not the only root cause

## Authority

1. `.hako` owner / policy
2. MIR builder function sealing owner
3. current known-receiver rewrite consumer
4. backend / pure-first / asm keepers

This phase repairs step 2 so step 3 stops observing missing receiver metadata on direct lowered methods.

## Fix

### 1. Keep one deterministic traversal owner

`src/mir/builder/declaration_order.rs`

This owner provides sorted traversal for:

- methods
- constructors

### 2. Route every box-member lowering path through that owner

Consumers:

- `module_lifecycle.rs`
- `exprs.rs`
- `decls.rs`
- `declaration_indexer.rs`

### 3. Unify instance-method finalize ownership

- route `lower_method_as_function()` through shared `finalize_function()`
- keep type propagation, result type hints, and `metadata.value_types` sealing on one owner

### 4. Seed receiver type facts at the parameter boundary

- `setup_method_params()` records `me` as both origin and `MirType::Box(<box>)`
- instance method parameters also register `MirValueKind::Parameter(..)` the same way static lowered methods do

### 5. Add direct regression coverage

Keep two layers:

- unit: sorted traversal ignores `HashMap` insertion/random order
- integration: `Counter.step_chain/0` keeps receiver `Box(Counter)` metadata and always lowers through known-receiver `Method` instead of `Global("Counter.step/0")`

## Non-Goals

- do not widen backend matcher tolerance
- do not add a new generic declaration-presence owner in this phase
- do not mix this fix with `phase166x` generic relation work

## Acceptance

- MIR builder no longer iterates box member `HashMap`s directly on the live lowering paths
- instance methods seal through the same metadata owner as static lowered methods
- `Counter.step_chain/0` keeps receiver `Box(Counter)` metadata and stays on canonical known-receiver `Method` shape in regression coverage
- release direct emit stays stable in repeated probes
- pure-first/backend contract widening remains out of scope for this phase
