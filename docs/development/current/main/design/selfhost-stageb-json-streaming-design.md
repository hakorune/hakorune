Status: Active  
Scope: Stage‑B `emit program-json` / `emit mir-json` 出力のメモリ削減と運用安定化  
Related:
- docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- docs/development/current/main/design/ai-handoff-and-debug-contract.md
- docs/development/current/main/design/hako-module-cache-build-ssot.md

# Stage‑B JSON Streaming / JSONL Design (Draft)

## Problem

- Stage‑B が **巨大な AST/Program/MIR を一括で構築・保持**したまま
  最後に JSON へ文字列化するため、ピークメモリがデータ量比例で増える。
- 実行中に **出力ファイルが 0 bytes のまま長時間**になり、途中失敗時の診断性が悪い。
- ローカル環境で Stage1 build が現実的に回らない（G1 gate が運用不能）。

## Goals

- Stage‑B のメモリピークを **“最大モジュール/関数サイズ” 近傍**に近づける。
- 出力が早期に書き出される（0 bytes 停滞をなくす）。
- 既存の意味論・SSOT を維持（AST rewrite 禁止 / analysis‑only view で観測）。
- 既定挙動は維持しつつ、移行可能な設計にする。

## Non‑Goals

- 言語仕様の変更。
- AST の書き換えや評価順序の変更。
- JoinIR/Planner の受理形増加。

## Design Summary (Phased)

### Phase 0: Streamed JSON Writer（最小差分）

目的: まず **“0 bytes のまま”** を解消し、出力を早期に可視化する。

- `emit *-json` で `to_string` / DOM 生成ではなく **`to_writer` で書く**。
- 出力ファイルを開いたら **ヘッダを書いて flush** する（進捗確認用）。
- 既存の JSON v0 形式は維持（互換性優先）。

期待効果:
- “最後までメモリ上に文字列を抱える” を防ぐ。
- 失敗時も途中までの出力が残る（診断性向上）。

Status:
- Implemented for Rust MIR JSON emit (`src/runner/mir_json_emit/mod.rs`) using `to_writer_pretty`.
- Note: 生成対象の JSON を **全保持する構造**は残るため、ピークメモリの主因は未解決。

### Phase 1: JSONL / NDJSON 形式の導入（レコード単位）

目的: **“吐いて捨てる”** を成立させる土台を作る。

- 新しい出力形式を追加（例: `--emit-mir-jsonl`）。
- 1レコード = 1 JSON 行（JSON Lines / NDJSON）。
- ny‑llvmc 側は **1行ずつ読み込み処理**できる構造にする。

最小レコード案:

```
{"kind":"header","format":"mir-jsonl","version":0,"entry":"...","timestamp":"..."}
{"kind":"module","name":"...","id":1}
{"kind":"function","id":123,"sig":{...},"mir":{...}}
{"kind":"end","stats":{"modules":N,"functions":M}}
```

ガイドライン:
- `kind` は固定語彙で SSOT 化。
- レコード順序は決定的にする（同一入力で順序一致）。

### Phase 2: モジュール単位の分割コンパイル

目的: ピークメモリを **“全体” → “単位”** に落とす。

- 依存グラフを解決して **モジュール単位で lower**。
- 1モジュール分を吐いたら **AST/MIR arena を drop** できる設計へ移行。
- ny‑llvmc はモジュール単位で object/bitcode を生成し、最後にリンク。

### Phase 3: 最小 Incremental キャッシュ（任意）

目的: “毎回フル” を避ける。

- `module_hash = hash(source + using‑resolved deps versions)`
- hash が一致する単位は **既存 output を再利用**。
- 実装境界（MIR/object/link の3層）は
  `docs/development/current/main/design/hako-module-cache-build-ssot.md` を SSOT とする。

## Compatibility Plan

- 既存の `--emit-mir-json` は維持（Phase 0 のストリーミング化のみ）。
- JSONL は **新しいフラグ**で導入し、移行期間を設ける。
- ny‑llvmc 側は JSON v0 と JSONL の **両方をサポート**する。

## Acceptance Criteria (Draft)

Phase 0:
- `emit-mir-json` が **開始直後にファイルを非 0 bytes** にする。
- 失敗時でも途中出力が残る（診断距離短縮）。
- 検証用の最小入力は **using/loop を含まない**ものを SSOT 化する
  （例: `apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako`）。

Phase 1:
- JSONL を受理する consumer が 1行ずつ処理できる。
- 大きな入力でも “全体 JSON を保持” しない。

Phase 2:
- ピークメモリが **最大モジュール単位**に近づく。
- モジュール処理後に arena が drop されることを確認できる（計測ログ）。

## Notes

- 受理形の拡張は行わない（BoxCount の対象ではない）。
- 出力形式の SSOT は本ドキュメントで固定し、実装は最小差分で行う。
