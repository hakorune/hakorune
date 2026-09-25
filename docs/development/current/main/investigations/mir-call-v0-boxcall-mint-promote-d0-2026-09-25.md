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

- [ ] All four evidence items answered with file:line citations.
- [ ] One accepted Decision; next row named (S-promote or explicit
  frontier pause).
