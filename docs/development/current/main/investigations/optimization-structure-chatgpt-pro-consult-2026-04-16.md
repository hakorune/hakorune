# Optimization Structure Consult (for ChatGPT Pro)

Status: Archived consult packet
Date: 2026-04-16
Scope: Hakorune の最適化構造が妥当か、`.hako -> MIR -> Rust kernel -> LLVM` の責務分割と移行順を外部レビューにかけるための相談パケット
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/current-optimization-mechanisms-ssot.md
- docs/development/current/main/design/optimization-layer-roadmap-ssot.md
- docs/development/current/main/design/semantic-optimization-authority-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/optimization-tag-flow-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
- docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md

## Post-Consult Reading

This file preserves the question packet that was sent out before the taxonomy refresh.

- authority order stayed the same: `.hako -> MIR -> Rust kernel -> LLVM`
- the adopted correction was taxonomy, not direction
- current docs now read optimization rows as `substrate / producer / exporter / consumer`
- `LLVM attrs`, `.hako -> ny-llvmc(boundary) -> C ABI`, `ThinLTO`, and `PGO` should now be read as exporter/consumer rows, not authority rows
- `thin-entry` and `handle ABI -> value ABI` now read through `call surface substrate`
- `birth placement / placement-effect` now reads through `value representation / materialization / scalarization substrate`
- `float optimization` is now separated from the early `numeric loop / SIMD` reading

Use these SSOTs for the current picture:

- [optimization-layer-roadmap-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-layer-roadmap-ssot.md)
- [current-optimization-mechanisms-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/current-optimization-mechanisms-ssot.md)

## ChatGPT Pro へ投げる文章

Hakorune の最適化アーキテクチャを辛口でレビューしてください。

### 前提

最適化の authority order は次で考えています。

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust kernel / executor`
4. `LLVM generic optimization / codegen`

狙いは、`.hako` で意味と policy を決め、MIR で canonical contract を持ち、Rust はハードコード無しの薄い kernel に徹し、LLVM には generic optimization だけをさせることです。

### 当時の pre-consult row wording

- `semantic simplification bundle`
- `escape analysis`
- `birth placement / placement-effect`
- `memory-effect layer`
- `thin-entry / call optimization`
- `handle ABI -> value ABI`
- `.hako -> ny-llvmc(boundary) -> C ABI`
- `LLVM attrs`
- `numeric loop / SIMD`
- `closure split`
- `IPO / ThinLTO / PGO`

今は owner seam / proof seam / scaffold はかなり入っていますが、actual widening は narrow cut が多いです。

### 制約

- Rust 側に string/box 専用ハードコードを増やしたくない
- `.hako` 側 workaround ではなく、compiler/MIR/kernel 側の構造で直したい
- AST rewrite は禁止
- fallback より fail-fast を優先する
- `@rune Hint/Contract/IntrinsicCandidate` は parse/noop のまま
- optimize lane では `same artifact`, `3 runs`, `asm`, `whole-kilo` で judge する
- string 専用 MIR dialect は増やしたくない

### いまの strong / weak

強い点:
- owner seams が explicit
- perf ladder / asm / same-artifact judge がある
- string, user-box, numeric-loop, closure/IPO で narrow backend consumers は実在する

弱い点:
- backend-active な高位 hint 消費がない
- generic `noalias` / broader LLVM attrs feed がない
- full value-ABI coverage がない
- thin-entry に universal consumer がない
- float-specific widening は backlog

### 相談したいこと

1. この authority order と row 分割は妥当ですか。
2. `.hako -> MIR -> Rust kernel` の導線は、このままで間違えない構造ですか。
3. Rust kernel に残すべき generic minimum は何ですか。
4. `birth placement`, `alias reduction`, `escape`, `memory-effect`, `thin-entry`, `value ABI` は 1 つの generic substrate に寄せるべきですか。
5. `@rune Hint/Contract/IntrinsicCandidate` を backend-active に昇格させる前提は何ですか。
6. `thin-entry` と `handle ABI -> value ABI` の先後はどうあるべきですか。
7. `.hako -> ny-llvmc(boundary) -> C ABI` corridor を generic に広げる境界はどこですか。
8. `LLVM attrs` は今の `readonly` / `nocapture` から、`noalias` / `readnone` / TBAA へどう進めるべきですか。
9. `numeric loop / SIMD / float` の row 分けは妥当ですか。
10. `closure split` と `IPO / ThinLTO / PGO` の順序は妥当ですか。
11. いま欠けている重要な row はありますか。
12. 構造的に誤っているところがあれば遠慮なく壊してください。

### 期待する回答形式

- 良い点 / 危ない点 / 変えるべき点を先に列挙
- その後に推奨 end-state architecture を 1 枚で提示
- 最後に段階移行の順序を 5〜8 step で提示
- 可能なら `must keep in Rust`, `must move to MIR`, `must stay language-level`, `leave to LLVM` の4区分で整理

## Pre-Consult Snapshot

Row summary:

- `semantic simplification bundle`: landed mechanism
- `escape analysis`: landed mechanism
- `birth placement / placement-effect`: owner seam
- `memory-effect layer`: landed mechanism (narrow)
- `dead store / load-store forwarding / alias軽減`: landed mechanism (narrow)
- `thin-entry / call optimization`: owner seam
- `handle ABI -> value ABI`: owner seam
- `call を C レベルに寄せる`: landed mechanism (narrow)
- `LLVM attrs`: landed mechanism (narrow)
- `language-level hints / metadata plumbing`: scaffold
- `numeric loop / SIMD / induction / reduction / vectorization`: owner seam + landed mechanism (narrow)
- `float optimization`: backlog
- `closure split / capture classification / env scalarization / closure thin-entry`: owner seam + landed mechanism (narrow)
- `IPO / ThinLTO / PGO`: owner seam + scaffold

Parent reading:

- row order SSOT: `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- current mechanism map: `docs/development/current/main/design/current-optimization-mechanisms-ssot.md`
- authority order: `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
- measurement/judge: `docs/development/current/main/design/perf-optimization-method-ssot.md`

## Strong critique requested

特に次の指摘を歓迎します。

- その row 分割は bad
- その責務 split は hardcode を呼ぶ
- `.hako` に上げるべきものを Rust に置いている
- MIR で canonicalize せず metadata に逃がしている
- rollout 順が逆
- narrow cut の accumulation に architectural debt がある

## Notes

- 現在の docs snapshot は commit `701edbfb6`
- 目的は「褒めてもらうこと」ではなく、「構造の誤りや弱さを早めに露出すること」
