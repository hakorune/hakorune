# Phase 29y: ABI SSOT（lifecycle / RC / weak）

Status: Ready (docs-first, post self-host)  
Scope: “脱Rustランタイム（NyRT/.hako）” を進める前提で、lifecycle/RC/weak の境界を ABI として固定する。  

この文書は「実装の現状」を説明するのではなく、**将来の置換（NyRT/.hako化）に耐える契約**を先に固定する。

## 0. 目的（SSOT）

- backend（VM/LLVM/wasm/others）が “同じ意味” を実装できる最小 ABI を定義する
- “言語意味論” と “実装都合（VMのregs等）” を切り離し、hidden root を再発しにくくする
- cycle collector（GC）の ON/OFF で ABI 意味を変えない（差分は回収タイミングのみ）

## 1. 前提（言語意味論）

ここでの weak は観測可能:
- `weak_to_strong()` が成功/失敗を返す（失敗は `null`）
- したがって「SSA last-use = 寿命」を意味論にしてはいけない

## 2. ハンドル/値の表現（最低限）

### 2.1 Box handle

- `BoxHandle` は runtime 管理の参照（opaque）とする
- 返値/引数は「0 = null」を許す（`null` を表す）

### 2.2 Weak handle（token identity）

weak の等価性（identity）は backend 差が出にくい token を SSOT として固定する:
- `alloc_id + generation` を含む token（概念）
- ログ表示も token を使う（例: `WeakRef(#1234:g7)`）

比較（`==`）の扱い:
- `WeakRef` の `==` は token 同一性比較（Alive/Dead/Freed に依存しない）

## 3. 関数 ABI（最小契約）

推奨 SSOT:
- **args borrowed / return owned**

契約:
- 引数は borrowed（callee は retain/release しない）
- 戻り値は owned（caller が release 責務を持つ）
- borrowed を保存/捕獲/フィールド格納/返す等の “escape” を行う場合のみ、compile 側が acquire（retain）を挿入する

## 4. NyRT lifecycle ABI（最小セット案）

ここでは “名前” より契約を SSOT とする（既存の実装名が異なる場合は shim で吸収）。

### 4.1 strong

- `retain(h: BoxHandle) -> BoxHandle`
  - `h == 0` のときは `0` を返す（no-op）
- `release(h: BoxHandle) -> void`
  - `h == 0` のときは no-op
  - 物理解放のタイミングは runtime 実装に委譲（言語意味論では規定しない）

### 4.2 weak

- `weak_new(h: BoxHandle) -> WeakHandle`
  - `h == 0` はエラー（または `0` weak を禁止）
- `weak_drop(w: WeakHandle) -> void`
- `weak_to_strong(w: WeakHandle) -> BoxHandle`
  - 成功: Alive のときだけ non-null を返す
  - 失敗: Dead/Freed は `0`（null）を返す

## 5. 非目標（このABIで扱わない）

- finalizer の自動実行（RC/GC が勝手に `fini()` を呼ばない）
- GCアルゴリズム（cycle回収等）の規定

## 6. 関連（参考リンク）

- ABIの既存ドキュメント（参考）:
  - `docs/reference/abi/nyrt_c_abi_v0.md`
  - `docs/reference/abi/nyrt_host_surface_v0.md`
  - `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
  - `docs/reference/abi/NYASH_ABI_MIN_CORE.md`
- 言語意味論 SSOT（参照のみ）:
  - `docs/reference/language/lifecycle.md`
