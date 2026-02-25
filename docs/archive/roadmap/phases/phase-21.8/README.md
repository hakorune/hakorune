# Phase 21.8 — Numeric Core Integration & Builder Support

Status: partially completed（builder/imports導線まで・EXEベンチはPhase 25へ移管）

## Goal

Integrate the new numeric core boxes (IntArrayCore + MatI64) into the Hakorune selfhost chain so that:

- Stage‑B → MirBuilder → ny‑llvmc(crate) can emit MIR(JSON) for code that uses:
  - `using nyash.core.numeric.intarray as IntArrayCore`
  - `using nyash.core.numeric.matrix_i64 as MatI64`
- The `matmul_core` microbench（MatI64 + IntArrayCore）については、**構造的な導線（builder/imports）まで本フェーズで整備し、EXE/LLVM ベンチ統合は Phase 25 に移管する**。

21.6 provides the core boxes; 21.8 focuses on wiring them into the builder/runtime chain（Stage‑B/Bridge/MirBuilder）without changing default behaviour for other code. LLVM での数値コア実行経路は Phase 25（Ring0/Ring1 再編・numeric runtime AOT）で扱う。

## Scope (21.8, this host)

- Stage‑B / MirBuilder:
  - Ensure `MatI64` and `IntArrayCore` are recognized as valid boxes when referenced via:
    - `using nyash.core.numeric.matrix_i64 as MatI64`
    - `using nyash.core.numeric.intarray as IntArrayCore`
  - Fix the current provider‑emit failure:
    - Error today: `[mirbuilder/parse/error] undefined variable: MatI64` during `env.mirbuilder.emit`.
    - Diagnose and adjust Stage‑B / MirBuilder so that static box references (`MatI64.new`, `A.mul_naive`) compile in the same way as other boxes.

- AotPrep / emit pipeline:
  - Keep AotPrep unchanged for now; the goal is to make `tools/hakorune_emit_mir.sh` succeed on `matmul_core` sources without special‑casing.
  - Ensure `tools/hakorune_emit_mir.sh` with:
    - `HAKO_APPLY_AOT_PREP=1 NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1`
    - can emit valid MIR(JSON) for MatI64/IntArrayCore code.

- Microbench integration:
  - Finish wiring `matmul_core` in `tools/perf/microbench.sh`:
    - Hako side: MatI64/IntArrayCore based O(n³) matmul (`MatI64.mul_naive`).
    - C side: `MatI64Core { int64_t *ptr; rows; cols; stride; }` with identical algorithm.
  - Accept that performance may still be far from the 80% target; 21.8 focuses on **structural integration and parity**, not tuning.

Out of scope:

- New optimizations inside AotPrep / CollectionsHot.
- SIMD/blocked matmul kernels (to be handled in a later optimization phase).
- f64/complex matrix variants.

## Tasks for implementation (Claude Code)

1) **Fix MatI64 visibility in Stage‑B / MirBuilder**
   - Reproduce the current failure:
     - Use a small `.hako` like:
       - `using nyash.core.numeric.matrix_i64 as MatI64`
       - `static box Main { method main(args) { local n = 4; local A = MatI64.new(n,n); return A.at(0,0); } }`
     - Confirm `env.mirbuilder.emit` reports `undefined variable: MatI64`.
   - Investigate how modules from `nyash.toml` (`"nyash.core.numeric.matrix_i64" = "lang/src/runtime/numeric/mat_i64_box.hako"`) are made visible to Stage‑B and MirBuilder.
   - Adjust the resolver / module prelude so that `MatI64` (and `IntArrayCore`) are treated like other core boxes:
     - Either via explicit prelude inclusion,
     - Or via module registry entries consumed by the builder.

2) **Ensure `tools/hakorune_emit_mir.sh` can emit MIR(JSON) for matmul_core**
   - Once MatI64 is visible, run:
     - `HAKO_APPLY_AOT_PREP=1 NYASH_AOT_COLLECTIONS_HOT=1 NYASH_LLVM_FAST=1 NYASH_MIR_LOOP_HOIST=1 NYASH_JSON_ONLY=1 tools/hakorune_emit_mir.sh <matmul_core.hako> tmp/matmul_core.json`
   - Acceptance:
     - No `undefined variable: MatI64` / `IntArrayCore` errors.
     - `tmp/matmul_core.json` is valid MIR(JSON) (same schema as existing matmul case).

3) **Finish `matmul_core` microbench**
   - Use the existing skeleton in `tools/perf/microbench.sh` (`case matmul_core`):
     - Confirm Hako side compiles and runs under `--backend vm`.
     - Confirm EXE path works:
       - `NYASH_SKIP_TOML_ENV=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/microbench.sh --case matmul_core --backend llvm --exe --runs 1 --n 64`
   - Update `benchmarks/README.md`:
     - Add `matmul_core` row with a short description:
       - “MatI64/IntArrayCore vs MatI64Core C struct (ptr+rows+cols+stride)”
     - Record initial ratios (even if far from 80%).

4) **Keep existing behaviour stable**
   - No changes to default user behaviour, env toggles, or existing benches beyond adding `matmul_core`.
  - Ensure quick/profile smokes (where applicable) remain green with numeric core present.

## 結果（2025-11-14 時点）

本フェーズは「builder/imports 導線の整備」までを完了とし、EXE/LLVM ベンチ統合は Phase 25 に移管する。

**達成済み（21.8）**

- Stage‑B / Bridge / MirBuilder:
  - `BridgeEnv` に `imports: HashMap<String,String>` フィールドを追加し、using 由来のエイリアス情報（alias → box_type）を保持できるようにした。
  - `MapVars::resolve` を拡張し、`env.imports` を参照して `MatI64` / `IntArrayCore` などの using エイリアスを「静的 box 参照」として MIR 上で解決できるようにした。
  - JSON v0 → MIR(JSON) 経路に `*_with_imports` 版を追加:
    - `runner::json_v0_bridge::parse_json_v0_to_module_with_imports`
    - `host_providers::mir_builder::program_json_to_mir_json_with_imports`
  - `HAKO_MIRBUILDER_IMPORTS` 環境変数を経由して、Stage‑B 側で収集した imports を Rust 側の MirBuilder に渡す配線を追加（読み取り側）。
- Using / imports 収集:
  - `collect_using_and_strip` の戻り値を `(cleaned, prelude_paths, imports)` に拡張し、using から alias 情報を収集できるようにした。
  - 既存呼び出し側は `_imports` として無視するため、従来挙動は維持。
- MatI64/IntArrayCore:
  - `using nyash.core.numeric.matrix_i64 as MatI64` / `using nyash.core.numeric.intarray as IntArrayCore` の構文が Stage‑B/Bridge で undefined variable エラーにならないところまで確認済み（builder 経路に乗る）。

**未完・移管（Phase 25 に送るもの）**

- `matmul_core` microbench の EXE/LLVM 統合:
  - MIR 上では `MatI64` が BoxCall として現れるが、LLVM ラインには BoxCall 実行経路がなく、AotPrep も MatI64 向けの書き換えを持たない。
  - 「MatI64/IntArrayCore を LLVM までどう運ぶか」（numeric ABI / AotPrep / Ring1 AOT 設計）は、Phase 25「脱Rustランタイム / Ring0-Ring1再編」で扱う。
- Numeric core AOT ライン:
  - IntArrayCore/MatI64 の実装を `.hako`（Ring1）側に寄せ、LLVM からは汎用 `ExternCall`/numeric ABI のみを見る構造は Phase 25 の設計スコープとする。

詳細な将来計画・numeric runtime AOT 方針については `docs/private/roadmap2/phases/phase-25/README.md` を参照。
