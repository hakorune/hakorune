# MIR PHI Invariants

Note
- Phase‑15 では PHI‑on が既定だよ。この資料の不変条件は MIR ビルダーが生成する PHI と、レガシーで `NYASH_MIR_NO_PHI=1` を指定したときに LLVM が補完するケースの両方へ適用するよ。詳しくは `phi_policy.md` を参照してね。

Scope: Builder/Bridge, PyVM, llvmlite (AOT)

Goal: Ensure deterministic PHI formation at control-flow merges so that
PyVM and LLVM backends agree for nested short-circuit, loop-if merges,
and chained ternary expressions.

Invariants
- If-merge ordering: Record incoming as [then, else] in this order when
  both branches reach the merge. When a branch is structurally absent,
  synthesize a carry-over from the pre-merge value.
- Loop latch snapshot: The latch (backedge) snapshot must be taken after
  per-iteration merges (i.e., after any phi binding for variables assigned
  in the loop body or nested if). Builder must bind the merged value to the
  loop-carried variable map before capturing the end-of-body state.
- Self-carry handling: A PHI with self-carry is allowed only when there is
  at least one non-self incoming. At finalize, map self-carry to the most
  recent non-self source visible at the predecessor end.
- Resolved one-level nested If: the inner merge block may be the outer
  `then` predecessor. Each PHI input set must equal the actual CFG predecessor
  set, and the nested profile emits exactly one PHI per merge for its shared
  binding. This is a profile-scoped invariant, not a claim for arbitrary
  recursive/effectful If lowering.

Representative Cases
- Nested short-circuit: `a && (b || c)` with selective assignments in nested
  branches. Expect single-eval per operand and deterministic merge order.
- Loop + if merge: A running sum updated in only one branch inside a while
  loop. Expect the latch to capture the phi-merged value, not a pre-merge
  temporary.
- Chained ternary: `cond1 ? (cond2 ? x : y) : z`. Expect linearized branches
  with merge ordering preserved at each join.

Diagnostics
- Enable `NYASH_LLVM_TRACE_PHI=1` to record per-block snapshots and PHI
  wiring in the LLVM path.
- Bridge verifier may allow `verify_allow_no_phi()` in PHI-off mode, but
  the invariants above still apply to resolver synthesis order.

Caller-zero Loop observer boundary
- `LoopPhiMaterializerV1` is mechanical parity evidence for a verified Loop
  JoinSig plus a sealed logical-to-physical map; it is not a production PHI or
  SSA owner.
- PHI mutation uses one `PhiTxn` provisional/patch/commit/abort lifecycle.
  Missing paths, predecessor mismatches, unknown types, and duplicate
  destinations fail before or within that transaction.
- Canonical CFG, function-owned Binding SSA, and `PhiTxn` remain the sole
  production owners. M6-B and the structural P1b edge-path witness introduce
  no grammar, IR, route, Retry, or publication behavior.

Resolved DirectAccum final-carrier boundary (M10a D2-S5/P4-S1)
- The resolved caller seals the `After` block from terminator-derived
  predecessors before reading final carrier bindings.
- Carrier keys 0/1 are read through the same function-owned Binding-SSA
  adapter and handed to a typed `DirectAccumFinalBindingReceiptV1`; no second
  map or PHI writer is allowed.
- A sealed `After` block with one predecessor forwards directly and must not
  gain synthetic final PHIs. The P4-S1 observer compares the resulting MIR
  semantically and never fabricates final rows from header PHIs.
- The generic open-After continuation receipt is unchanged. This is a
  singleton resolved profile, not an all-route or IR-wide loop rule.
