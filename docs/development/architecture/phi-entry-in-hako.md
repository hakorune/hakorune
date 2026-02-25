Phi Entry in Hako — Design Notes (SSA/CFG Parity)

Purpose
- Specify how to implement SSA φ (phi) on the Hakorune side cleanly, mirroring Rust/Core invariants while keeping the code small and testable.

Rust/Core invariants to adopt (parity)
- Placement: φ nodes are considered at the head of a block (grouped), applied once at block entry.
- Selection: choose one incoming (value, pred) where pred == prev_bb (the block we arrived from).
- Coverage: incoming pairs cover all reachable predecessors. Missing entries are a hard error in strict mode.
- Execution: after φ application, the resulting dst registers are defined before any instruction in the block reads them.

Hako design (Reader → IR → Runner)
- Reader (JsonV1ReaderBox, extended):
  - Parse MIR JSON v1 into a minimal per-function IR: blocks (id, insts[]), and extract φ entries into a phi_table (block_id → [(dst, [(pred,val)])]).
  - Keep scanning light by using JsonFragBox helpers (read_int_from/after, seek_array_end, scan_string_end).
- PhiTable (V1PhiTableBox):
  - API: apply_at_entry(regs, phi_table, prev_bb, block_id, policy) → writes dst from the matched incoming.
  - policy.strict (default ON): fail-fast when incoming is missing or source is undefined; policy.tolerate_void (dev) treats missing/undefined as Void/0.
- Runner (NyVmDispatcherV1Box):
  - On block entry: apply φ via PhiTable; then run instructions (φ removed from the runtime loop).
  - Branch/jump update prev_bb and bb; compare/branch read the compare.dst as the condition value.

Flags
- HAKO_V1_PHI_STRICT=1 (default), HAKO_V1_PHI_TOLERATE_VOID=0 (dev-only safety).
- HAKO_V1_DISPATCHER_FLOW=1 to run the IR-based flow; keep fallback to Mini-VM and Core for stability during bring-up.

Testing plan
- Canary 1: simple if (then/else with single incoming) → ret of φ.dst equals the selected value.
- Canary 2: multi-incoming with (pred,val) pairs for both paths; ensure prev_bb select works for both branches.
- Canary 3: nested branch (entry φ in deeper block).
- Negative: missing incoming for reachable pred → strict fail; tolerate_void → rc stable with Void/0.

Why this works in Hako
- Although Hako doesn’t have first-class structs, the minimal IR and phi_table can be represented as arrays of tuples or MiniMap-backed strings with helper boxes.
- JsonFragBox provides escape-aware scanning; Reader avoids brittle substring logic.
- Runner remains small and composable: “read/apply/run” with φ isolated at entry.

Migration plan
- Phase 20.37: introduce Reader+PhiTable+entry-apply (flagged), keep fallback to Mini-VM/Core.
- Phase 20.38+: expand coverage (binop/compare edges), flip v1 verify default to Hako when parity canaries are green.

