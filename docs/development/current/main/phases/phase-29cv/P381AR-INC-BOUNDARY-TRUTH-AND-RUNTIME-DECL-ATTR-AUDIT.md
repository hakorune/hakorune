---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv next keeper-bucket selection for `.inc` boundary ownership and the first runtime-decl attrs audit (`nyash.string.len_h`)
Related:
  - docs/development/current/main/design/llvm-line-ownership-and-boundary-ssot.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - lang/c-abi/shims/README.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_helpers.rs
---

# P381AR: `.inc` Boundary Truth And Runtime Decl Attr Audit

## Problem

After the P381O-AP closeout, the next open question is not another body-shape
patch. It is whether the remaining `.inc` / runtime-helper work is still moving
in the correct direction.

The boundary SSOT already says the daily LLVM line is:

```text
.hako -> MIR -> thin backend boundary -> ny-llvmc(boundary pure-first) -> object/exe
```

That means `.inc` is allowed as boundary glue, but it must stay thin:

- consume MIR-owned facts / LoweringPlan rows
- validate backend-local schema / ABI facts
- emit LLVM IR
- fail fast when the plan is unsupported

The risk is regrowth:

```text
.inc
  -> route/body classifier
  -> local value/dataflow rediscovery
  -> semantic clone of selfhost helper bodies
```

P207A and the Stage0 line-shape inventory already reject that growth, but the
current keeper bucket had not been stated explicitly after the P381 closeout.

At the same time, `runtime-decl-manifest-v0.toml` still leaves
`nyash.string.len_h` weaker than obvious read-only rows like
`nyash.array.slot_len_h`. Before tightening attrs, we need to prove whether the
kernel implementation is actually read-only / willreturn-safe.

## Decision

### 1. Next keeper bucket

The next explicit `phase-29cv` keeper bucket is:

```text
thin backend boundary truth
  = `.inc` stays plan-reader / emitter only
  + runtime-decl manifest stays truthful about helper side effects
```

This bucket sits on the same owner path as the current P381 work:

- `.hako` owns route policy
- MIR owns canonical contract / route proof
- thin backend boundary owns transport / plan validation / emission only
- runtime/kernel owns concrete runtime semantics

From this point, new `.inc` work should be rejected if it adds:

- source-helper body understanding
- receiver-family rediscovery
- raw local-dataflow re-analysis
- new body-specific C shim emitters as the default answer to
  `missing_multi_function_emitter`

Preferred fix order remains the Stage0 SSOT order:

1. source-owner cleanup
2. MIR-owned fact / LoweringPlan contract
3. generic MIR op support
4. uniform multi-function MIR emitter

### 2. `nyash.string.len_h` attrs audit result

Do **not** tighten the manifest entry today.

Keep:

```toml
symbol = "nyash.string.len_h"
attrs = ["nounwind"]
memory = "readwrite"
```

Reason:

- `string_len_export_impl()` can dispatch through a registered raw function
  pointer (`string_len_dispatch_probe_raw()` + transmute), so the active route
  is not a closed local pure helper.
- the implementation records route observations
  (`record_str_len_route_*()`), which is mutable side-effectful state.
- successful resolution stores into the fast cache
  (`string_len_fast_cache_store()`), which mutates thread-local cache cells.
- the slow path may `eprintln!()` when tracing is enabled.

Because of these effects, marking the helper as `readonly` / `memory = "read"`
or `willreturn` would overstate what the implementation guarantees today.

## Boundary

Allowed next:

- audit whether LoweringPlan already expresses lowering tiers cleanly
  (`HotInline` / `DirectAbi` / `ColdRuntime`)
- plan a uniform multi-function emitter gap closeout
- tighten runtime-decl attrs only when kernel behavior proves the row is
  actually read-only and willreturn-safe

Not allowed next:

- "fix" `missing_multi_function_emitter` by adding another body-specific `.inc`
  shape as the default
- move route policy or helper semantics back into `.inc`
- lie in runtime-decl attrs for performance

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- selected the next `phase-29cv` keeper bucket as thin backend boundary truth
- fixed the runtime-decl attrs reading for `nyash.string.len_h`: the row must
  stay `nounwind` + `readwrite` until dispatch/cache/observe/tracing side
  effects are removed or separately proven harmless
