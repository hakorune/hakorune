# Phase 25 — 脱Rustランタイム / Ring0-Ring1 再編

**Status: ✅ MVP COMPLETED** (2025-11-15)

## 🎉 Phase 25 MVP 完全成功！

**numeric_core BoxCall→Call 変換** が完全動作確認済み！

### 主要成果 (2025-11-14 → 2025-11-15)

1. **✅ 型伝播システム完全動作**:
   - 4回反復型伝播で copy → phi → copy チェーン完全対応
   - MatI64 型を15レジスタまで正しく追跡
   - PHI 検出バグ修正（8d9bbc40）: `indexOf("{")` → `indexOf("\"op\":\"")`

2. **✅ 両SSAパターン対応確認**:
   - 冗長版（13 PHI nodes）: test_direct.json, test_matmul_debug.json
   - 最適化版（1 PHI node）: test_matmul_with_wrapper.json, microbench_matmul_core.json
   - すべてのパターンで BoxCall → Call 変換成功 ✅

3. **✅ 環境変数伝播修正** (3d082ca1):
   - microbench.sh に `NYASH_AOT_NUMERIC_CORE` と `NYASH_AOT_NUMERIC_CORE_TRACE` 伝播追加
   - `tools/perf/microbench.sh --case matmul_core --backend llvm --exe` で完全動作

4. **✅ ログ転送問題根治**:
   - hakorune_emit_mir.sh の provider 経路にログ転送追加（ユーザー実装）
   - `[aot/numeric_core]` ログが NYASH_AOT_NUMERIC_CORE_TRACE=1 で正しく表示

5. **✅ 開発ワークフロー確立**:
   - `tools/dev_numeric_core_prep.sh` で環境変数自動設定
   - 推奨開発フロー確立・ドキュメント化完了

### 変換例

**Before** (BoxCall):
```json
{"args":[15],"box":7,"dst":53,"method":"mul_naive","op":"boxcall"}
```

**After** (Call):
```json
{"dst":53,"op":"call","name":"NyNumericMatI64.mul_naive","args":[7,15]}
```

### 既知の制限・次フェーズ

- `NYASH_AOT_NUMERIC_CORE_STRICT=1`: 検証関数実装済みだが未使用（タイミング問題）
- microbench 性能チューニング: **Phase 25.2** に移管
- 他の numeric メソッド（add, sub, etc.）: 将来対応

---

## Phase status (2025-11-14 - 初期バージョン):
- このフェーズでは「Ring0/Ring1 の設計」と「numeric_core (MatI64.mul_naive) の BoxCall→Call 降ろし用 AotPrep パス」の MVP 実装までをカバーする。
- `NYASH_AOT_NUMERIC_CORE=1` + AotPrep.run_json による MatI64.mul_naive 降ろしは、代表的な MIR パターン（13 PHI / 1 PHI 両方）で動作確認済み。
- `NYASH_AOT_NUMERIC_CORE_STRICT=1` は AotPrep 後の MIR(JSON) に対してのみ BoxCall(mul_naive) 残存をチェックするように整理済み（pre-AotPrep の MirBuilder には干渉しない）。
- microbench（`tools/perf/microbench.sh --case matmul_core --backend llvm --exe`）による EXE/LLVM ベンチ統合と性能チューニングは **Phase 25.2** に移管する。

Related docs:
- `docs/private/roadmap2/phases/phase-25.1/README.md` … Stage0（Rust bootstrap）/Stage1（Hakorune selfhost）によるバイナリ二段構えの設計。
- `docs/development/runtime/NUMERIC_ABI.md` … IntArrayCore/MatI64 など numeric ABI の関数契約。
- `docs/development/runtime/system-hakorune-subset.md` … Ring1/System Hakorune サブセットの範囲と責務。
- `docs/development/runtime/ENV_VARS.md` … `NYASH_AOT_NUMERIC_CORE` など Phase 25 関連の環境変数。

## ゴール

- Rust 層を **Ring0（最小シード）** に縮退し、それ以外のランタイム・数値コア・箱ロジックを **Hakorune(Ring1)** 側へ段階的に移行する。
- 具体的には、Phase 21.6/21.8 で導入した:
  - `IntArrayCore`（数値一次元配列コア）
  - `MatI64`（行列箱・i64版）
 などを、「Rust プラグイン実装」ではなく **Hakorune 実装＋ごく薄い intrinsic** に置き換えるための設計ロードマップを固める。
- 新しい箱・数値カーネル・標準ランタイム機能は **原則 .hako で実装する** 方針を明文化し、「Rust Minimal Policy（Self‑Host First, but not Frozen）」として Phase 25 で具体化する。

## レイヤー方針（Ring0 / Ring1）

### Ring0（Rust / C 相当 ― 最小シード）

**責務:**
- プロセス起動・エントリポイント
- OS / FFI / LLVM C API への極小ラッパ
- VM の実行コア（命令デコード・レジスタファイル・GC/alloc の最小部分）
- **汎用 intrinsic** のみ提供（例: メモリ確保・生ポインタload/store・基本的な memcpy 等）

**禁止 / 抑制:**
- 新しい Box 種類（IntArrayCore / MatI64 / StringBuilder 等）の**本体ロジック**を Rust 側に増やさない（型安全な intrinsic のみに留める）。
- 新しい最適化ロジック・言語ルール・Box メソッド実装を Rust に追加しない（AGENTS.md 5.2 Rust Minimal Policy に準拠）。

### Ring1（Hakorune / Nyash ― System サブセット）

**責務:**
- 数値コア・行列コア・文字列ビルダなどの **「C 言語で書いていた部分」** を、Hakorune で実装する層。
- 代表例:
  - `nyash.core.numeric.intarray.IntArrayCore`
  - `nyash.core.numeric.matrix_i64.MatI64`
  - `StringBuilder` / 将来の `F64ArrayCore` 等
- ランタイムポリシー・統計・ログ・一部の AotPrep / MIR パス（構造的なもの）。

**方針:**
- Rust 側が提供するのは `alloc/free/copy/load/store` などの **型パラメトリックな intrinsic** のみ。
- 箱のフィールド管理（ptr+len+stride）、境界チェック、ループ本体、行列演算アルゴリズムなどは **すべて .hako 側で記述**。
- Ring1 コードは AOT して `stdlib` 相当の成果物（例: `stdlib.hbc`）として VM 起動時にロードする構造を目指す。

### Rust ソースの保存ポリシー

- 本フェーズは **Rust コードの削除ではなく、責務の縮退と凍結** を目的とする。
- `src/` 以下の Ring0 Rust ソース（VM コア / LLVM・OS FFI / 起動コード）は、将来もブートストラップ用としてリポジトリに残す前提とする。
- 脱Rustが進み、将来 Hakorune EXE 単独で自己ホスト可能になっても:
  - Ring0 Rust は「アーカイブ兼非常用バックアップ」として保持する。
  - 削除や完全な Rust 依存断絶は、別フェーズ（かつ明示的な設計・合意）なしには行わない。

## スコープ（Phase 25）

Phase 25 は「設計とロードマップの確定」が主目的。実装・移行作業自体は後続フェーズ（22.x/26.x など）で分割実施する。

### 1) Rust Minimal Policy の明文化とチェックリスト

- 既存の「Rust Freeze Policy（Self‑Host First）」を、「Self‑Host を支える最小＋必要な整備は許可する」Rust Minimal Policy として再整理:
  - 新規 Box / ランタイム機能は Rust ではなく .hako で実装する。
  - Rust 変更は「最小の intrinsic 追加」「Stage‑1/Stage‑B ブリッジの改善」「エラーログ・可観測性の向上」か「バグ修正」に限定。
- PR / フェーズ用チェックリスト案を作成:
  - [ ] この変更は Ring0 の責務か？（VM/allocator/LLVM/OS FFI のみ）
  - [ ] 新しい Box/アルゴリズムを Rust に追加していないか？
  - [ ] .hako に移せる部分が残っていないか？

### 2) IntArrayCore / MatI64 の移行設計

- 現状:
  - Phase 21.6: Rust プラグイン `IntArrayCore` + Hako ラッパ Box。
  - Phase 21.8: `MatI64` Box を Hako で実装しつつ、コア配列は IntArrayCore に依存。
- 目標:
  - IntArrayCore の **本体ロジック（len 管理・get/set/fill 等）を Hako 側に移す**。
  - Rust 側は:
    - `rt_mem_alloc_i64(len) -> (ptr,len)`
    - `rt_mem_free_i64(ptr,len)`
    - `rt_unsafe_load_i64(ptr, idx)`
    - `rt_unsafe_store_i64(ptr, idx, val)`
    など、小さな intrinsic 群に縮退。
- タスク（設計レベル）:
  - 必要な intrinsic セットの定義（型・エラー処理ポリシー・Fail‑Fast方針）。
  - `nyash.core.numeric.intarray` / `nyash.core.numeric.matrix_i64` の API 仕様と内部構造（ptr+len/rows/cols/stride/所有権/ライフサイクル）を docs に固定。
    - Box レベルの仕様: `lang/src/runtime/numeric/README.md` に IntArrayCore / MatI64 のフィールド・メソッド契約を記述。
    - Core / Handle レベルの仕様: 本ファイルおよび System Hakorune subset / numeric ABI ドキュメントで補完。
  - MatI64 が IntArrayCore をどう利用するか（row-major/stride/ビューなど）を整理。

#### 2.1) Numeric ABI（IntArrayCore / MatI64）の詳細方針

**ゴール:**

- MatI64 / IntArrayCore のような数値箱を LLVM まで運ぶ際に、「箱の知識」は Ring1 に閉じ込め、Ring0（Rust+LLVM）は汎用的な `Call` / `ExternCall` しか知らなくて済む構造にする。

**基本方針:**

- BoxCall を LLVM まで持ち込まない:
  - LLVM に渡す最終 MIR には、原則として:
    - `Const / BinOp / Compare / Branch / Jump / Ret`
    - `Call`（通常の関数呼び出し）
    - `ExternCall`（NyRT / 低レベル intrinsic 呼び出し）
    - `NewBox`（必要最小限）
  - `BoxCall(MatI64, ...)` や `BoxCall(IntArrayCore, ...)` は AotPrep で潰す。
- 箱の構造・都合は Ring1 で完結させる:
  - MatI64 / IntArrayCore のフィールド構造やメソッドは Ring1 が知るだけでよい。
  - Ring0/LLVM から見ると「固定 ABI の関数呼び出し」に見えるようにする。

**Ring1 側の責務: numeric ABI 定義とラッパー**

- IntArrayCore / MatI64 向けに固定された numeric ABI 関数セットを定義する（例・概念レベル）:
  - `ny_numeric_intarray_new(len: i64) -> IntArrayHandle`
  - `ny_numeric_intarray_get(a: IntArrayHandle, idx: i64) -> i64`
  - `ny_numeric_intarray_set(a: IntArrayHandle, idx: i64, v: i64)`
  - `ny_numeric_mat_i64_new(rows: i64, cols: i64) -> MatI64Handle`
  - `ny_numeric_mat_i64_mul_naive(a: MatI64Handle, b: MatI64Handle, n: i64) -> MatI64Handle`
- 具体的な関数一覧・事前条件・Fail‑Fast 方針は `docs/development/runtime/NUMERIC_ABI.md` に整理する。
- 実装:
  - Phase 25 の段階では、これらを **Ring1 の通常の Hako 関数** として実装し、MIR 上では `Call` 命令として現れる形を基本とする。
  - 将来、別バイナリや C 実装に差し替える場合のみ、同じ関数群を ExternCall/FFI 経由で公開する案を検討する（Phase 26 以降）。
  - これらの関数名・引数型を「numeric ABI」として docs に固定する。
- MatI64 / IntArrayCore Box メソッドは「numeric ABI の薄ラッパ」として実装する:
  - 例: `MatI64.mul_naive(self, rhs, n)` の本体は Ring1 numeric core 関数（例: `NyNumericMatI64.mul_naive(self, rhs, n)`）を 1 回呼ぶだけ。
  - VM/インタープリタライン: BoxCall をそのまま実行すればラッパ経由で numeric core に到達する。
  - AOT/LLVM ライン: BoxCall を numeric core 関数への `Call` に書き換えるだけで済む（BoxCall を LLVM まで持ち込まない）。

**BoxCall → Call（numeric core）変換（AotPrep / builder の責務）**

- 初期 MIR では `BoxCall(MatI64, "new", ...)` や `BoxCall(MatI64, "mul_naive", ...)` が現れる。
- Ring1 の AotPrep パスで、これらを numeric core 関数への `Call` に変換する計画とし、Phase 25 ではそのための診断パス（`AotPrepNumericCoreBox`）を用意する:
  - 例（概念レベル）:
    - `BoxCall(MatI64, "new",  ...)` → `Call NyNumericMatI64.new_core(...)`
    - `BoxCall(MatI64, "mul_naive", ...)` → `Call NyNumericMatI64.mul_naive(...)`
- これらは一時しのぎのハードコードではなく、Ring1 numeric ランタイムの正規インターフェースとして docs に固定する（実際の書き換えは後続フェーズで実装）。
- 拡張性:
  - 可能なら「Box 型ID + メソッドID → numeric core 関数 ID」のテーブルで持つ（メタデータ化）。
  - 少なくとも `MatI64` / `IntArrayCore` を識別する Box 型IDを見てから変換する方針にする（文字列 if の乱立を避ける）。

**Ring0 側の責務: 汎用 Call/ExternCall のみ**

- LLVM backend は汎用的な `Call` / `ExternCall` のコード生成のみ実装する。
  - `Call` → Hako から生成された通常の関数呼び出しに変換（numeric core 関数もここに含まれる）。
  - `ExternCall` → NyRT / OS / C など「Hakorune 外部」の FFI だけを扱う（`rt_mem_alloc_i64` 等）。
- Ring0 は「MatI64 という箱がある」「IntArrayCore という型がある」といった情報を持たない。
  - numeric core について知っているのは「`Call @ny_numeric_*` という形の関数が存在する」という事実だけであり、Box 型や内部フィールド構造は Ring1 に閉じ込める。

**Handle / Core の設計ポリシー（概念レベル）**

- `IntArrayHandle` / `MatI64Handle` は実質 Core 構造を指すものとして扱う:
  - 例: `struct IntArrayCore { i64* ptr; i64 len; };`
    - `struct MatI64Core { i64* ptr; i64 rows; i64 cols; i64 stride; };`
- Box 側（MatI64 Box）はこれら Core をラップするだけにする。
- GC を導入する場合、numeric Core は pinned / non‑moving 領域または明示的 malloc/free 管理とし、Box→Core の所有権・ライフサイクルを Ring1 側で管理する。

**ABI 関数セット（初期案の固定方針）**

- IntArrayCore（1 次元 i64 配列）:
  - `ny_numeric_intarray_new(len: i64) -> (IntArrayHandle)`  
    - 役割: 長さ `len` のゼロ初期化配列を確保する。
    - 失敗時: OOM など致命的エラーは Fail‑Fast（プロセス終了または未定義だが「静かに 0 を返す」等は禁止）。
  - `ny_numeric_intarray_free(a: IntArrayHandle)`  
    - 役割: Core を解放する（多重 free は未定義とし、Ring1 側の所有権設計で防ぐ）。
  - `ny_numeric_intarray_len(a: IntArrayHandle) -> i64`  
    - 役割: 現在の長さを返す（境界チェック不要）。
  - `ny_numeric_intarray_get(a: IntArrayHandle, idx: i64) -> i64`  
    - 役割: `0 <= idx < len` を前提とした読み取り。範囲外は Fail‑Fast。
  - `ny_numeric_intarray_set(a: IntArrayHandle, idx: i64, v: i64)`  
    - 役割: `0 <= idx < len` を前提とした書き込み。範囲外は Fail‑Fast。

- MatI64（2 次元 i64 行列; row‑major）:
  - `ny_numeric_mat_i64_new(rows: i64, cols: i64) -> (MatI64Handle)`  
    - 役割: 行列本体を `rows * cols` 要素で確保し、ゼロ初期化する。
  - `ny_numeric_mat_i64_free(m: MatI64Handle)`  
    - 役割: Core を解放する（所有権は Box 側が管理）。
  - `ny_numeric_mat_i64_dims(m: MatI64Handle) -> (rows: i64, cols: i64)`  
    - 役割: 行数・列数を返す（構造検査用）。
  - `ny_numeric_mat_i64_get(m: MatI64Handle, row: i64, col: i64) -> i64`  
    - 役割: `0 <= row < rows`, `0 <= col < cols` を前提とした読み取り。範囲外は Fail‑Fast。
  - `ny_numeric_mat_i64_set(m: MatI64Handle, row: i64, col: i64, v: i64)`  
    - 役割: 上記と同じ前提の書き込み。範囲外は Fail‑Fast。
  - `ny_numeric_mat_i64_mul_naive(a: MatI64Handle, b: MatI64Handle, n: i64) -> MatI64Handle`  
    - 役割: `n x n` 行列同士のナイーブな行列積。`n` と `dims` の不整合は Fail‑Fast（ベンチ用の前提エラーは早期に止める）。

**ABI 型と呼び出し規約（概念レベル）**

- `IntArrayHandle` / `MatI64Handle` は LLVM / C 側では「Core 構造体を値渡しする ABI」として扱う案を第一候補とする:
  - C 側イメージ（proposal）:
    - `typedef struct { int64_t *ptr; int64_t len; } ny_intarray_handle;`
    - `typedef struct { int64_t *ptr; int64_t rows; int64_t cols; int64_t stride; } ny_mat_i64_handle;`
  - MIR から見ると「2 〜 4 個の `i64` をまとめた値」として ExternCall の引数/戻り値に現れる。
  - 将来、GC 等でハンドルをテーブル管理に変えたくなった場合も、「ハンドルは ABI 上は i64×N で表現する」という規約だけを維持すればよい。
- ExternCall 側の型:
  - `ExternCall` から見える型はすべて `i64` のみとし、「どのスロットが ptr/len/rows/cols か」は numeric ABI 側の約束で固定する。
  - これにより LLVM backend は「i64 のタプルをそのまま C 関数に渡す」だけで済み、箱/行列の構造を知らなくてよい。

**エラー処理と Fail‑Fast ポリシー**

- OOM / 致命的エラー:
  - numeric ABI レベルでは「戻り値でのエラー表現」は行わず、Fail‑Fast を原則とする（プロセス終了 or 例外経路など、実装詳細は後続フェーズで決める）。
  - 「負の長さ」「rows*cols のオーバーフロー」など明らかなバグ入力も Fail‑Fast。
- 境界違反:
  - `*_get` / `*_set` / `*_mul_naive` など、index/dims に依存する API は **事前条件を満たさない呼び出しをすべてバグ扱い** とし、Fail‑Fast する。
  - 「エラーコードを返して上層で if する」スタイルは禁止（AGENTS.md の対処療法禁止と揃える）。
- Box 側との責務分離:
  - Box メソッドは「precondition を満たすように引数を構成して numeric ABI を呼ぶ」責務のみを持ち、境界チェックの抜けや重複を避ける。
  - numeric ABI 側は「precondition 違反を検出したら即 Fail‑Fast」することで、バグを早期発見する。

### 3) System Hakorune サブセットの定義

- Ring1 で「C 代替」として安全に使える記法/機能を定義:
  - 推奨: 明示ループ（while/for）、Fail‑Fast、Box フィールドの明示管理。
  - 慎重に: 例外/非同期/動的ロードなど、ランタイム依存が重い機能。
- ドキュメント:
  - `docs/development/runtime/system-hakorune-subset.md`（本ガイド）
  - 想定ユース:
    - numeric core / matrix core
    - runtime policy / stats
    - 一部 MIR/AotPrep ロジック

### 4) stdlib ビルド／ロード戦略のたたき台

- 目標:
  - 「Hakorune で書かれた runtime/numeric コード」を AOT して、VM 起動時に一括ロードする仕組みを設計。
- 方針案:
  - `tools/hakc_stdlib.sh`（仮）で:
    - `lang/src/runtime/**/*.hako` のうち Ring1 対象（特に `lang/src/runtime/numeric/` 以下）をコンパイルして `build/stdlib.hbc` を生成。
  - `hakorune` / `nyash` バイナリ起動時に:
    - `stdlib.hbc` を自動ロード（PATH または env で切り替え）。
  - Phase 25 では「どのモジュールを stdlib に含めるか」「ビルド/ロードの責任境界」を文章で決めるところまで。

### 5) stdlib モードと衝突回避ポリシー（embedded / source）

- 目的:
  - IntArrayCore / MatI64 など、同じモジュール名を持つ数値箱が「埋め込み stdlib」と「開発中 .hako ソース」で二重定義されて衝突しないようにする。
- 方針:
  - `nyash.core.numeric.*` 系モジュールは **stdlib 専用の名前空間**として扱い、1 度の実行中に有効な実装は常に 1 つだけとする。
  - 実装の SSOT は `.hako` とし、埋め込みは「その時点の .hako を AOT した成果物」としてのみ存在させる（別実装は持たない）。
- モード案（env ベースの切替; 名前は Phase 26 以降で最終決定）:
  - `NYASH_STDLIB_MODE=embedded`（デフォルト候補）:
    - 起動時に `stdlib.hbc` をロードし、`nyash.core.numeric.*` は埋め込み stdlib から提供。
    - 同じモジュール名をファイルで定義しても、原則として無視 or 警告（開発時のみ許可）とし、実行時には埋め込み版だけが有効になる。
  - `NYASH_STDLIB_MODE=source`（開発専用候補）:
    - `stdlib.hbc` をロードせず、Stage‑B/VM が `lang/src/runtime/numeric/*.hako`（など）を直接コンパイルして runtime/numeric を提供。
    - このモードでは埋め込み stdlib は無効化され、`.hako` ソースでのみ挙動が決まる。
- 利点:
  - 本番/ベンチでは embedded モードで安定した numeric stdlib を使用できる。
  - 開発時は source モードで IntArrayCore/MatI64 の `.hako` を編集しながら試せる。
  - 「同じ名前の箱が2つ同時に有効になる」状態を構造的に防げる。

## 実装チェックリスト（Phase 25 以降で順番にやる）

Phase 25 自体は設計フェーズだが、後続フェーズ（22.x / 26.x など）で実装を進める際のチェックリストをここにまとめておく。

### A. 設計・ドキュメント

- [ ] Rust Freeze（ランタイム/箱/数値系）の詳細ポリシーを docs に固定する。
  - [ ] 「新しい箱・数値カーネルは .hako で書く」方針を明文化。
  - [ ] Ring0 で許可される変更種別（intrinsic 追加 / バグ修正のみ）を列挙。
- [ ] System Hakorune サブセットのガイド（`docs/development/runtime/system-hakorune-subset.md`）を整備する。
  - [ ] 使用を推奨する構文/機能（ループ、Fail‑Fast 等）。
  - [ ] 慎重に扱う機能（例外/非同期/動的ロード 等）。
- [ ] IntArrayCore / MatI64 の API 仕様と内部構造を docs で固定する。
  - [ ] フィールド（ptr/len/rows/cols/stride 等）の意味と所有権ポリシー。
  - [ ] public メソッドとその契約（境界チェック有無、Fail‑Fastポリシー）。
- [ ] Numeric ABI（`ny_numeric_*`）の関数セットを文書化する。
  - [ ] 関数名・引数型・戻り値型・エラーハンドリング規約。
  - [ ] （必要になった場合のみ）C/Rust から呼ぶ際のシンボル名規約を決める。

### B. Ring0（Rust）側の最小実装

- [ ] 既存ランタイムに不足している最小 intrinsic を確認し、必要なら追加する。
  - [ ] `rt_mem_alloc_i64(len) -> (ptr,len)`
  - [ ] `rt_mem_free_i64(ptr,len)`
  - [ ] `rt_unsafe_load_i64(ptr, idx)`
  - [ ] `rt_unsafe_store_i64(ptr, idx, val)`
- [ ] LLVM backend が既存の `ExternCall` メカニズムで Ring0 intrinsic（`rt_mem_*` 等）を扱えることを確認する。
  - [ ] numeric 用に特別な分岐を追加せず、必要なら共通の規約ベースでシンボル名を組み立てる。

### C. Ring1（.hako）側 numeric runtime

- [ ] `nyash.core.numeric.intarray` を Ring1 実装に移行する。
  - [ ] IntArrayCore を `.hako` で実装（ptr+len 管理 / get/set/fill 等）。
  - [ ] 内部で Ring0 intrinsic（alloc/free/load/store）を使用する。
  - [ ] 既存の Rust プラグイン実装との整合性を確認し、最終的に Rust 実装を縮退 or 退役できるようにする。
- [ ] `nyash.core.numeric.matrix_i64`（MatI64）を numeric ABI ベースのラッパ Box に整える。
  - [ ] フィールドに Core ハンドル（MatI64Handle）を持つ構造に整理。
  - [ ] `new/at/set/mul_naive` などのメソッド本体を Ring1 numeric core 関数（通常の Hako 関数）呼び出しに寄せる。
- [ ] Numeric ABI 関数群（`ny_numeric_intarray_*` / `ny_numeric_mat_i64_*`）を `.hako` で実装し、AOT 可能な状態にする。

### D. AotPrep / builder 経路

- [ ] `BoxCall(MatI64, ...)` / `BoxCall(IntArrayCore, ...)` を Ring1 numeric core 関数への通常 `Call` に変換する AotPrep パスを設計する（Phase 25 では診断パスまで、実際の変換は後続フェーズ）。
  - [ ] Box 型ID / メソッド名から numeric core 関数 ID にマップする表（メタ）を用意（対処療法的な文字列 if の乱立を避ける）。
- [ ] 変換後の MIR から `BoxCall` が LLVM ラインには残らないことを確認。
- [ ] imports / using 経路（Phase 21.8 で導入済み）を再確認し、MatI64/IntArrayCore の静的参照が安定して解決されることを確認。

### E. stdlib ビルド／ロード

- [ ] `lang/src/runtime/numeric/*.hako` を含む Ring1 モジュールを AOT して `stdlib.hbc`（仮）にまとめるビルドスクリプト（設計どおりなら `tools/hakc_stdlib.sh` 相当）を用意する。
- [ ] `hakorune` / `nyash` 起動時に `stdlib.hbc` をロードする導線を設計し、Ring0 に最小限のフックを追加する。
- [ ] VM/LLVM 両ラインで numeric runtime が利用できるかを確認する（どちらも BoxCall→Call(numeric core) の同一 MIR を実行する）。

### F. 検証・移行

- [ ] 代表的な数値ベンチ（`matmul_core` など）を:
  - [ ] VM ライン（BoxCall 経路）で確認。
  - [ ] LLVM ライン（numeric ABI 経路）で確認。
- [ ] 21.x の既存ベンチが regression していないことを確認する（数値系以外は挙動不変）。
- [ ] Rust 側の IntArrayCore plugin 実装を「縮退 or optional 化」するタイミングと手順を docs に追記する。

## アウト・オブ・スコープ（Phase 25）

- 実際のコード移行（Rust 実装の削除や .hako への完全移植）は、このフェーズでは行わない。
- 新しい機能追加や大規模最適化（VM/LLVM 側）は対象外。
- 既存の 21.x フェーズのベンチ結果改善は、Phase 25 の直接スコープ外（ただし設計上のゴールには参考としてリンクする）。

## このフェーズ終了時の「完成形」

- Rust / Hakorune の責務分離が文書として明確になり、「新しい箱・数値カーネルは .hako で書く」がプロジェクトの合意として固定されている。
- IntArrayCore / MatI64 の「Rust→Hakorune 移行」手順が、段階ごとのタスクリストとして整理されている。
- System Hakorune サブセットと stdlib ビルド/ロード戦略のたたき台があり、後続フェーズ（例: Phase 22.x / 26.x）でそのまま実装に着手できる状態になっている。
