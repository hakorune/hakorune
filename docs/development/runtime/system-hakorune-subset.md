# System Hakorune Subset — Runtime / Numeric Core Design

Status: design-stage; subset definition only. No behavior change yet.

## Purpose

- Provide a small, predictable subset of Hakorune for implementing **runtime / numeric core logic** in Ring1（IntArrayCore, MatI64, stats, policy, etc.）。
- Make this subset explicit so that:
  - Box / numeric kernels can be moved from Rust to `.hako` **without increasing semantic complexity**.
  - AotPrep / VM / LLVM can rely on a restricted feature set when reasoning about these modules.
- Align with Phase 25 goals:
  - Ring0（Rust）は intrinsic と最小 VM/FFI のみ。
  - Ring1（Hakorune）は「元・C 相当の実装」をこの subset で記述する。

Related docs:
- Phase 25 roadmap: `docs/private/roadmap2/phases/phase-25/README.md`
- Ring1 numeric runtime layout: `lang/src/runtime/numeric/README.md`
- Numeric ABI surface (IntArrayCore / MatI64): `docs/development/runtime/NUMERIC_ABI.md`

## Scope & Typical Use

- 対象モジュールの例:
  - `nyash.core.numeric.intarray`（IntArrayCore 本体）
  - `nyash.core.numeric.matrix_i64`（MatI64 本体）
  - 将来の数値箱（例: F64ArrayCore, MatF64 など）
  - ランタイムポリシー / stats / 軽量な AotPrep 補助ロジック
- 非対象（ここでは扱わないもの）:
  - 高レベルアプリケーションロジック（UI、CLI コマンド等）。
  - 重い I/O / ネットワーク / プラグイン動的ロード。
  - 実験的な言語機能（例外、async 等）を前提とするコード。

## Allowed Features（推奨）

- 制御構造:
  - `if / else`、`while`、インデックス付きの従来型 `for` ループ。
  - ネストは OK だが、循環依存や過度な再帰は避ける。
- データ構造:
  - Box フィールドを明示した構造体的なパターン（`handle.ptr`, `handle.len` 等）。
  - 固定長 or 単純な一次元/二次元配列アクセス（IntArrayCore / MatI64）。
- 関数:
  - 引数/戻り値が `i64` と Box/Handle 型に限定された関数。
  - 純粋 or 副作用が局所（配列/行列への書き込み）のみの関数。
- エラーハンドリング:
  - **Fail‑Fast** を前提とした設計（事前条件違反は即失敗）。
  - 戻り値でのエラーコード運搬は禁止（AGENTS.md の対処療法禁止と合わせる）。

## Restricted / Forbidden Features

- 例外 / 非同期:
  - `throw/try` や async 相当の機能は numeric core の実装では使わない方針（後続フェーズで必要になった場合に個別に設計）。
- 動的ディスパッチ:
  - 文字列ベースの `by-name` ディスパッチ（`if method == "mul"` 等）は避け、事前に決まった numeric ABI 関数を直接呼び出す。
- 動的ロード / プラグイン依存:
  - numeric core から直接プラグインをロードしない。必要なら上位層（アプリ側）がプラグイン経由で呼ぶ。
- 型拡張:
  - 汎用的な「任意型の配列/行列」をここで扱わない。Phase 25 では **i64 専用（IntArrayCore / MatI64）** にスコープを絞る。

## Design Patterns & Guidelines

- 明示的ループ:
  - 数値カーネルは map/filter 的な高階関数ではなく、`for` / `while` による明示ループで書く。
  - これにより AotPrep / LLVM でのループ変換・アンローリング等の解析がしやすくなる。
- Box / Handle の役割分離:
  - Box（例: `MatI64`）は **API と所有権** を管理する層。
  - Handle/Core（例: `IntArrayCore`, `MatI64Core`）は **実データとループ本体** を持つ層。
  - Box メソッドは「引数検査 → numeric ABI 呼び出し」の薄いラッパに留める。
- Fail‑Fast:
  - インデックスや次元の検査は、「バグを隠さない」方向で実装する。
  - `idx < 0` や `idx >= len` など明らかなバグは例外 or プロセス終了で即座に検出し、フォールバックや silent failure は行わない。

## Relation to Ring0 / Numeric ABI

- Ring0（Rust）:
  - IntArrayCore / MatI64 向けには「ptr/len/rows/cols/stride を受け取る intrinsic」だけを提供する。
  - ループ本体や境界チェックは実装しない（System Hakorune subset 側に責務を寄せる）。
- Ring1（Hakorune）:
  - この subset 上で numeric ABI 関数群（`ny_numeric_intarray_*`, `ny_numeric_mat_i64_*`）を実装する。
  - AotPrep は BoxCall → numeric ABI への `ExternCall` 変換を行い、LLVM 側は汎用 Call/ExternCall として扱うだけでよい。

## Roadmap（Phase 25 以降）

- Phase 25:
  - Subset の定義を docs として固定（このファイル）。
  - IntArrayCore / MatI64 の API 仕様と合わせて、どの機能を subset に含めるかを明示する。
- 後続フェーズ（22.x / 26.x など）:
  - 実際の `.hako` 実装を subset 遵守で書き起こす。
  - AotPrep / VM / LLVM の観点から「subset で書かれた numeric core を前提にした最適化/診断」を設計する。
