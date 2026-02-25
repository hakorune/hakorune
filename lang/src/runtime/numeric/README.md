# lang/src/runtime/numeric — Ring1 Numeric Runtime

Scope: Phase 21.6+ / 21.8 / 25 で導入する **数値コア箱（numeric core boxes）** を配置するディレクトリ。

## 目的

- ArrayBox/MapBox などの汎用箱とは別に、数値・行列・ビルダ系の「C 相当コア」を Ring1（Hakorune）側で実装する。
- 将来的には、このディレクトリ配下の `.hako` を AOT して `stdlib` 相当の成果物にまとめ、VM 起動時にロードする（Phase 25 以降）。

## 代表モジュール

- `intarray_core_box.hako`
  - 数値一次元配列コア（IntArrayCore）の Box ラッパ。
  - Phase 21.6: Rust プラグイン `IntArrayCore` に委譲。
  - Phase 25: `.hako` 実装に移行し、Ring0 は `alloc/free/load/store` などの intrinsic のみ提供する方針。
- `mat_i64_box.hako`
  - 行列箱 `MatI64`（i64 行列）の Box 実装。
  - Phase 21.6/21.8: IntArrayCore に委譲する naive 実装。
  - Phase 25: numeric ABI（`ny_numeric_mat_i64_*`）の薄ラッパとして整理し、VM / LLVM ともに「BoxCall → numeric core 関数への Call」という同じ MIR を実行する形を目指す（numeric 専用の ExternCall 境界は増やさない）。
  - 現状: 数値核 `mul_naive` の本体は `NyNumericMatI64.mul_naive` に分離し、`MatI64.mul_naive` はその薄ラッパとして実装。

## Phase 25 との関係

- Phase 25 では:
  - IntArrayCore / MatI64 の本体ロジックを `.hako` 側に移し、Rust 側は最小の numeric intrinsic と LLVM/OS FFI だけを持つ Ring0 とする。
  - AotPrep / builder で `BoxCall(MatI64, ...)` / `BoxCall(IntArrayCore, ...)` を **numeric core 関数への通常 `Call`** に変換する設計を導入する（BoxCall を LLVM まで持ち込まない）。
- このディレクトリはそのための **Ring1 numeric runtime の定位置**として扱う。
  - 本番やベンチでは、AOT 済み stdlib（例: `stdlib.hbc`）から `nyash.core.numeric.*` モジュールをロードする embedded モードを使用。
  - 開発時には、`NYASH_STDLIB_MODE=source`（候補）などで埋め込み stdlib を無効化し、このディレクトリ配下の `.hako` を直接コンパイルして挙動を確認する運用を想定。

## IntArrayCore Box API（設計メモ）

実装ファイル: `lang/src/runtime/numeric/intarray_core_box.hako`

### フィールド設計

- `handle: i64`
  - NyRT/Rust 側で管理される IntArrayCore インスタンスへのハンドル。
  - 現行実装では `nyash.intarray.*` C シンボル経由で操作される（Phase 25 以降で numeric ABI に移行予定）。
- `len: i64`
  - 配列長（要素数）。`new(len)` 時の引数と一致する前提。
  - 変更不可（再割り当てや resize は想定しない）。

### メソッド仕様（Box レベル）

- `static new(len: i64) -> IntArrayCore`
  - 役割: 長さ `len` の 0 初期化配列を作成する。
  - 事前条件:
    - `len >= 0` を前提とする（負値はバグ扱い; Fail‑Fast 寄りのポリシーで扱う）。
  - 実装メモ:
    - 現行: `externcall "nyash.intarray.new_h"(len)` を呼び、返ってきたハンドルを `handle` に格納。
    - 将来: numeric ABI（`ny_numeric_intarray_new` 等）経由で Core を構築し、Handle/Core の表現を段階的に置き換える。

- `length() -> i64`
  - 役割: 現在の要素数を返す。
  - 期待挙動:
    - 通常は `len` と同値。
    - Core 側の情報と不整合が生じないよう、Phase 25 以降は「単一の SSOT（Core 側）」に寄せる設計を検討する。

- `get_unchecked(idx: i64) -> i64`
  - 役割: インデックス `idx` から値を取得する。
  - 事前条件:
    - `0 <= idx < length()` を呼び出し側が保証する（unchecked の名の通り）。
  - エラー処理:
    - Phase 21.x 実装では、範囲外アクセス時に 0 を返すなどの挙動が残っているため、Phase 25 では「Fail‑Fast（バグを隠さない）」方向に整理する。
  - 用途:
    - MatI64 や numeric カーネル内部でのループ本体から使用する前提（System Hakorune subset 内）。

- `set_unchecked(idx: i64, v: i64) -> null`
  - 役割: インデックス `idx` に値 `v` を書き込む。
  - 事前条件:
    - `0 <= idx < length()` を呼び出し側が保証する。
  - エラー処理:
    - Phase 21.x 実装では戻り値でエラーコードを返す形跡があるが、Phase 25 では Fail‑Fast で統一し、Box レベルでは成功/失敗を返さない設計を目指す。

## MatI64 Box API（設計メモ）

実装ファイル: `lang/src/runtime/numeric/mat_i64_box.hako`

### フィールド設計

- `rows: i64`
  - 行数（row count）。`rows >= 0` を前提。
- `cols: i64`
  - 列数（column count）。`cols >= 0` を前提。
- `stride: i64`
  - 1 行あたりのステップ幅。現行実装では `cols` と同一（row‑major の連続配置）。
  - 将来、ビューやサブ行列を導入する場合に `stride != cols` を許容する余地を残す。
- `core: IntArrayCore`
  - 実データを保持する一次元配列コア（長さは `rows * cols` を前提）。

### メソッド仕様（Box レベル）

- `static new(rows: i64, cols: i64) -> MatI64`
  - 役割: `rows x cols` のゼロ初期化行列を作成する。
  - 事前条件:
    - `rows >= 0`, `cols >= 0`。
    - `rows * cols` が i64 の範囲内に収まること（オーバーフロー時は Fail‑Fast 寄りに扱う）。
  - 実装メモ:
    - `IntArrayCore.new(rows * cols)` を呼び出し、`stride = cols` として初期化。

- `rowsCount() -> i64`
  - 役割: 行数を返す（読み取り専用）。

- `colsCount() -> i64`
  - 役割: 列数を返す。

- `at(r: i64, c: i64) -> i64`
  - 役割: 要素 `(r, c)` を読み取る。
  - 事前条件:
    - `0 <= r < rows`, `0 <= c < cols` を呼び出し側が保証。
  - 実装メモ:
    - `idx = r * stride + c` を計算し、`core.get_unchecked(idx)` を呼ぶ。
  - 将来:
    - デバッグ/strict モードでは範囲チェックを追加するオプションを検討（prod ではループ性能優先）。

- `set(r: i64, c: i64, v: i64) -> null`
  - 役割: 要素 `(r, c)` に `v` を書き込む。
  - 事前条件:
    - `0 <= r < rows`, `0 <= c < cols`。
  - 実装メモ:
    - `idx = r * stride + c` を計算し、`core.set_unchecked(idx, v)` を呼ぶ。

- `mul_naive(b: MatI64) -> MatI64`
  - 役割: `me * b` のナイーブな O(n³) 行列積を計算する。
  - 事前条件:
    - 現行スケルトンでは「正方行列かつ形状一致」を暗黙前提としている（Phase 25 で明示条件に格上げする）。
    - 最低限 `me.cols == b.rows` を事前条件とし、満たさない場合は Fail‑Fast とする方向。
  - 実装メモ:
    - 三重ループ (`i`,`k`,`j`) で `out[i,j] += me[i,k] * b[k,j]` を計算。
    - `out` 行列は `MatI64.new(me.rows, b.cols)` で確保。
  - Phase 25 以降:
    - Box メソッドとしての `mul_naive` は numeric ABI の薄ラッパに縮退し、ループ本体は System Hakorune subset 上の別関数（`ny_numeric_mat_i64_mul_naive`）に切り出す。
