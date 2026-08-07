# LOOP-CALLER-ZERO-PARITY-G0-I0-R0

Status: `Implementation open; prerequisite API correction first`
Date: `2026-08-08`
Parent: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-design-2026-08-08.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Sole claim

Issue one exact compiler-side G0 prepared ingress that pairs the existing
resolver input capability with the neutral S4 Recipe product and proves the
complete fifteen-row common demand before any Builder/MIR effect.

```text
exact ResolvedFunctionLoweringInputV1<'source>
 + existing resolver ledger/source receipt
 + neutral VerifiedGenericRecipeProductG0
 + exact G0 entry-input and After capability
       -> VerifiedGenericG0SourceIngressV1<'source>
       -> PreparedGenericG0LoopPhysicalizationV1<'source>
       -> VerifiedLoopOperationPhysicalDemandV1 / prepare_all
```

The ingress is a thin move-only transport/compatibility product. It does not
become a new semantic owner and does not add compiler fields to the neutral
S4 product.

## Required implementation

1. First split the G0 After product at its consuming boundary. Add a
   `VerifiedGenericG0TailCapabilityV1` carrying post-loop read, exact I64 ABI,
   owner, and frame, and a consuming `into_physical_parts()` (or equivalent)
   that returns it separately from the neutral
   `VerifiedLoopContinuationContractV1`. Do not use the current
   `into_operation_demand_parts()` for physical G0 work because it drops this
   profile-specific tail information.
2. Add the smallest compiler-side ingress module beside the existing Generic
   source projection/hand-off code. Keep it out of
   `loop_structural_facts` and `loop_recipe_contract`; those remain AST-free
   semantic authorities.
3. Accept exact resolver-issued input and the already-issued S4 product from
   the same source request. Do not call a second resolver, clone AST, navigate
   by name/path, or rebuild S4 facts.
4. Validate before Builder effect:

   ```text
   owner / origin / source kind
   root Loop source site and execution frame
   Scope/Region relation
   resolver forest identity and root/child membership
   two exact G0 entry bindings (v0/v1)
   operation/effect owner and placement relations
   G0 After L0.After/b1 and post-loop BindingRef
   NumericTarget / exact I64 ABI compatibility
   ```

5. Consume the ingress exactly once to issue the existing common
   `VerifiedLoopOperationPhysicalDemandV1`, then call `prepare_all` for the
   complete G0 schedule. No single-row extraction API is allowed.
6. Keep the G0 After/Tail capability distinct from Callable `value`; do not
   produce physical blocks, ValueIds, Completion, DraftSeal, or module output.
7. Add focused positive and negative tests for missing, duplicate, foreign,
   stale, wrong-frame, wrong-scope, wrong-entry, wrong-After, wrong-owner, and
   target/ABI mismatch. Every reject must occur before Builder effect and be
   typed `NoSafeSlice` or an existing exact ingress rejection.

## Acceptance

- one positive exact G0 ingress reaches full fifteen-row `prepare_all`;
- G0 After is split once into neutral continuation and a distinct
  `VerifiedGenericG0TailCapabilityV1`; post-loop read/ABI/owner/frame are not
  dropped or relabeled;
- item keys follow Recipe structure, not evidence-vector order or profile
  labels; item 3 remains `DerivedCarrierEntry(C2)` and item 4 remains nested
  Loop structure rather than a flattened operation;
- Callable seven-row and G0 fifteen-row products are never compared by count
  or order;
- foreign and stale same-shape inputs are rejected before Builder effects;
- the ingress has no AST, name lookup, resolver, fallback, retry, selector,
  physical ID, CFG/SSA/PHI, Completion, or module-publication authority;
- all touched source/check files remain below 800 lines;
- focused tests, `cargo check --lib`, rustfmt, current-state guard,
  replacement guard, and diff check are green;
- implementation closeout updates the physical-demand/session SSOT, Generic
  source-to-portable Recipe SSOT, JoinIR pipeline SSOT, MIR references,
  affected README, `CURRENT_STATE.toml`, `10-Now.md`, and workstream in the
  same commit.

## Explicit non-claims

This row does not open common physical emission, recursive root/child CFG,
operation MIR, G0 After-to-tail physical read, Completion, DraftSeal,
production caller/selector, module collector, retry/fallback, M8/M9 parity,
or legacy deletion. If exact resolver input/entry capability cannot be paired
without reconstruction, stop with `NoSafeSlice` and update this task/SSOT
instead of adding a workaround.
