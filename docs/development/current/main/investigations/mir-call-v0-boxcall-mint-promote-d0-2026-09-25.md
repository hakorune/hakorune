# MIR-CALL-V0-BOXCALL-MINT-PROMOTE-D0 — boxcall carrier promote feasibility

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-V0-BOXCALL-DEPRECATION-D0 (closed NoSafeSlice —
  wire retirement is product-level; carrier promote evidence owed)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Question

`src/runner/mir_json_v0/module.rs:439-475` is the sole production
`LegacyCallV0` minter (v0 `boxcall` → `LegacyCallV0{func: INVALID,
callee: Some(Callee::Method{receiver: Some})}`). Wire-level retirement
is product-level (deprecation D0, closed NoSafeSlice). This row
decides whether a **carrier-only** promote — minting
`MirInstruction::call(dst, Callee::Method{..}, args, READ)` while
keeping the `boxcall` wire unchanged — is a bounded slice that zeroes
the last minter and reopens R7 caller-zero.

Design_stop census only — no implementation.

## Open evidence (all must resolve before any promote)

1. **Canonicalize coupling.** `callsite_canonicalize/pass.rs` reads
   `MirInstruction::LegacyCallV0{callee: Some(Callee::Method)}`
   exclusively and rewrites receiver-known user boxes to
   `Known`/`UserDefined` typed calls. Options: (a) keep the pass
   legacy-only and prove boxcall receivers never resolve to user
   boxes via v0 `value_types`; (b) extend the pass to typed
   `Call{Method}` — requires census that no production emitter
   produces `Call{Method{Union|RuntimeDataBox}}` rows whose receiver
   resolves to a user box (currently: `effect_emission.rs`,
   `exprs_qmark.rs`, `rewrite/special.rs`, `boxcall_emit.rs`,
   `resolver.rs` all emit `TypeCertainty::Union` — classify which of
   those produce `Call` vs `LegacyCallV0` and whether their receivers
   can be user boxes).
2. **rc=3 equivalence.** phase14/17 pins require `--mir-json-file`
   re-entry of a boxcall module to exit rc=3. Confirm the
   `[vm-reference/canonical-call]` stop maps to the same exit code as
   `[vm-reference/legacy-call/method-stopped]` — or record the tag
   change as an accepted residual if only the code is pinned.
3. **Selected-Dynamic terminal.** `Call{Method}` rows bypass the
   `call-legacy-carrier` scan in `src/runner/product/llvm/mod.rs`;
   name the terminal that admits/rejects them and pin it.
4. **Remaining LegacyCallV0{Method} readers.** Enumerate readers that
   match `LegacyCallV0`+`Method` specifically (not generic callee
   arms) and confirm each either tolerates `Call{Method}` or is a
   named stop — `is_supported_*` allowlists, published view, verifier
   arms, `mir_json_emit` compat egress.

## Decision options

1. Bounded — mint promote + (only if evidence requires) bounded
   canonicalize read-side extension; named S-row follows.
2. `NoSafeSlice` — record which evidence item fails and the reopen
   trigger; the Call/R7 lane then pauses fully until the product
   wire decision lands.

## Exit

- [x] All four evidence items answered with file:line citations.
- [x] One accepted Decision; next row named (S-promote or explicit
  frontier pause).

## Evidence resolution (2026-09-25)

1. **Canonicalize coupling — empty for v0 input.**
   `canonicalize_callsite_instruction`'s Method arm needs
   `known_user_box_name_from_value(value_types, .., receiver)`
   (`pass.rs`/`helpers.rs`), but `src/runner/mir_json_v0/module.rs`
   never populates `func.metadata.value_types` (zero references) and
   v0 modules carry no box declarations for `collect_known_user_boxes`
   to bind. The arm returns 0 on every v0 boxcall row today, so
   promoting the mint changes nothing for this input and the pass
   needs no typed-Method arm.
2. **rc=3 — already unreachable on the current path.** The
   `--mir-json-file` chain (`runner/mod.rs:166` → `core_executor`
   → `execute_mir_module_quiet_exit`) maps `VMError` to rc=1
   (vm-reference) or 2 without the feature; the phase14/17 rc=3 pins
   are non-live contract pins already blocked upstream
   (`static-call/legacy-fallback-retired` baseline). Carrier promote
   is rc-neutral: both carriers hit a `VMError` stop; only the tag
   string changes
   (`legacy-call/method-stopped` → `canonical-call`).
3. **Selected-Dynamic — typed Method is an admitted production
   shape.** `boxcall_emit.rs:225` emits `method_call(...)` =
   `MirInstruction::Call{Callee::Method}` in production, so every
   selected-Dynamic reader already handles typed Method rows; the
   `call-legacy-carrier` scan intentionally only covers
   `LegacyCallV0`. Promoted boxcall rows take the same terminals as
   production Method rows.
4. **Remaining readers — carrier-agnostic.** `backend_core_ops`
   allowlists map `Call` and `LegacyCallV0` to the identical op set
   (`mir_call`,`call`,`boxcall`,`externcall`); v0/v1 egress is
   callee-driven (`emit_call` shared seam); the published view's
   `Some(_)` arm treats Method identically; the interpreter stops
   both with named tags.

## Decision (accepted 2026-09-25): bounded — carrier-only promote

`module.rs` boxcall mints `MirInstruction::call(dst,
Callee::Method{box_name, method, receiver: Some(box_id),
certainty: Union, box_kind: RuntimeData}, args, EffectMask::READ)`
instead of `LegacyCallV0{func: INVALID, ..}`. Wire contract unchanged
(`boxcall` stays parsed and emitted); the last production
`LegacyCallV0` minter retires and R7 caller-zero becomes re-openable.

- Named next row: `MIR-CALL-V0-BOXCALL-MINT-PROMOTE-S5`.
- Recorded residual: interpreter stop tag changes to
  `[vm-reference/canonical-call] only Global targets are admitted`
  (same named-stop class); `canonicalize` LegacyCallV0{Method} arm
  becomes fully dead production-side (R7 deletion candidate, not this
  slice).
