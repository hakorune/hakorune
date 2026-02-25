# Phase 29ap P10: Pattern6_NestedLoopMinimal minimal CorePlan subset (strict/dev)

## Goal

- In strict/dev, adopt a minimal nested-loop CorePlan for the `phase1883_nested_minimal` shape.
- Keep release/default behavior unchanged (legacy JoinIR path remains).
- Fail-fast in strict/dev when nested loops are detected but the subset does not match.

## Subset (SSOT)

- Outer loop: `loop(i < <int>)` with step `i = i + 1` (step=1 only).
- Inner loop: `loop(j < <int>)` with body `sum = sum + 1; j = j + 1` (step=1 only).
- Inner init: `j = 0` (via `local j` + assignment, or `local j = 0`).
- No break/continue/return inside outer or inner loop.
- No value-join / exitmap / cleanup (presence must be empty).

## Steps

1. Facts (SSOT)
   - Add `Pattern6NestedMinimalFacts` and wire it into `LoopFacts` (optional field).
   - Keep `Ok(None)` for non-matches; no hardcode by-name.

2. Composer v2 (strict/dev)
   - Add `coreloop_v2_nested_minimal.rs` and compose a single CoreLoopPlan with a nested CFG.
   - Body remains effect-only (j-init only); inner loop is encoded via extra blocks + Frag wiring.

3. Strict/dev adopt
   - If nested facts are present, adopt and emit tag:
     - `[coreplan/shadow_adopt:pattern6_nested_minimal]`
   - If nested facts are missing, keep the strict/dev freeze (fail-fast).

4. Smoke gate
   - Update `phase29ap_pattern6_nested_strict_shadow_vm.sh` to require the shadow-adopt tag.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

