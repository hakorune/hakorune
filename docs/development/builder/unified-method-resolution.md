# Unified Method Resolution — Design Note (Phase P4)

Purpose
- Document the unified pipeline for method resolution and how we will roll it out safely.
- Make behavior observable (dev-only) and gate any future default changes behind clear criteria.

Goals
- Single entry for all method calls via `emit_unified_call`.
- Behavior-preserving by default: Unknown/core/user‑instance receivers route to BoxCall.
- Known receivers may be rewritten to function calls (obj.m → Class.m(me,…)) under strict conditions.
- Keep invariants around SSA and instruction order to prevent sporadic undefined uses.

Pipeline (concept)
1) Entry: `emit_unified_call(dst, CallTarget::Method { box_type, method, receiver }, args)`
2) Explicit early special route: toString/stringify/str keeps its existing terminal.
3) Canonical method route: the incoming typed `Method(Some(receiver))` is
   preserved; the optional Known/Unique and equals rewrites are retired.
4) Routing: `RouterPolicy.choose_route` decides Unified vs BoxCall.
5) Emit guard: LocalSSA finalize (recv/args in current block) + BlockSchedule order contract (PHI → Copy → Call).
6) MIR emit: `Call { callee=Method/Extern/Global }` or `BoxCall` as routed.

Invariants (dev-verified)
- SSA locality: All operands are materialized within the current basic block before use.
- Order: PHI group at block head, then materialize Copies, then body (Calls). Verified with `NYASH_BLOCK_SCHEDULE_VERIFY=1`.
- The retired Known/Unique rewrite is not a target authority; canonical method
  lowering does not reconstruct a target from names, headers, or suffixes.

Behavior flags (existing)
- `NYASH_ROUTER_TRACE=1`: short route decisions to stderr (reason, class, method, arity, certainty).
- `NYASH_LOCAL_SSA_TRACE=1`: LocalSSA ensure/finalize traces (recv/arg/cond/cmp).
- `NYASH_BLOCK_SCHEDULE_VERIFY=1`: warn when Copy/Call ordering does not follow the contract.
- KPI (dev-only):
  - `NYASH_DEBUG_KPI_KNOWN=1` → aggregate Known rate for `resolve.choose`.
  - `NYASH_DEBUG_SAMPLE_EVERY=N` → sample output every N events.

Retired selector (policy)
- `NYASH_REWRITE_KNOWN_DEFAULT`, `NYASH_BUILDER_REWRITE_INSTANCE`,
  `NYASH_DEV_REWRITE_USERBOX`, and `NYASH_DEV_REWRITE_NEW_ORIGIN` are retired.
  Canonical lowering does not read them and does not recreate a Global target
  from generated names or header/suffix candidates.

Rollout note
- Existing callers should remove these selectors. `NYASH_ROUTER_TRACE=1`
  remains an observation-only route trace.

Key files
- Entry & routing: `src/mir/builder/builder_calls.rs`, `src/mir/builder/router/policy.rs`
- Explicit special route: `src/mir/builder/rewrite/special.rs`
- SSA & order: `src/mir/builder/ssa/local.rs`, `src/mir/builder/schedule/block.rs`, `src/mir/builder/emit_guard/`
- Observability: `src/mir/builder/observe/resolve.rs`

Acceptance for P4
- quick/integration stay green with flags OFF.
- With flags ON (dev), green remains; KPI reports sensible Known rates without mismatches.
- No noisy logs in default runs; all diagnostics behind flags.

Notes
- This design keeps Unknown/core/user‑instance routing rules unchanged.
- The optional Known/Unique/equals rewrite was a non-authoritative
  optimization and is retired; exact typed method targets remain the sole
  ordinary method route.
