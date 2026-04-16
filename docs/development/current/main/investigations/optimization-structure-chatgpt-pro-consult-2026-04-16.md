# Optimization Structure Consult (for ChatGPT Pro)

Status: Draft (for external consultation)
Date: 2026-04-16
Scope: Hakorune の最適化方法の構造が妥当か、`.hako -> MIR -> Rust kernel -> LLVM` の責務分割と移行順を外部レビューにかけるための相談パケット
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/current-optimization-mechanisms-ssot.md
- docs/development/current/main/design/optimization-layer-roadmap-ssot.md
- docs/development/current/main/design/semantic-optimization-authority-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/optimization-tag-flow-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
- docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
- docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md
- docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md

## 1. ChatGPT Pro へ投げる文章

以下の条件で、Hakorune の最適化アーキテクチャをかなり厳しめにレビューしてください。

### コンテキスト

Hakorune は、最適化を次の authority order で固定しようとしています。

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust kernel / executor`
4. `LLVM generic optimization / codegen`

狙いは次です。

- `.hako` 側で policy と意味を決める
- MIR では canonical contract と metadata を持つ
- Rust 側はハードコードを増やさず、薄い kernel / substrate / executor に徹する
- LLVM には generic optimization だけをさせる

今の repo では、最適化機構の current map を SSOT 化しました。

- semantic simplification bundle
- escape analysis
- birth placement / placement-effect
- memory-effect layer
- thin-entry / call optimization
- handle ABI -> value ABI
- `.hako -> ny-llvmc(boundary) -> C ABI` corridor
- LLVM attrs (`readonly`, `nocapture`; `noalias` はまだ)
- numeric loop / SIMD
- closure split
- IPO / ThinLTO / PGO

ただし現状は、owner seam / proof seam / scaffold はかなり入っている一方で、actual behavior-changing widening は narrow cut が多いです。

### 制約 / 方針

- Rust 側に string/box 専用のハードコードを増やしたくない
- `.hako` 側 workaround ではなく、compiler/MIR/kernel 側の構造で直したい
- AST rewrite は禁止。analysis-only view で観測する
- fallback より fail-fast を優先する
- `@rune Hint/Contract/IntrinsicCandidate` は現在 parse/noop で、backend-active にはしていない
- optimize lane では `same artifact`, `3 runs`, `asm`, `whole-kilo` で judge する
- string 専用 MIR dialect は増やしたくない
- 「LLVM に頑張らせる」のではなく、「LLVM が generic optimization できる contract を MIR で揃える」方針

### いまの strong points

- owner seams はかなり explicit
- phase closeout と proof docs が多い
- perf ladder / asm / same-artifact judge が固定されている
- string, user-box, numeric-loop, closure/IPO で narrow backend consumers は実在する

### いまの weak points

- backend-active な高位 hint consumption はまだ無い
- generic `noalias` / broader LLVM attrs feed はまだ無い
- full value-ABI coverage はまだ無い
- thin-entry は universal backend consumer まで行っていない
- float-specific widening は backlog

### 相談したいこと

1. この authority order と row 分割は妥当ですか？
   - `.hako owner / policy`
   - `MIR canonical contract`
   - `Rust kernel / executor`
   - `LLVM generic optimization / codegen`
   - `semantic simplification`
   - `memory-effect`
   - `thin-entry`
   - `value ABI`
   - `LLVM attrs`
   - `numeric loop / SIMD`
   - `closure split`
   - `IPO / PGO / ThinLTO`

2. `.hako -> MIR -> Rust kernel` の “間違えない導線” は、このままでよいですか？
   特に、最適化 hint / contract / recipe / proof / metadata を、どの層で canonicalize して、どの層で consume すべきでしょうか。

3. Rust kernel を generic に保つには、何を Rust に残し、何を `.hako` / MIR に上げるべきですか？
   逆に、今の設計で Rust 側に残しすぎているもの、あるいは `.hako` に上げすぎているものがあれば厳しく指摘してください。

4. `birth placement`, `alias reduction`, `escape`, `memory-effect`, `thin-entry`, `value ABI` は、将来的に 1 つの generic substrate language に寄せるべきですか？
   それとも今のように row を分けたまま、MIR metadata だけを共有する形がよいでしょうか。

5. `@rune Hint/Contract/IntrinsicCandidate` のような language-level metadata は、今の parse/noop 戦略のままで正しいですか？
   backend-active に昇格させるとしたら、どの前提が揃ってからですか。

6. `thin-entry` と `handle ABI -> value ABI` の関係はどう整理するのがよいですか？
   - thin-entry を先に actual consumer 化するべきか
   - value ABI を先に widen するべきか
   - それとも両者を共通 manifest / contract に寄せるべきか

7. `.hako -> ny-llvmc(boundary) -> C ABI` corridor を将来もっと generic に広げるとき、何を invariant にして、何を lane-local に留めるべきですか？
   string lane で当たった pattern を generic framework 化する境界も教えてください。

8. `LLVM attrs` は今 `readonly` / `nocapture` だけ narrow に付けています。
   `noalias`, `readnone`, TBAA, dereferenceable などは、どの層の契約が揃ってから足すべきですか。

9. `numeric loop / SIMD / float` は、今は proof-first / hint-first です。
   float を `numeric loop / SIMD` の subtheme に留めるのが良いか、それとも別 row に分けるべきかを教えてください。

10. `closure split` と `IPO / ThinLTO / PGO` の関係は、今の順序で正しいですか？
    closure env scalarization と thin-entry specialization が narrow なうちに IPO scaffold を先に入れたのは妥当でしょうか。

11. いまの row で欠けている重要なものはありますか？
    例:
    - loop canonicalization
    - MemorySSA / alias framework
    - effect lattice
    - inline-cost / devirtualization policy
    - verification contract row

12. もしこの構造に概念的な誤りがあるなら、遠慮なく壊してください。
    特に「その row 分割は悪い」「その層に置くと必ずハードコード化する」「その rollout 順は逆」みたいな critique を歓迎します。

### 期待する回答形式

- 「この構造で良い点 / 危ない点 / 明確に変えるべき点」を先に列挙
- その後に、推奨する end-state architecture を 1 枚で提示
- 最後に、段階移行の順序を 5〜8 step くらいで提示
- 可能なら、`must keep in Rust`, `must move to MIR`, `must stay language-level`, `leave to LLVM` の4区分で整理してください

## 2. 現在の最適化 row の読み

### Row summary

- `semantic simplification bundle`
  - landed mechanism
- `escape analysis`
  - landed mechanism
- `birth placement / placement-effect`
  - owner seam
- `memory-effect layer`
  - landed mechanism (narrow)
- `dead store / load-store forwarding / alias軽減`
  - landed mechanism (narrow)
- `thin-entry / call optimization`
  - owner seam
- `handle ABI -> value ABI`
  - owner seam
- `call を C レベルに寄せる`
  - landed mechanism (narrow)
- `LLVM attrs`
  - landed mechanism (narrow)
- `language-level hints / metadata plumbing`
  - scaffold
- `numeric loop / SIMD / induction / reduction / vectorization`
  - owner seam + landed mechanism (narrow)
- `float optimization`
  - backlog
- `closure split / capture classification / env scalarization / closure thin-entry`
  - owner seam + landed mechanism (narrow)
- `IPO / ThinLTO / PGO`
  - owner seam + scaffold

### Parent reading

- row order SSOT:
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
- current mechanism map:
  - `docs/development/current/main/design/current-optimization-mechanisms-ssot.md`
- authority order:
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
- measurement/judge:
  - `docs/development/current/main/design/perf-optimization-method-ssot.md`

## 3. 追加で強めに聞きたい質問

1. `current-optimization-mechanisms-ssot.md` の status legend は妥当ですか？
   - `landed mechanism`
   - `landed mechanism (narrow)`
   - `owner seam`
   - `scaffold`
   - `backlog`

2. `optimization-layer-roadmap-ssot.md` の row order は、実装順としても正しいですか？
   特にこの並び:
   - generic placement / effect
   - agg_local scalarization
   - thin-entry actual consumer switch
   - semantic simplification bundle
   - memory-effect layer
   - escape / barrier -> LLVM attrs
   - numeric loop / SIMD
   - closure split
   - IPO / build-time optimization

3. `alias軽減` を独立 row にせず、複数 row に分散したのは正しいですか？
   それとも alias を中心 row にしたほうが設計が締まりますか。

4. `value ABI` と `call-to-C corridor` を optimization row として持つのは正しいですか？
   それともこれは optimization ではなく “execution substrate” として別管理すべきでしょうか。

5. `.hako` の optimization metadata を MIR に下ろすとき、
   - canonical op
   - recipe
   - proof
   - candidate
   - manifest row
   - attr request
   のどれを第一級にするのがよいですか。

6. “Rust kernel に残す generic minimum” を、もしあなたならどう定義しますか。
   具体的に API/Box/trait/metadata boundary の形まで落として提案してください。

## 4. 期待する厳しい critique

次の kinds の critique を歓迎します。

- その最適化 row の切り方は bad
- その responsibility split は hardcode を呼ぶ
- `.hako` に上げるべきものを Rust に置いている
- MIR で canonicalize せず metadata に逃がしているせいで LLVM に伝わらない
- reverse dependency があるので rollout 順が危ない
- “narrow cut が多い” こと自体は正しいが、その accumulation に architectural debt がある

## 5. 補足

- 現在の docs snapshot は commit `701edbfb6`
- 相談の目的は「いまの仕組みを褒めてもらう」ことではなく、「最適化構造の誤りや弱さを早めに露出する」こと
- かなり辛口で構わない
