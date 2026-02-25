# Phase 29y: Observability SSOT（root surface / diagnostics）

Status: Ready (docs-first, post self-host)  
Scope: hidden root を “再発しにくく” するために、root面と観測点を SSOT として固定する。  

## 0. 目的

- weak が観測できる以上、hidden root は “意味論上のバグ” として露見する
- 露見したときに「どこが strong を保持しているか」を追える設計（観測点）を固定する

制約:
- 診断 ON/OFF で意味論を変えない（観測のみ）
- 環境変数を増殖させない（既存 verbose/trace と統合）
- smoke は stdout 比較に寄せず、exit code SSOT を優先

## 1. root surface（root面）の SSOT（カテゴリ）

root面は “場所の列挙” ではなく “カテゴリ契約” として固定する。

最低限のカテゴリ（backendが0でもよい）:
- `locals`: binding（local変数）が保持する strong
- `temps`: 一時値/評価結果が保持する strong（VM regs 等）
- `heap_fields`: オブジェクトの strong-owned fields が保持する strong
- `handles`: host-visible registry/handle table が保持する strong
- `singletons`: runtime の singleton/グローバルが保持する strong（存在する場合）

カテゴリ契約:
- 上記 5 カテゴリを root surface の固定語彙とし、別名や ad-hoc 分類を増やさない。

## 2. 診断API（summaryのみで十分）

相談パケットの目的は “原因が追える” ことであり、詳細な個体列挙は後回しでよい。

推奨の観測:
- root summary: カテゴリ別の strong root 数
- optional: 特定 handle/token の参照数（strong/weak）
- optional GC mode diagnostics（dev/diagnostic only）:
  - stable tag: `[gc/optional:mode] mode=<...> collect_sp=<...> collect_alloc=<...>`
  - default OFF（`NYASH_GC_METRICS=0`）では出力しない
  - metrics ON（`NYASH_GC_METRICS=1`）時のみ 1行出力し、意味論は不変

例（概念）:
- `debug_root_summary() -> { locals: N, temps: N, heap_fields: N, handles: N, singletons: N }`

カテゴリ語彙 inventory（smoke SSOT）:
- `tools/checks/phase29y_observability_categories.txt`

## 3. テスト（観測の固定）

- LLVM/harness は stdout にログが混ざり得るため、スモークは exit code を SSOT とする
- stdout比較が必要な場合は、先に “出力ノイズの抑制” を設計で固定してから（環境変数スパロー禁止）

## 4. 破綻しやすい罠

- 診断トグルが意味論を変える（ON/OFFで挙動が変わる）
- hidden root を “仕様” として放置し、weak_to_strong の意味が揺れる
- root面が実装の場所列挙になり、backend追加時に追従できなくなる
