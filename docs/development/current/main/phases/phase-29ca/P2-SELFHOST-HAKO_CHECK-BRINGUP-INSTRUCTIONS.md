---
Status: Complete
Scope: Selfhost tooling bringup (hako_check) on CorePlan/Composer pipeline
---

# Phase 29ca P2: Selfhost `hako_check` bringup instructions

## Goal

Make sure the selfhost toolchain can be exercised end-to-end without tripping JoinIR/CorePlan loop restrictions, and keep the verification commands stable and repeatable.

## Non-goals

- Do not relax CorePlan contracts “just to pass tools”.
- Do not add new env vars.
- Do not introduce silent fallbacks (strict/dev should fail-fast with observable tags when the tool hits an unsupported shape).

## Commands (SSOT)

- Build: `cargo build --release`
- Smoke (deadcode): `./tools/hako_check/deadcode_smoke.sh`
- Smoke (deadblocks): `./tools/hako_check_deadblocks_smoke.sh`
- Rule fixtures (JSON-LSP): `bash tools/hako_check/run_tests.sh`

Optional diagnostics:
- VM trace: `NYASH_VM_TRACE_LOG=__mir__.log ./tools/hako_check/deadcode_smoke.sh`

Notes:
- deadblocks smoke currently warns about CFG integration and may report 0 passes without failing the script.

## Runtime policy (tests / tooling)

- Analyzer runs are plugin-free by default:
  - `NYASH_DISABLE_PLUGINS=1`
  - `NYASH_BOX_FACTORY_POLICY=builtin_first`
  - `NYASH_JSON_ONLY=1`
- Prefer `--source-file <path> <text>` to avoid FileBox/plugin dependency.

## Loop policy for selfhost tooling sources

Selfhost tools should avoid “deep recursion as a loop substitute” because the Rust VM call-depth limit can cause non-local failures.

Preferred:
- Restricted structured loops that the current CorePlan/Composer accepts (generic loop v0 subset).

Avoid:
- `continue` (unless/until generic loop explicitly supports it)
- nested loops (unless/until explicitly supported subset exists)
- “if-effect” inside loops unless it is within the allowed CorePlan vocabulary for loop bodies

## When a tool loop fails

Triage order:
1. Determine whether the loop is a gate-candidate (part of selfhost regression set). If yes, strict/dev should surface `flowbox/freeze` early.
2. If the loop is structurally representable by current CorePlan primitives, prefer adjusting the tool code to the restricted subset (no recursion-to-loop conversion).
3. If repeated tool rewrites indicate missing composition primitives, propose a minimal CorePlan vocabulary addition with:
   - SSOT update
   - verifier contract
   - lowering
   - fixture+smoke gate
