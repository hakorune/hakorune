---
Status: Provisional SSOT
Decision: accepted-for-phased-rollout
Date: 2026-04-19
Scope: string lane を perf helper 名ではなく language-clean な値モデルで固定し、`String` の意味・publish 境界・birth sink・future storage specialization を 1 本の導線に分ける。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md
---

# String Semantic Value And Publication Boundary SSOT

## Goal

- `String` を helper 実装や runtime carrier ではなく、言語上の immutable value として正本化する。
- `publish` を boundary effect、`freeze.str` を唯一の birth sink として分離する。
- `TextLane` を future storage specialization として位置づけ、意味論や public ABI の truth にしない。
- container lane-host generalization を「Array/Map の内部 residence だけに及ぶ後続境界」として固定し、Array/Map 自体の identity semantics は再定義しない。
- phase-137x の rollout を「きれいな値モデルをどの順で runtime に降ろすか」の話に戻す。

この文書は runtime-wide value/object boundary の first proving ground だよ。
runtime-wide の親読みは
`docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`、
generalization taskboard は
`docs/development/current/main/phases/phase-289x/README.md` に置く。

String は proving ground であって、runtime-wide vocabulary の唯一の源ではない。
future typed lanes は string-specific carrier 名を増殖させず、
`lifecycle-typed-value-language-ssot.md` の `Ref / Owned / Cell / Stable`
と boundary vocabulary を再利用する。

## Core Lock

この lane の北極星は次で固定する。

```text
Language meaning
  String value

MIR contract
  proof_region
  publication_boundary
  borrow.text_from_obj
  publish.text(reason, repr)
  same-corridor unpublished outcome

Runtime-private execution world
  TextRef
  TextPlan
  OwnedText
  TextCell (sink/residence only)
  read-side alias lane

Cold boundary
  freeze.str birth sink
  publish.text(reason, repr)

Public world
  StringHandle / ArrayHandle / Box<dyn NyashBox>
```

読み方は単純だよ。

- `String` は language meaning では値
- handle / box / registry は boundary representation
- same corridor の内部では text を object world の steady-state carrier にしない
- string birth は `freeze.str` だけが担当する

## Generalization Boundary

この SSOT は string-first で進めるけれど、将来の container lane-host への一般化は次だけを許す。

- Array/Map は language meaning でも public surface でも identity container のまま
- lane-host 化の対象は Array element / Map key/value の internal residence だけ
- `publish` / `promote` は boundary effect のまま保ち、container helper が legality owner にならない
- `freeze.str` は string だけの birth sink であり、container lane-host generalization が第二の string birth sink を増やしてはならない
- public handle ABI は widening しない

## Layer Ownership

### 1. Language meaning (`.hako` / docs)

ここが持つもの:

- `String` の immutable value semantics
- `concat`, `substring`, `len`, `indexof` などの meaning
- どこで value が external/public world へ escape するか

ここが持たないもの:

- handle class
- registry / TLS
- stable object cache
- `TextLane` storage detail

### 2. Canonical MIR / lowering contract

ここが持つもの:

- `proof_region`
- `publication_boundary`
- `borrow.text_from_obj` provenance / borrow contract
- `publish.text(reason, repr)` boundary effect
- `same-corridor unpublished outcome`
- sink capability / stable identity demand

ここが持たないもの:

- runtime route re-recognition
- helper-name allowlist
- public raw string ABI
- runtime-side provenance re-inference

Lock:

- phase-137x では string-specific public MIR dialect を増やしすぎない
- `text.ref` / `text.plan` / `text.owned` はまず contract vocabulary / recipe metadata として読む

### 3. Runtime-private carrier

ここが持つもの:

- `VerifiedTextSource`
- `TextPlan`
- `OwnedBytes`
- `KernelTextSlot` as transport adapter / sink seed
- read-side alias lane:
  - `TextReadOnly`
  - `EncodedAlias`
  - `StableObject`

Semantic-vs-adapter lock:

- future semantic carriers are `TextRef`, `TextPlan`, and `OwnedText`
- `TextCell` is future sink/residence only; it is not a corridor value
- `BorrowedHandleBox` belongs to boundary/cache behavior, not semantic `Ref`
- `KernelTextSlot` must not become the public or long-term `TextCell`
- `StringViewBox` is an object-world view, not internal substring carrier

ここが持たないもの:

- 言語意味の決定
- boundary legality の再判定
- public ABI の拡張

### 4. Cold boundary

ここが持つもの:

- `publish` effect
- `freeze.str` birth sink
- `publish.text(reason, repr)` adapter
- `objectize`
- `issue_fresh_handle`

ここが持たないもの:

- producer meaning
- sink storage policy
- helper ごとの special-case truth

Glossary lock:

- `publish` is the boundary effect selected by MIR/lowering demand.
- `freeze.str` is the string birth sink used at that boundary.
- `publish.text` is the first explicit string bridge on this lane.
- `objectize` / `handle issue` are mechanics below that boundary.
- typed-lane docs must not reinterpret `freeze.str` as a generic publish effect.

## Publish vs `freeze.str`

この 2 つは似て見えるけれど、同じ責務にしてはいけない。

- `publish`
  - MIR/lowering が「ここから public/object world に出る」と決める boundary effect
  - legality owner
  - v1 は `publish.text(reason, repr)` として string lane にだけ導入する
  - `reason` と `repr` は分ける:
    - `reason`: なぜ publish が必要か
    - `repr`: public world にどう出すか
  - `repr` は request であって guarantee ではない
    - provenance / mutability / lifetime の legality が満たせない場合、MIR/lowering は conservative に downgrade してよい
    - runtime は `repr` legality を再推測しない
- `freeze.str`
  - その boundary で実際に birth を行う sink
  - retained string birth / reuse の mechanical owner
  - public world が必要なら cold objectize / handle issue へ handoff する
  - `TextPlan` は直接 `publish.text` に渡さない。copy/materialize が必要なら先に `freeze.str` を通して `OwnedText` へ寄せる

### `repr` Request vs Guarantee

`publish.text(reason, repr)` の `repr` は「こう公開したい」という request であって、
常にそのまま public contract になる guarantee ではない。

- `reason`
  - boundary が必要な理由
  - 例: `stable_object_demand`, `explicit_api_replay`
- `repr`
  - 希望する公開表現
  - 例: `StableOwned`, `StableView`
- legality
  - その `repr` が本当に許されるかを決める third axis
  - provenance / mutability / lifetime / borrow-scope で判定する

Lock:

- legality owner は MIR/lowering であり、runtime ではない
- legality が満たせない `repr` request は conservative に downgrade してよい
- runtime-private helper / site name / operand shape から legality を再推測しない

### `StableView` Legality Lock

`StringViewBox` は internal carrier ではなく public replay object に限定する。
ただし `StableView` を public に出してよい条件は先に固定する。

`StableView` が合法なのは、少なくとも次のどれかを満たす provenance に限る。

- literal / already-published stable string
- immutable host-owned bytes
- pinned provenance が証明され、公開 lifetime の間に mutation されない residence

次の形は `StableView` 不可として扱う。

- mutable `TextCell` / future mutable residence
- same-slot update で再書き換えされうる residence
- borrow-scope や lifetime continuity を verifier が証明できない source

Illegal `StableView` request は boundary の前に downgrade する。

- preferred fallback: `StableOwned`
- copy/materialize が必要なら `freeze.str -> OwnedText -> publish.text(...)`
- `publish.text` 自体を birth sink に戻してはならない

## Bridge Lock

この lane の bridge truth は 2 本で固定する。

- `borrow.text_from_obj`
  - object world から text world へ戻るときの provenance / proof 入口
  - runtime helper ではなく MIR/lowering 側が責務を持つ
- `publish.text(reason, repr)`
  - text world から object world へ出る explicit boundary effect
  - runtime は実行するだけで、`need_stable_object` を再推測しない

v1 lock:

- `publish.text` だけ先に導入する
- `publish.any` は deferred
- `borrow.text_from_obj` は最初は opcode でも metadata でもよいが、責務は MIR/lowering に置く

禁止:

- `publish` を第二の birth sink にすること
- `freeze.str` を publication policy owner にすること
- runtime が `need_stable_object` を推測して勝手に publish すること

## Array Corridor Contract

write/read は同じ text corridor の契約として読む。

### Write side

- sink は `VerifiedTextSource` / `TextPlan` / `OwnedBytes` / current `KernelTextSlot` transport を consume できる
- phase-1 canonical sink transport is `KernelTextSlot`; future residence is `TextCell`
- same corridor の store では eager `StringBox -> handle` を禁止する

### Read side

- `array.get` の common path は `TextReadOnly` or `EncodedAlias`
- `StableObject` は identity demand や external boundary のときだけ使う
- stable objectize は cache-backed and cold
- per-read fresh promotion は rejected shape

### Invalidation rule

- mutation / drop-epoch change / proof loss が起きたら alias continuity は失効してよい
- ただし失効は runtime の silent policy widening ではなく、既存 conservative fallback に限定する

## Migration Order

### Phase 0. Semantic lock

- docs で `String = value`, `publish = boundary effect`, `freeze.str = only birth sink` を固定する
- `TextLane` は future storage specialization として扱う

### Phase 1. Producer outcome -> canonical sink

- `VerifiedTextSource -> TextPlan -> OwnedBytes -> KernelTextSlot transport`
- producer が public handle を返さなくても corridor が閉じることを証明する
- `borrow.text_from_obj` provenance owner は MIR/lowering に置き、runtime helper が borrow truth を再判定しない

### Phase 2. Cold publish effect

- `publish.text(reason, repr)` を explicit boundary event に寄せる
- `reason` と `repr` を分離したまま string-only v1 を固定する
- `repr` は request-shaped のまま扱い、legality が満たせない `StableView` は downgrade する
- producer/helper から objectize / handle issue を退避する
- `publish.any` はこの phase では始めない

### Phase 2.5. Read-side alias lane

- `TextReadOnly` / `EncodedAlias` / `StableObject`
- read path を cheap alias continuity へ寄せる
- stable objectize は one-shot cache-backed cold path に保つ

### Phase 3. Future `TextLane` storage

- array/map internal residence specialization only
- container semantics や public handle ABI はここで変更しない
- semantic truth ではなく runtime-private storage truth

### Phase 4. MIR legality / verifier

- publication boundary だけでなく、provenance / borrow-scope / freeze→publish separation を verifier-visible にする
- `borrow.text_from_obj` provenance と `publish.text(reason, repr)` boundary を verifier-visible にする
- verifier は最低でも次を区別して見る
  - `publication_boundary`: どこで public/object world に出るか
  - `publish_reason`: なぜ publish が必要か
  - `publish_repr_policy`: どの公開表現を request しているか
  - `borrow/provenance`: その `repr` が legal か
  - `freeze.str -> publish.text`: birth sink と boundary effect の段階分離
- verifier は次を reject する
  - partial `publish.text` metadata
  - unsupported `repr` legality
  - `freeze.str` と `publish.text` の責務混線
  - runtime fallback に legality を押し戻す shape
- runtime private carrier が proven になってから legality を持ち上げる

## Forbidden Moves

- `TextLane` を semantics や public MIR truth として先に立てる
- container lane-host を Array/Map semantic rewrite として扱う
- helper 名で publication legality を持つ
- registry-backed transient carrier を steady-state 化する
- read path で stable object を毎回 fresh に作る
- public ABI を phase-137x で widening する

## Acceptance

- `String` を handle/object と同一視しない
- `publish` と `freeze.str` が二重SSOTにならない
- `TextLane` を入れなくても phase 1/2.5 の contract が読める
- array read/write が同じ text corridor として説明できる
- runtime は semantic owner ではなく executor として読める
