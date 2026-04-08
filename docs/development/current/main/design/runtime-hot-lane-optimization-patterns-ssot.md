---
Status: SSOT
Scope: runtime hot lane 最適化で「先に pattern を固定し、実装の汎用 framework 化は後ろへ送る」ための共通ルール
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-137x/README.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/helper-boundary-policy-ssot.md
- docs/development/current/main/design/optimization-portability-classification-ssot.md
---

# Runtime Hot Lane Optimization Patterns SSOT

## Goal

この文書は、`len_h` / `substring_hii` のような runtime hot lane 最適化を、

1. 関数ローカルな当てものだけで終わらせない
2. まだ勝っていない abstraction を早まって共通化しない

ための正本だよ。

## Current Rule

- 先に固定するのは implementation ではなく pattern
- generic framework 化は後ろ
- keep するのは「別 lane にも伸ばせる invariant」だけ
- reopen 中の current proving ground は `string` lane

## Why

`Rust source がきれい` と `emitted asm が細い` は同じではない。

いま hot なのは算術や copy 本体より、

- dispatch probe
- trace gate
- drop epoch / invalidation
- fast cache hit path
- fallback / publication / reissue の境界

のような runtime mechanics だよ。

だから、helper 名や file split を generic にする前に、
「どの invariant が actual keeper だったか」を先に固定する必要がある。

## Landed Keeper Patterns

### 1. Direct Dispatch Probe

- hot path では state machine 全体を通さず、必要な raw dispatch source を 1 回だけ読む
- current keeper example:
  - `string_len_dispatch_probe_raw()`
- expected effect:
  - `STRING_DISPATCH_STATE` reread を消す
  - hot block の control-plane branch を減らす

### 2. Raw State Read + Cold Init Split

- hot path では cached raw state を読むだけ
- 初期化 / env read / slow setup は cold helper に落とす
- current keeper example:
  - `jit_trace_len_state_raw()`
  - `jit_trace_len_state_init()`
- expected effect:
  - hot block の `OnceLock` / env access / init branch を外へ出す

### 3. Global Epoch Mirror

- invalidation source は global mirror を直接読む
- registry-ready probe や lazy init path を hot block に持ち込まない
- current keeper example:
  - `host_handles::DROP_EPOCH`
- expected effect:
  - hot block から `REG` ready probe / `OnceCell` path を消す

### 4. Read-Only Scalar Lane Discipline

- scalar read fast hit では次を禁止する:
  - alloc
  - object birth
  - publication
  - trace emission
  - cache update
- current keeper example:
  - `len_h` trace-off fast return
- expected effect:
  - `cache hit -> return` を thin block として維持する

### 5. Hot Path Must Not Init / Allocate / Objectize

- hot lane で新しい state 初期化や objectize を inline しない
- fallback / publication / reissue は cold sink へ押し出す
- current string lane では substring 側がまだ proving 中だが、この制約自体は共通

## What Is Not Yet Generic

以下はまだ string-local / lane-local で扱う。

- `substring_view_arc_cache` shape
- borrowed-view publication / reissue protocol
- retained-view planner / materialize boundary
- route-specific cache state machine

理由:

- まだ multi-lane keeper になっていない
- abstraction を足すと exact front を悪化させやすい

## Genericization Gate

実装を generic にしてよいのは、次のどちらかを満たした時だけ。

1. 同じ pattern が 2 lane 以上で keeper になった
2. asm-visible change が 2 lane 以上で同じ形に再現した

例:

- `len_h` で勝っただけなら pattern 記録まで
- `charCodeAt` や `array.len` でも同じ direct probe / raw state split が keeper になったら framework 候補に上げる

## Current Stop-Line

今は次をしない。

- generic scalar lane framework の導入
- generic cache probe framework の導入
- generic route policy framework の導入
- substring runtime mechanics の higher-layer 化

今やってよいのは次だけ。

- pattern の naming を揃える
- narrow reusable primitive を置く
- docs / handoff に keeper invariant を残す

## Current Work Order

1. `string` lane で keeper invariant を増やす
2. exact + asm で「何が効いたか」を固定する
3. pattern 名で docs に登録する
4. 2 lane 目で再利用が confirmed してから framework 化する

## Operational Rule

- pattern doc は implementation ledger ではない
- rejected probe の exact evidence は phase investigation に置く
- ここには「勝った invariant」と「genericization gate」だけを残す
