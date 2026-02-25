Status: Historical

Instruction Deduplication — 2025‑11 Sweep (BinOp / Loop / Control Flow)

Purpose
- Remove duplicated lowering/handling of core instructions across Builder (AST→MIR), Program(JSON v0) Bridge, MIR loaders/emitters, and backends.
- Establish single‑source helpers (SSOT) per instruction family with narrow, testable APIs.

Scope (first pass)
- BinOp (Add/Sub/Mul/Div/Mod/BitOps)
- Loop (Continue/Break semantics: latch increment and PHI sealing)
- Control Flow (Compare/Branch/Jump/Phi placement and sealing)
- Const/Copy (uniform emission; string/int/float/bool coercions)

Hotspots (duplicated responsibility)
- BinOp
  - Builder: src/mir/builder/ops.rs
  - Program v0 bridge: src/runner/json_v0_bridge/lowering/expr.rs
  - Loader/Emitter/Printer: src/runner/mir_json_v0.rs, src/runner/mir_json_emit.rs, src/mir/printer_helpers.rs
  - LLVM Lower: src/backend/llvm/compiler/codegen/instructions/arith_ops.rs
- Loop (Continue/Break/PHI)
  - Program v0 bridge: src/runner/json_v0_bridge/lowering/loop_.rs, lowering.rs (snapshot stacks)
  - MIR phi core: src/mir/phi_core/loop_phi.rs, src/mir/loop_api.rs
- Control Flow
  - Compare/Branch/Jump/Phi scattered in: json_v0_bridge/*, mir/builder/emission/*, mir/builder/if_form.rs, runner/mir_json_v0.rs

SSOT Helpers — Proposal
- mir/ssot/binop_lower.rs
  - parse_binop(op_str, lhs, rhs) -> (BinaryOp, ValueId, ValueId)
  - emit_binop(builder_or_func, dst, op, lhs, rhs)
- mir/ssot/loop_common.rs
  - detect_increment_hint(stmts) -> Option<(name, step)>
  - apply_increment_before_continue(func, cur_bb, vars, hint)
  - seal_loop_phis(adapter, cond_bb, latch_bb, continue_snaps)
- mir/ssot/cf_common.rs
  - emit_compare/branch/jump helpers; insert_phi_at_head

Adoption Plan (phased)
1) Extract helpers with current logic (no behavior change). Add unit tests per helper.
2) Replace callers (Builder & Program v0 bridge first). Keep backends untouched.
3) Promote helpers to crate::mir::ssot::* public modules; update MIR JSON loader/emitter callsites.
4) Enforce via clippy/doc: prefer helpers over ad‑hoc code.

Verification & Canaries
- BinOp: existing Core quick profile covers arithmetic/bitops; add two v0 Program cases for mixed ints.
- Loop: continue/break matrix (then/else variants, swapped equals, '!=' else) — already added under phase2039; keep green.
- Control Flow: phi placement/sealing stays under phi_core tests.

Acceptance (first pass)
- BinOp lowering in Builder and Program v0 bridge uses ssot/binop_lower exclusively.
- Continue semantics unified: apply_increment_before_continue used in both bridge and (if applicable) builder.
- No regressions in quick core profile; all new canaries PASS.
