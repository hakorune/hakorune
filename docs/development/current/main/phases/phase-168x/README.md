# Phase 168x: counter step_chain pure-first exact route refresh

- Status: Landed
- Purpose: repair the stale pure-first exact contract for `Counter.step_chain` after the direct MIR builder cut tightened the forwarding body from a two-copy chain to the current narrow one-copy alias shape.
- Scope:
  - direct contract truth for `Counter.step_chain/0`
  - pure-first seed matcher refresh for the same exact bench shape
  - exact build/asm verification on `kilo_micro_userbox_counter_step_chain`
- Non-goals:
  - no new user-box method parity widening
  - no generic walker/backend recipe widening
  - no helper-retry fallback absorb

## Decision Now

- treat this as a backend exact-route contract refresh, not a new semantic widening
- keep the direct canonical method shape from `phase-167x` as the authority
- refresh the stale `Counter.step_chain/0` forwarding expectation only as far as needed to match the current direct contract

## Restart Handoff

- parent lane:
  - `docs/development/current/main/phases/phase-163x/README.md`
- current snapshot:
  - `docs/development/current/main/10-Now.md`
- workstream map:
  - `docs/development/current/main/15-Workstream-Map.md`
- SSOT:
  - `docs/development/current/main/phases/phase-168x/168x-90-counter-step-chain-pure-first-refresh-ssot.md`
  - `docs/development/current/main/phases/phase-168x/168x-91-task-board.md`
- code owner seam:
  - `lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc`
  - `tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_chain_contract.sh`

## Current Cut

- `phase-167x` already repaired direct lowering determinism and receiver metadata sealing
- the stop-line was narrowed and closed:
  - direct JSON emits `Counter.step_chain/0` as `copy -> mir_call -> ret`
  - pure-first exact seed and direct contract now accept the current narrow forwarding body, while still tolerating the older two-copy alias form
- acceptance for this cut is green:
  - direct contract smoke passes on the refreshed shape
  - boundary owner-lane known-receiver smoke stays green
  - pure-first build/asm is green again on `kilo_micro_userbox_counter_step_chain`
  - latest exact reread: `c_instr=127244 / c_cycles=220847 / c_cache_miss=3880 / c_ms=3` vs `ny_aot_instr=466555 / ny_aot_cycles=812475 / ny_aot_cache_miss=8825 / ny_aot_ms=4`
  - current `ny_main` object snippet is `mov $0x2b, %eax ; ret`

## Stop Line

- do not loosen the matcher beyond the current direct exact route
- do not change `Counter.step/0` leaf semantics in this phase
- do not mix this backend exact-route refresh with broader user-box local-method parity work
