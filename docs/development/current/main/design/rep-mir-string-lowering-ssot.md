---
Status: SSOT
Decision: provisional
Date: 2026-03-19
Scope: `substring -> concat3 -> length` hot chain を AOT backend 内でだけ boxless/native value に寄せる temporary pilot の境界と手順を固定する
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/rep-mir-string-birth-map-inventory.md
- docs/development/current/main/design/transient-text-pieces-ssot.md
- docs/development/current/main/design/string-transient-lifecycle-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- src/mir/
- crates/nyash_kernel/src/exports/string.rs
- crates/nyash_kernel/src/exports/string_view.rs
---

# Shadow RepMIR String Lowering SSOT

## Goal

`Everything is Box` を source semantics と boundary protocol に残しつつ、AOT backend の内部だけは string hot chain を SSA/native value で流せるようにする。

この文書の目的は 2 つだけだよ。

1. `RepMIR` を Rust の新しい permanent owner にしない。
2. `substring_concat` 系の pilot を、あとで `.hako` authority へ戻せる薄い手順で固定する。

この wave で採る形は **Shadow RepMIR** だよ。

- MIR / JoinIR は正本のまま残す
- AOT consumer が一時的にだけ `RepKind` を読む
- backend lowering が終わったら、その shadow 表現は捨てる

## Core Position

この wave では次を固定する。

- source semantics:
  - Everything is Box
- VM / plugin / FFI / ABI boundary:
  - Everything is Box
- AOT backend internal lowering:
  - not everything is Box

つまり、**Box は observable / retained / substrate-visible boundary でだけ生まれる**。
内部 SSA / register / stack まで Box にするのはやめる方向だよ。

## Lowering Contract Ownership

Shadow RepMIR は独立した意味層ではない。
この文書で固定する contract を、AOT consumer が一時的に mirror するだけだよ。

### owner

- `.hako` / docs:
  - string algorithm control structure
  - escape/birth rule
  - intrinsic contract
- Rust/C substrate:
  - raw byte scan / compare / copy
  - allocation / flatten
  - `freeze.str` の leaf 実装

### non-owner

- Shadow RepMIR 自体
- private Rust enum / struct
- runtime helper ABI

要するに、**RepMIR は `.hako` owner の lowering contract を映す影であって、新しい runtime semantics ではない**。

## Rust Growth Lock

この pilot は Rust に新しい意味 owner を作るためのものではない。

### owner はここ

- `.hako` / docs / SSOT
  - escape/birth rule
  - `freeze/thaw` の意味
  - `RepKind` の意味
  - non-goals

### Rust はここまで

- AOT backend-local consumer
- temporary lowering substrate
- pilot 実装
- perf proof
- pass-local temporary representation

### Rust に持たせないもの

- generic language semantics
- VM-visible new runtime contract
- plugin-visible token contract
- backend ごとに違う意味ルール
- `.hako` に戻せない private truth
- new permanent runtime layer
- `NyashBox` / `host_handles` / plugin ABI に混ざる transient token

要するに、**Rust は temporary implementation lane であって SSOT ではない**。

## Placement

いちばんよい placement は、runtime 末端ではなく **AOT consumer 側**だよ。

### best

- `ny-llvmc(boundary)` 側の AOT lowering module
- backend-native value を直接組み立てる narrow consumer

### acceptable

- boundary codegen に隣接した thin shim

### avoid

- `crates/nyash_kernel/src/exports/string.rs` を新しい permanent owner にすること
- `crates/nyash_kernel/src/exports/string_view.rs` に transient semantics を積むこと
- runtime helper ABI に `substring_t` / `concat_t` / `len_t` を常設すること

## Representation Layer

最初の pilot で扱う候補は次だよ。

```text
RepKind =
  Boxed
  I64
  Bool
  StrView
  StrOwned
  StrPieces
```

ただし、初手で全部を有効化しない。

### pilot で優先するもの

- `StrView`
- `StrPieces`
- `I64`
- `Boxed`

### 後回しでよいもの

- `Bool`
- full generic scalar propagation
- VM/shared runtime まで広げる token 化

### minimal transient carriers

最初の pilot で十分なのは、次の narrow carrier だけだよ。

```text
TStr =
  RootView  { base_handle, base_ptr, start, len }
  Pieces3   { p0, lit, p2, total_len }
  OwnedTmp  { ptr, len, cap }
```

ここで大事なのは:

- `Pieces3` を heap object にしない
- public runtime object にしない
- pass-local / stack-local / backend-local struct に留める

## Minimal Ops

pilot は最小の命令表に留める。

```text
thaw.str
str.slice
str.cat3
str.len
freeze.str
```

この 5 つで `kilo_micro_substring_concat` の loop body を読めれば十分だよ。

## Canonical Naming

命名は 2 層に固定する。

### canonical contract names

```text
thaw.str
str.slice
str.concat3
str.len
str.find_byte_from
str.eq_at
freeze.str
```

### `.hako` kernel-side spellings

```text
__str.thaw_box
__str.slice
__str.concat3
__str.len
__str.find_byte_from
__str.eq_at
__str.freeze
```

`.hako` 側は `__str.*` を使ってよい。
ただし docs/SSOT では contract 名を `str.*` / `freeze.str` に寄せて、backend-local 実装差分を増やさない。

## Intrinsic Contract

low-level string kernel を `.hako` に寄せるときの contract は、次の分割に固定する。

### `.hako` owner

- `find_index`
- `contains`
- `starts_with`
- `ends_with`
- `split_once_index`

これは algorithm/control structure の owner だよ。

### substrate leaves

- `str.find_byte_from`
- `str.eq_at`
- raw byte copy / flatten
- flat string allocation
- `freeze.str`

これは raw byte/memory/freeze leaf であり、当面 Rust/C substrate に残す。

## Pilot Scope Lock

### in scope

- AOT backend only
- `kilo_micro_substring_concat`
- `substring -> concat3 -> length` chain
- loop 内の local SSA value
- `text = out.substring(...)` は current pilot では first escape boundary のまま

### out of scope

- VM
- plugin ABI
- FFI token exposure
- generic container
- `box_id`
- GC / finalization
- `BoxBase::new` semantics
- loop-carried `text` を transient のまま backedge 越しに運ぶ endgame widening

## Birth Rule

`freeze.str` は次の地点にしか置かない。

- identity 観測
- clone/share/materialize
- generic container 格納
- FFI / plugin / GC-visible root
- generic `NyashBox` 動的 call

この pilot では、`substring_concat` の loop 内 local 値は、可能な限り `freeze` しない。
ただし current pilot では、loop-carried `text` は first escape boundary のままにして widening を抑える。

## Layering

この pilot は次の 3 段で切る。

1. `RepInference`
   - narrow pilot only
   - `substring_concat` chain を `Boxed/StrView/StrPieces/I64` に分類
2. `BirthPlacement`
   - `freeze` / `thaw` を escape point にだけ置く
3. `StringFusion`
   - helper ABI call ではなく backend-native string op に落とす

ただし current wave では、まず docs/shape を固定する。
実装は narrow pilot 1 本に留める。

現時点では full generic pass を作るより、**shadow lowering** として exact chain だけ読む方を優先する。

## Handoff Rule

`.hako` へ戻しにくくしないため、次を必須にする。

1. Rust pass 名・命令名・境界名は docs に先に書く
2. private Rust enum のみを truth にしない
3. pilot scope を `substring_concat` 以外へ自動で広げない
4. `.hako` 側が同じ `RepKind/freeze/thaw` を表現できる構造を保つ
5. backend-local helper は transport であって semantics owner ではないと明記する
6. runtime helper ABI や `NyashBox` trait に transient representation を混ぜない
7. low-level string algorithm control structure は `.hako` kernel library へ戻す前提で naming と boundary を固定する

## Birth Map Requirement

実装前に、最低限この birth map を切ることを必須にする。

- current input representation
- current birth site
- handle 化の有無
- `BoxBase::new` / `Registry::alloc` まで到達するか
- future shadow op で何に置き換えるか
- owner が docs か Rust substrate か

## Fixed Order

1. docs-first
   - `RepKind`
   - minimal ops
   - `freeze/thaw`
   - owner lock
2. birth map inventory
   - `substring_hii`, `concat3_hhh`, `string_len_from_handle`, `string_handle_from_owned`
   - current birth site と future shadow op を 1 枚で固定する
3. narrow pilot
   - `kilo_micro_substring_concat` だけ
   - search/control kernel wave は follow-up に分離し、current pilot へ黙って混ぜない
4. perf proof
   - asm/stat を主証拠
   - stable whole-program は guard
5. reopen decision
   - ほかの string chain に広げるか再判定
6. authority migration prep
   - `.hako` 側へ戻す contract/export shape を先に書く

## Stop Lines

次のどれかが必要なら、この pilot は止めて設計を切り直す。

1. VM に token を見せたくなった
2. plugin/FFI に `StrView/StrPieces` を見せたくなった
3. backend ごとに別 `RepKind` を作りたくなった
4. Rust docs なしで private optimizer rule を増やしたくなった
5. `TransientStringBox` や新しい runtime layer を足したくなった
6. raw byte leaf まで `.hako` に即移したくなった
7. `BoxBase::new` / `box_id` の semantics を変えたくなった

## Non-goals

1. Rust に恒久 optimizer owner を作ること
2. runtime substrate を今すぐ boxless にすること
3. VM / plugin / ABI contract を広げること
4. `llvmlite` keep lane を reopen すること
5. `RepMIR` を repo-wide public IR として先に広げること
