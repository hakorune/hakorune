# Numeric ABI (IntArrayCore / MatI64) — Design Stage

Status: design-stage; Phase 25 scope; no behaviour change yet.

## Purpose

- Define a minimal, stable **numeric ABI** for IntArrayCore / MatI64 that:
  - 実装としては Ring1 Hakorune の通常関数（`Call`）として表現でき、
  - 必要に応じて Ring0 Rust / C 側からも同じシグネチャで呼び出せる（将来の FFI/ExternCall 連携の基礎になる）。
- Keep **all box-specific knowledge in Ring1**:
  - Box/Handle/Core の構造（ptr/len/rows/cols/stride 等）。
  - 行列積などの数値アルゴリズム本体。
  - 境界チェックと Fail‑Fast ポリシー。
- Allow Ring0/LLVM to treat numeric calls as:
  - 通常の関数呼び出し（`Call @ny_numeric_*`）として扱えるようにし、
  - 必要な場合のみ、既存の `ExternCall`/FFI 構造の上に同じ関数群を載せ替えられるようにする（Phase 25 では必須ではない）。

Related docs:
- Phase 25 roadmap: `docs/private/roadmap2/phases/phase-25/README.md`
- Ring1 numeric runtime layout: `lang/src/runtime/numeric/README.md`
- System Hakorune subset (runtime/numeric 用記法): `docs/development/runtime/system-hakorune-subset.md`

## Conventions

- **Types / encoding**
  - Numeric ABI 関数のシグネチャは **i64 と Handle（実体は i64×N）** のみを使う。
  - `IntArrayHandle` / `MatI64Handle` は「i64×N の値」として表現される:
    - C 側イメージ（概念）:
      - `typedef struct { int64_t *ptr; int64_t len; } ny_intarray_handle;`
      - `typedef struct { int64_t *ptr; int64_t rows; int64_t cols; int64_t stride; } ny_mat_i64_handle;`
    - MIR では「複数の i64 をまとめた値」として `Call` の引数/戻り値に現れる。
- **命名**
  - Ring1/Hakorune 側では、`ny_numeric_intarray_*` / `ny_numeric_mat_i64_*` 系の関数名（またはそれに相当するモジュール＋メソッド名）で実装する。
  - C/Rust 側から直接呼びたい場合は、同じ名前でシンボルをエクスポートする（必要になったタイミングで ExternCall に載せ替え可能）。
- **Fail‑Fast**
  - numeric ABI では、事前条件違反（負の長さ、境界外 index、dims 不整合）は **すべてバグ扱い** とし、Fail‑Fast で扱う。
  - 「エラーコードを返して上層で if」するスタイルは禁止（AGENTS.md の対処療法禁止に準拠）。

## ABI Surface — IntArrayCore (i64 1D)

Conceptual handle:
- `IntArrayHandle`: 実体は「`ptr: i64` + `len: i64`」等で構成される Core 構造体を指す値。
- Box レベル仕様は `lang/src/runtime/numeric/README.md` の IntArrayCore セクションを参照。

Functions (proposal; Phase 25 scope):

- `ny_numeric_intarray_new(len: i64) -> IntArrayHandle`
  - Allocate zero-initialized buffer of length `len`.
  - Preconditions:
    - `len >= 0`
    - `len` が i64 範囲内であり、`len * sizeof(i64)` の計算がオーバーフローしない。
  - Failure:
    - OOM / 明らかな誤りは Fail‑Fast（戻り値でのエラー表現は行わない）。

- `ny_numeric_intarray_free(a: IntArrayHandle)`
  - Free underlying Core.
  - Double-free / invalid handle は未定義動作とし、所有権設計（Ring1 側）で防ぐ。

- `ny_numeric_intarray_len(a: IntArrayHandle) -> i64`
  - Return current logical length.
  - Used mainly for assertions / diagnostics;通常は Box 側で長さを保持しない方向に寄せる。

- `ny_numeric_intarray_get(a: IntArrayHandle, idx: i64) -> i64`
  - Load `a[idx]`.
  - Preconditions:
    - `0 <= idx < len(a)`.
  - Failure:
    - Precondition violated → Fail‑Fast（例外 or プロセス終了; 実際の実装は後続フェーズで決定）。

- `ny_numeric_intarray_set(a: IntArrayHandle, idx: i64, v: i64)`
  - Store `v` into `a[idx]`.
  - Preconditions:
    - `0 <= idx < len(a)`.
  - Failure:
    - Precondition violated → Fail‑Fast。

Mapping to MIR（Phase 25 の基本方針）:
- BoxCall（例）: `BoxCall(IntArrayCore, "get_unchecked", [arr, idx])`
  - AotPrep: 数値コア関数への通常 `Call` に書き換え（例: `Call NyNumericIntArray.get(arr_handle, idx)`）。
  - LLVM/VM: どちらも同じ `Call` を実行するだけで numeric ABI に到達する。

## ABI Surface — MatI64 (i64 2D matrix)

Conceptual handle:
- `MatI64Handle`: 「`ptr`, `rows`, `cols`, `stride`」を持つ Core 構造を指す値。
- Box レベル仕様は `lang/src/runtime/numeric/README.md` の MatI64 セクションを参照。

Functions (proposal; Phase 25 scope):

- `ny_numeric_mat_i64_new(rows: i64, cols: i64) -> MatI64Handle`
  - Allocate zero-initialized `rows x cols` matrix, row‑major。
  - Preconditions:
    - `rows >= 0`, `cols >= 0`.
    - `rows * cols` が i64 範囲内であり、`rows * cols * sizeof(i64)` がオーバーフローしない。

- `ny_numeric_mat_i64_free(m: MatI64Handle)`
  - Free underlying matrix Core（所有権は Box 側で管理）。

- `ny_numeric_mat_i64_dims(m: MatI64Handle) -> (rows: i64, cols: i64)`
  - Return matrix dimensions（行数・列数）。

- `ny_numeric_mat_i64_get(m: MatI64Handle, row: i64, col: i64) -> i64`
  - Load element at `(row, col)`.
  - Preconditions:
    - `0 <= row < rows(m)`, `0 <= col < cols(m)`.

- `ny_numeric_mat_i64_set(m: MatI64Handle, row: i64, col: i64, v: i64)`
  - Store `v` into `(row, col)`.
  - Preconditions:
    - 同上。

- `ny_numeric_mat_i64_mul_naive(a: MatI64Handle, b: MatI64Handle, n: i64) -> MatI64Handle`
  - Naive O(n³) matrix multiplication for `n x n` matrices.
  - Preconditions:
    - `rows(a) == cols(a) == rows(b) == cols(b) == n`.
    - `n >= 0`。
  - Failure:
    - 上記条件を満たさない場合は Fail‑Fast。ベンチマークバグを隠さない。

Mapping to MIR（Phase 25 の基本方針）:
- BoxCall（例）: `BoxCall(MatI64, "mul_naive", [a, b])`
  - AotPrep: `Call NyNumericMatI64.mul_naive(a_handle, b_handle, n)` のような numeric core 関数呼び出しに書き換え。
  - LLVM/VM: どちらも同じ `Call` を実行するだけで numeric ABI に到達する。

## BoxCall / Call / ExternCall の役割分担

- Initial MIR:
  - `BoxCall(MatI64, "new",  [rows, cols])`
  - `BoxCall(MatI64, "mul_naive", [a, b])`
- AotPrep (Ring1):
  - これらを numeric core 関数への `Call` に変換する（BoxCall を LLVM まで持ち込まない）。
  - Box 名やメソッド名ベースの一時的 if ではなく、「Box 型ID + メソッドID → numeric core 関数 ID」テーブル化を目指す。
- Ring0 / LLVM:
  - `Call @ny_numeric_*` を普通の関数呼び出しとして扱う。
  - 低レベルのメモリアクセスや OS 連携が必要な場合のみ、既存の `ExternCall`（例: `rt_mem_alloc_i64`）を使う。numeric ABI 自体は必須では ExternCall に依存しない。

## Relation to other docs

- Phase 25 roadmap:
  - Numeric ABI の設計方針（Ring0/Ring1 分離、Call/ExternCall のみ、Fail‑Fast）を高レベルで説明。
  - このファイルは、そのうち IntArrayCore / MatI64 に関わる ABI 面のみを詳細化する。
- `lang/src/runtime/numeric/README.md`:
  - IntArrayCore / MatI64 の **Box レベル API**（フィールド・メソッド契約）を定義。
  - Numeric ABI はその下で動く「Core/Handle レベル」の境界として扱う。
- `docs/development/runtime/system-hakorune-subset.md`:
  - Numeric ABI の実装を記述する際に使う Hakorune 言語機能の subset を規定。
  - ループ/Fail‑Fast/Box/Handle パターンなど、実装スタイルを制約する。
