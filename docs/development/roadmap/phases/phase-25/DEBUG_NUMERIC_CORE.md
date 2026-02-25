# Phase 25 — numeric_core / AotPrep デバッグ指針

Status: memo（Claude Code / Codex 向け運用ガイド）

Phase 25 で導入した `numeric_core.hako`（BoxCall→Call 変換パス）が動かない・怪しいときに辿るルートをまとめる。
Rust 側の修正に入る前に、ここに沿って Hako 側の構造・設定を確認する。

## 1. まず「パスが本当に動いているか」を確認する

### 1-1. using / nyash.toml マッピング

- `nyash.toml` に次のマッピングが存在するか確認する:
  - `selfhost.llvm.ir.aot_prep.passes.numeric_core = "lang/src/llvm_ir/boxes/aot_prep/passes/numeric_core.hako"`
- `numeric_core` が見つからない場合でも致命的エラーにならず、単にパスがロードされないだけ、という挙動になりがちなので注意する。

### 1-2. AotPrep.run_json 経由での実行確認

- `tools/hakorune_emit_mir.sh` を使って、AotPrep が numeric_core を呼んでいるかを確認する:
  - `HAKO_APPLY_AOT_PREP=1`
  - `NYASH_AOT_NUMERIC_CORE=1`
  - `NYASH_AOT_NUMERIC_CORE_TRACE=1`
- 期待するログ:
  - `[aot/numeric_core] PASS RUNNING`
  - MatI64 検出や変換結果に関するメッセージ。
- これらが 1 行も出ない場合:
  - `using selfhost.llvm.ir.aot_prep.passes.numeric_core as AotPrepNumericCoreBox` が解決できていない
  - もしくは `NYASH_AOT_NUMERIC_CORE` が子プロセスに渡っていない
  可能性が高い。

## 2. standalone で numeric_core を動かしてみる

### 2-1. 最小 MIR(JSON) 入力を用意する

- `MatI64.mul_naive` を 1 回だけ呼ぶような小さな .hako を用意し、MIR(JSON v0) を吐き出す:
  - 例: `tmp/matmul_core_mir2.json`
- 重要なのは、JSON 内に次の 2 種類の命令が含まれていること:
  - `"op":"const_string", "value":"MatI64"` 相当の文字列定数。
  - `"op":"boxcall", "box":..., "method":"mul_naive", ...` の BoxCall。

### 2-2. numeric_core.hako を直接呼ぶ

- selfhost 側で `AotPrepNumericCoreBox.run(json_text)` を直接呼び出す小さなハーネスを作るか、既存のテストスクリプト（例: `/tmp/run_numeric_core_test.sh`）を流用する。
- 期待する振る舞い:
  - 型テーブルに MatI64 関連のエントリが 2〜3 件登録される。
  - BoxCall が `Call("NyNumericMatI64.mul_naive", args=[receiver, ...])` に 1 件だけ変換される。
  - 元の BoxCall 行は JSON から削除されている。

## 3. JSON スキャンの典型的な落とし穴

### 3-1. JSON 全体を 1 オブジェクトとして扱ってしまうバグ

- 過去のバグ:
  - `text.indexOf("{", pos)` でオブジェクト開始を探した結果、JSON 全体のルート `{` を拾ってしまい、
  - `instructions` 配列の全要素を 1 つの巨大な「inst」として扱ってしまった。
- 結果:
  - 最初の命令の `dst:0` と、どこかにある `"MatI64"` 文字列が同じオブジェクトに含まれてしまい、型推論が完全に壊れる。

### 3-2. 修正パターン（op-marker-first）

- オブジェクト境界は次の手順で決める:
  1. `pos` 以降で `"op":"` を検索する。
  2. 見つかった位置から後方へ `lastIndexOf("{", op_pos)` して、その命令オブジェクトの開始位置を求める。
  3. その開始位置から `_seek_object_end(text, obj_start)` を呼んで終了位置を決める。
- ポイント:
  - `_seek_object_end` 自体は「開始位置を与えればそのオブジェクトの終端を返す」役割に限定し、
  - 「どの `{` を開始とみなすか」という責務は外側のスキャンロジックに持たせる。

## 4. BoxCall→Call 変換結果の確認ポイント

### 4-1. 変換前後の JSON 断片

- 変換前（例）:
  - `{"args":[3],"box":2,"dst":4,"method":"mul_naive","op":"boxcall"}`
- 変換後（期待）:
  - `{"dst":4,"op":"call","name":"NyNumericMatI64.mul_naive","args":[2,3]}`

チェックリスト:
- [ ] `op:"boxcall"` が `"op":"call"` に変わっている。
- [ ] `name` フィールドに `NyNumericMatI64.mul_naive` が入っている。
- [ ] `args` の先頭が receiver（元の `box` のレジスタ）になっている。
- [ ] 元の BoxCall 行が JSON から消えている。

### 4-2. MatI64 型検出ログ

- TRACE=1 のとき、次のようなログが出ているか確認する:
  - `[aot/numeric_core] MatI64.new() result at r2`
  - `[aot/numeric_core] MatI64.new() result at r3`
  - `[aot/numeric_core] type table size: 3`
- これらが出ていない場合:
  - `"MatI64"` 文字列定数の検出に失敗しているか、
  - `MatI64.new` のパターンが期待とずれている（関数名・引数数など）のどちらか。

## 5. VM/LLVM ラインでの最終確認

### 5-1. VM ライン

- `NYASH_AOT_NUMERIC_CORE=0`:
  - 既存挙動が変わっていないか確認する（BoxCall のままでも OK）。
- `NYASH_AOT_NUMERIC_CORE=1`:
  - VM 実行結果が OFF のときと一致することを確認する。
  - 可能であれば、AotPrep 適用後の MIR(JSON) を一度ダンプし、`boxcall` が消えて `call` になっていることを目視確認する。

### 5-2. LLVM ライン

- `NYASH_AOT_NUMERIC_CORE=1 NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1` など、最小構成で `--backend llvm --exe` による実行を行う。
- 期待すること:
  - LLVM コンパイルがエラーなく通る。
  - VM ラインと同じ数値結果が得られる。
  - IR ダンプを取った場合、`NyNumericMatI64.mul_naive` が通常の `call` として現れ、BoxCall 特有のノードは存在しない。

## 6. それでも原因が分からないとき

- ここまでの手順を踏んでも原因が特定できない場合は、次の情報を CURRENT_TASK.md に貼ってから LLM にバトンタッチする:
  - AotPrep 適用前後の MIR(JSON) の短い抜粋（変換対象の関数のみ）。
  - `NYASH_AOT_NUMERIC_CORE_TRACE=1` 時点の `[aot/numeric_core]` ログ。
  - 使用した .hako ファイル名とベンチ名（例: `matmul_core`）。
- そのうえで、「numeric_core のどこまで動いていて、どの段階で期待と違うか」を一言で書いておくと、後続の LLM（Claude Code など）がすぐに再現・解析しやすくなる。

