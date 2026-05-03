Phase 21.6 — Solidification (Hakorune‑only chain)

Goal
- Develop and validate the full chain using Hakorune only:
  Parser(Stage‑B) → MirBuilder (selfhost‑first) → VM → ny‑llvmc(crate) object/exe.
- Stop optimizations until the chain is green and repeatable on this host.

Scope
- Parser(Stage‑B): JSON v0 correctness for control flow, call/method, literals.
- MirBuilder: stable MIR(JSON) emission (no spurious newbox/MapBox in loop JsonFrag path when not intended).
- VM: execute MIR(JSON) deterministically; stats/dev toggles optional.
- ny‑llvmc(crate): build obj/exe from MIR(JSON); no llvmlite dependency in the default path.

Default Policy
- Defaults remain unchanged for users. All bring‑up aids behind env toggles.
- Logs quiet by default. Dev tags require explicit env.
- Rust layer is used for diagnosis only; development proceeds via Hakorune scripts.

Env Toggles (recommended dev)
- HAKO_SELFHOST_BUILDER_FIRST=1
- NYASH_USE_NY_COMPILER=0 (alias of NYASH_DISABLE_NY_COMPILER)
- HAKO/NYASH_ENABLE_USING=1
- NYASH_PARSER_STAGE3=1, HAKO_PARSER_STAGE3=1
- NYASH_LLVM_BACKEND=crate
- Optional (debug): HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1, HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1

How to run (chain E2E)
1) Emit MIR(JSON):
   - bash tools/hakorune_emit_mir.sh input.hako /tmp/out.json
2) Build EXE (crate):
   - NYASH_LLVM_BACKEND=crate bash tools/ny_mir_builder.sh --in /tmp/out.json --emit exe -o a.out
3) Run + check rc:
   - ./a.out; echo $?

Canaries
- tools/archive/legacy-selfhost/engineering/stageb_loop_json_canary.sh — Program(JSON) shape for loop(i<n){i=i+1}
- tools/archive/legacy-selfhost/engineering/phase216_chain_canary.sh — end‑to‑end EXE rc=10 for minimal loop

Provider Path Notes (Dev)
- Optional normalization for provider output is available via `HAKO_MIR_NORMALIZE_PROVIDER=1`.
  - This applies the same JsonFrag normalizer/purifier to MIR(JSON) emitted by the Rust Provider path.
  - Keep defaults unchanged; use only during bring‑up to eliminate ret‑after effects.
- `HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1` now short‑circuits both selfhost‑first and provider‑first wrappers to emit a minimal, pure control‑flow MIR suitable for EXE build sanity.
  - Default OFF; intended for small canaries and performance harness bring‑up.

Removal Plan for temporary parser fallback
- Once VM/gpos interaction is fixed and parser emits correct loop JSON without guards,
  remove the conservative fallback in ParserControlBox.parse_loop.
