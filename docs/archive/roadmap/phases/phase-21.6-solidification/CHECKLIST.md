Phase 21.6 — Solidification Checklist

Acceptance (all must be green on this host)
- Parser(Stage‑B): loop JSON canary PASS (no empty body; cond = Compare <, lhs=Var i, rhs=Var n|Int const)
- MirBuilder(delegate): MIR(JSON) for minimal loop contains only control‑flow ops (const/phi/compare/branch/binop/jump/ret)
- VM: MIR(JSON) for minimal loop returns 10
- ny‑llvmc(crate) EXE: returns 10

Canaries to run
- bash tools/dev/stageb_loop_json_canary.sh
- bash tools/archive/legacy-selfhost/engineering/phase216_chain_canary.sh
- bash tools/archive/legacy-selfhost/engineering/phase216_chain_canary_return.sh
- bash tools/archive/legacy-selfhost/engineering/phase216_chain_canary_binop.sh
- bash tools/dev/phase216_chain_canary_loop.sh
- bash tools/archive/legacy-selfhost/engineering/phase216_chain_canary_call.sh (Phase 21.6 extension)

Guardrails
- No default behavior changes; all aids behind env toggles.
- Logs quiet; tags/dev traces are opt‑in.
- No llvmlite in default chain; crate backend is main line.

Rollback
- Parser fallback in parse_loop is conservative; remove after VM/gpos fix lands.
- Keep canaries; they protect against regressions.

## Phase 21.6 Call Support Extension

**Toggles**:
- `HAKO_STAGEB_FUNC_SCAN=1`: Enable function definition scanning in Stage-B
- `HAKO_MIR_BUILDER_FUNCS=1`: Enable function definition lowering in MirBuilder (unused - Rust delegate used)

**Implementation**:
1. Stage-B function scanner extracts `method <name>(params){ body }` definitions
2. Stage-B injects `defs` array into Program(JSON v0)
3. Rust delegate (src/runner/json_v0_bridge/) processes `defs` and generates MIR functions
4. Function names are qualified as `Box.method` (e.g., `Main.add`)

**Current Status**:
- ✅ Stage-B scanning implemented (compiler_stageb.hako)
- ✅ Rust delegate defs processing implemented (ast.rs, lowering.rs)
- ✅ MIR functions generated with correct signatures and bodies
- ⚠️  Call resolution incomplete: calls still use dynamic string lookup ("add") instead of static Main.add reference

**Next Steps for Call Completion**:
- Need to resolve Call("add") → Call(Main.add) at lowering time
- Implement call target resolution in json_v0_bridge/lowering.rs
- Update Call instruction to use function reference instead of string
- Alternative: use MirCall with Callee::Function for local calls

**撤去条件 (Removal Criteria)**:
- After Phase 22 introduces proper local function scoping with Callee typed calls
- Or when unified with broader namespace/using system Phase 15.5+
