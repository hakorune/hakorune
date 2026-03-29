# llvmlite Harness（compat/probe keep lane）

Purpose
- Python + llvmlite による compat/probe/canary 経路を提供する。
- shared MIR / ABI / parity contract の確認を支える keep lane として維持する。
- daily mainline backend owner は `ny-llvmc` であり、この文書は keep lane だけを扱う。

Route at a glance
- daily mainline: `.hako -> ny-llvmc (boundary default route) -> object/exe`
- explicit keep lane: `.hako -> ny-llvmc --driver harness` or `NYASH_LLVM_USE_HARNESS=1 -> tools/llvmlite_harness.py -> src/llvm_py/**`
- llvmlite is never the default route; it is only entered by explicit opt-in or replay.

Switch
- `NYASH_LLVM_USE_HARNESS=1` で explicit keep lane として起動する。
- daily route から自動選択される mainline backend ではない。

Tracing
- `NYASH_LLVM_TRACE_FINAL=1` を設定すると、代表コール（`Main.node_json/3`, `Main.esc_json/1`, `main` 等）を標準出力へ簡易トレースします。
  ON/OFF の最終 JSON 突合の補助に使用してください。

Protocol
- Input: MIR14 JSON（Rust 前段で Resolver/LoopForm 規約を満たした形）。
- Output: `.o` オブジェクト（既定: `NYASH_AOT_OBJECT_OUT` または `NYASH_LLVM_OBJ_OUT`）。
- 入口: `ny_main() -> i64`（戻り値は exit code 相当。必要時 handle 正規化を行う）。

Current ownership
- `.hako` caller の official facade は `LlvmBackendBox`
- C ABI transport は `hako_aot` / `hako_llvmc_ffi.c`
- concrete `MIR(JSON) -> {object, executable}` owner は `ny-llvmc`
- `llvmlite` harness は `ny-llvmc --driver harness` から使われる explicit keep lane

CLI（crate）
- `crates/nyash-llvm-compiler` 提供の `ny-llvmc` は `MIR(JSON) -> {object, executable}` の stable CLI contract だよ。
- `llvmlite` はその implementation detail の keep lane で、必要なときだけ `--driver harness` から通る。
  - ダミー: `./target/release/ny-llvmc --dummy --out /tmp/dummy.o`
  - JSON→.o: `./target/release/ny-llvmc --in mir.json --out out.o`
  - JSON→EXE（新規）: `./target/release/ny-llvmc --in mir.json --emit exe --nyrt target/release --out app`
    - `--nyrt <dir>` で `libnyash_kernel.a` の位置を指定
    - 追加フラグは `--libs "<flags>"` で渡せる（例: `--libs "-static"`）
  - keep lane のスクリプトは `tools/llvmlite_harness.py`（`--harness` で上書き可）。

Quick Start
- 依存: `python3 -m pip install llvmlite`
- ダミー生成（keep lane 配線検証）:
  - `python3 tools/llvmlite_harness.py --out /tmp/dummy.o`
  - `nyash_kernel` とリンクして EXE 化（例: `cc /tmp/dummy.o -L target/release -Wl,--whole-archive -lnyash_kernel -Wl,--no-whole-archive -lpthread -ldl -lm -o app_dummy`）。

Wiring（Rust 側）
- `NYASH_LLVM_USE_HARNESS=1` のとき:
  1) Rust helper が temp MIR(JSON) ファイルを書き出す
  2) `python3 tools/llvmlite_harness.py --in <mir.json> --out <obj.o>` を直接起動
  3) 成功後は通常のリンク手順（`libnyash_kernel.a` とリンク）
  - Rust 側の object emit は MIR JSON を文字列に戻して legacy front door に渡さない

Mainline note
- current daily/mainline route は `ny-llvmc` の default boundary route だよ。
- `llvmlite` は retire 済みではないが、hot-path design owner でもない。
- perf / route collapse / EXE daily acceptance は `ny-llvmc` 側で読む。

Tools / CLI（統合フロー）
- crate 直結の EXE 出力: `tools/build_llvm.sh apps/tests/ternary_basic.hako -o app`
  - 環境変数 `NYASH_LLVM_NYRT` で NyRT の場所を、`NYASH_LLVM_LIBS` で追加フラグを指定できる。
- keep lane 明示実行:
  - `NYASH_LLVM_COMPILER=harness tools/build_llvm.sh apps/tests/ternary_basic.hako -o app`
  - `./target/release/ny-llvmc --driver harness --in mir.json --out out.o`

Scope（Phase 15）
- 最小命令: Const/BinOp/Compare/Branch/Jump/Return（PHI は LLVM 側で合成）
- 文字列: NyRT Shim（`nyash.string.len_h`, `charCodeAt_h`, `concat_hh`, `eq_hh`）を declare → call
- NewBox/ExternCall/BoxCall: まずは固定シンボル／by-id を優先（段階導入）
- 目標: `apps/selfhost/tools/dep_tree_min_string.hako` の `.ll verify green → .o` 安定化

Acceptance
- Harness ON/OFF で機能同値（戻り値/検証）。代表ケースで `.ll verify green` と `.o` 生成成功。

Notes
- 初版は固定 `ny_main` から開始してもよい（配線確認）。以降、MIR 命令を順次対応。
- ハーネスは自律（外部状態に依存しない）。エラーは即 stderr に詳細を出す。
- mainline caller は `--driver harness` や `NYASH_LLVM_USE_HARNESS=1` に依存しない。

PHI Policy（要点）
- Phase‑15 の既定は PHI‑on。MIR 側で SSA `Phi` を生成し、ハーネスは incoming の検証と最終 IR への反映だけを行う。
- レガシー互換のために PHI‑off が必要なケースでは `NYASH_MIR_NO_PHI=1` を明示してね（ハーネスは旧 edge-copy 互換ルートで補完する）。
- 詳細と背景は `docs/reference/mir/phi_policy.md` を参照。

Schema Validation（任意）
- JSON v0 のスキーマは `docs/reference/mir/json_v0.schema.json` にあるよ。
- 検証: `python3 tools/validate_mir_json.py <mir.json>`（要: `python3 -m pip install jsonschema`）。

Appendix: 静的リンクについて
- 生成 EXE は NyRT（libnyrt.a）を静的リンク。完全静的（-static）は musl 推奨（dlopen 不可になるため動的プラグインは使用不可）。
