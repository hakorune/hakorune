# MIR-CALL-V0-BOXCALL-MINT-PROMOTE-S5 — boxcall minter promotes to typed Call

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-V0-BOXCALL-MINT-PROMOTE-D0 (accepted, bounded)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Slice

`src/runner/mir_json_v0/module.rs` boxcall arm (439-475): replace the
`MirInstruction::LegacyCallV0{func: ValueId::INVALID, callee:
Some(Callee::Method{..})}` mint with
`MirInstruction::call(dst_opt, Callee::Method{box_name, method,
receiver: Some(ValueId::new(box_id)), certainty: TypeCertainty::Union,
box_kind: CalleeBoxKind::RuntimeData}, args, EffectMask::READ)`.

Wire contract unchanged: `boxcall` stays a valid v0 input spelling and
the callee-driven compat emitter still writes `boxcall` for
`Callee::Method` (methodize=false).

### Named casualty / residuals

- Interpreter stop tag for a parsed boxcall row changes from
  `[vm-reference/legacy-call/method-stopped]` to
  `[vm-reference/canonical-call] only Global targets are admitted`
  (same VMError class; rc=1 either way on the quiet-exit path).
- `callsite_canonicalize`'s `LegacyCallV0{Method}` arm is dead on v0
  input already: the v0 loader never writes
  `function.metadata.value_types`, so the arm's `value_types` lookup
  returns early before it can rewrite a parsed boxcall row. The arm is
  an R7 deletion candidate, not this slice.
- Zero production ingress minters after this slice. The surviving
  expression-position mint is
  `mir::array_element_write::project_module_to_legacy_calls`, the
  documented ExplicitCompatibility projection owner for the
  feature-gated `llvmlite-compat` lane
  (`runner/modes/common_util/array_write_backend.rs` is its sole
  caller seam). Its retirement belongs to that lane, not R7 ingress.
- `LegacyCallV0` type, `func` slot, and reader arms stay for compat
  rows and test fixtures until R7 deletion evidence.

### Verification (landed)

- `cargo test --profile quick --lib boxcall_`: 76 passed; the 8 reds
  (`compile_v0_emits_mir_call_extern_*` x7,
  `test_lowering_boxcall_array_push`) are all in
  `cargo_lib_red_baseline.{tests,failures}.txt` — unrelated upstream
  admission freezes, not this change.
- New pins green:
  `runner::mir_json_v0::tests::boxcall_mints_canonical_method_call_carrier`,
  `runner::mir_json_v0::tests::boxcall_without_optional_fields_uses_runtime_data_defaults`,
  `handlers::calls::canonical_method_call_rejects_at_global_only_boundary`
  (vm-reference).
- Existing wire projection pins cover `Callee::Method` -> `boxcall`
  (`compatibility_profile_without_methodize_keeps_boxcall`,
  `method_none_keeps_legacy_receiver_func_until_r6`); emission is
  callee-driven so wire shape is unchanged.
- `mir_call_d1b_active_surface_guard` row `MIR-CALL-V0-BOXCALL-MINT-PROMOTE-S5`
  registered in r6 helpers.

## Exit

- [x] Zero production `LegacyCallV0` ingress minters (rg census:
      expression-position constructors outside `#[cfg(test)]` are only
      `array_element_write::project_module_to_legacy_calls`, the
      documented ExplicitCompatibility owner above).
- [x] Pins + focused suites green; new reds classified (all baseline).
- [x] Guard row registered and green; card/pointer/workstream synced.
