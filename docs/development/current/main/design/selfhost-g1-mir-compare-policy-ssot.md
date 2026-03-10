Status: Active
Updated: 2026-03-11

# Selfhost G1 MIR Compare Policy SSOT

Purpose:
- Define what `tools/selfhost_identity_check.sh --mode full` is allowed to treat as equivalent during `phase-29ch`.
- Keep the selfhost bootstrap authority unchanged while unblocking the current G1 branch point.

Scope:
- Stage1 vs Stage2 identity comparison for MIR(JSON v0) only.
- Initial target: `compiler_stageb.hako` matches on Program(JSON v0) but diverges on raw MIR text at `StageBArgsBox.resolve_src/1` and then `StageBDriverBox.main/1`.

Decision:
- During `phase-29ch`, the G1 done gate now uses `semantic canonical match` for MIR(JSON v0).
- Raw MIR text equality remains the tightening target, but it is not the first unblock criterion for this phase.
- Program(JSON v0) comparison stays raw exact match.
- Route authority stays unchanged:
  - `build_stage1.sh`
  - `stage1_contract_exec_mode`
  - `stage1_cli_env.hako`
  - `MirBuilderBox.emit_from_program_json_v0(...)`
  - `ny_mir_builder.sh`

Why:
- The red shape was narrower than a semantics change.
- Stage1 and Stage2 already match on:
  - Program(JSON v0)
  - function names
  - block counts
  - `user_box_decls`
- The first observed MIR divergence is an alpha-equivalent-looking live-in/copy bundle reorder.
- `phase-29ch` is a bootstrap unification phase, not a generator-determinism completion phase.

Allowed canonicalization in G1 MIR compare:
- ValueId renumbering within a function.
- BasicBlockId renumbering within a function.
- PHI incoming pair order.
- Contiguous block-local copy/live-in materialization bundles when the canonicalized instruction multiset is unchanged.

Not allowed:
- Function count mismatch.
- Function name mismatch.
- Function order drift.
- `user_box_decls` mismatch.
- CFG shape mismatch:
  - block count
  - successor sets
  - entry block shape
  - terminator kind
- Non-copy instruction reorder.
- Opcode mismatch.
- Callee mismatch.
- Constant/type payload mismatch.
- Added/removed side-effecting instructions.

Implementation ownership:
1. `tools/selfhost/lib/identity_compare.sh`
   - SSOT entry for G1 comparison policy.
   - Must remain the only compare policy entry used by `tools/selfhost_identity_check.sh`.
2. `tools/selfhost/lib/mir_canonical_compare.py` (new helper if needed)
   - Preferred place for JSON parse + canonicalize + semantic diff.
   - Keep the shell thin; do not spread compare rules into multiple scripts.
3. `tools/selfhost_identity_check.sh`
   - Keep route/probe authority logic unchanged.
   - May report both:
     - semantic canonical compare result
     - raw exact diff result as diagnostics/tightening evidence

Sequencing:
1. Land the narrow canonical compare for the allowed noise only.
2. Turn `G1 full` green on semantic canonical match. Status: done on 2026-03-11.
3. Keep raw exact diff visible as follow-up evidence.
4. After `phase-29ch` is stable, decide whether to tighten generator ordering so raw MIR text also converges.

Non-goals:
- Do not widen authority routes.
- Do not reopen `phase-29cg`.
- Do not retire Program(JSON v0) here.
- Do not introduce `.hako`-side workaround branches just to satisfy G1 compare.
