# 137x Current Entry

This is the active entry for phase-137x. The long `README.md` keeps historical
ledger details; current implementation work should start here.

## Current Lane

- lane: `137x-H25 array text residence session contract`
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- current blocker token: `137x-H25 array text residence session contract`
- current benchmark state:
  - `C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40330160`
  - `ny_aot_cycles=12366672`
- active owner:
  - write-lock acquire/release guard mechanics inside the fused loopcarry helper
  - H24 IP evidence: acquire/release `lock cmpxchg` sites own the samples
- non-owners:
  - fallback/promotion: H23a observed `update_text_resident_hit=179999`
  - helper-local resident/fallback compaction: H23b regressed to `ny_aot_instr=45910743`
  - byte-edit/memmove body: H24 samples did not land there

## Active Contract

- MIR owns:
  - residence-session eligibility
  - loop/session lifetime
  - alias/publication boundary
  - covered route facts
- `.inc` owns:
  - metadata consumption
  - later begin/update/end emit shape only
  - no raw MIR shape rediscovery
- Rust runtime owns:
  - write guard mechanics
  - text storage/slot access
  - mutation execution
  - no legality/provenance inference

## H25a Landed

- Added metadata-only `array_text_residence_sessions`.
- The loopcarry benchmark exposes exactly one session route:
  - `scope=loop_backedge_single_body`
  - `proof=loopcarry_len_store_only`
  - `consumer_capability=slot_text_len_store_session`
  - `publication_boundary=none`
- Behavior is unchanged:
  - no lowering change
  - no runtime/session helper change
  - no `.inc` route change

## Next Slice

H25b should design and implement begin/update/end lowering against the session
metadata. Do not edit runtime first.

Required order:
1. Define the backend metadata consumption shape.
2. Add runtime-private session executor surface only as needed by that shape.
3. Keep MIR metadata as the only legality source.
4. Rerun exact timing and asm after any behavior change.

Reject immediately if the implementation requires:
- runtime deciding session legality from residence state
- `.inc` scanning raw MIR JSON for session shape
- benchmark-specific whole-loop helper
- session across publish/objectize/generic fallback/unknown side-effect boundary

## Validation Anchor

- `cargo check -q`
- `cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route -- --nocapture`
- `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h25_loopcarry.mir.json benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako`
- after behavior change:
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry '' 20`

## Temporary Counters

H23 split counters are evidence-only and may stay while H25 session work is
active:
- `update_text_resident_hit`
- `update_text_resident_miss`
- `update_text_fallback_hit`
- `update_text_fallback_miss`

Retire them after H25 either lands a session keeper or rejects the session
hypothesis. Do not add a new env var for them.

