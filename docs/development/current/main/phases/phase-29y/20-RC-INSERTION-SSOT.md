# Phase 29y: RC insertion historical compatibility contract

Status: Historical/legacy after A′ Ownership SSA (2026-07-14)
Scope: 旧post-CFG passの互換境界と退役条件を記録する。canonical event
placement authorityは本書ではない。

Current authority:

```text
strong ownership event placement:
  Verified Ownership SSA
  CopyOwned / DestroyOwned / exact edge forwarding

physical materialization:
  selected backend/runtime ObjectCell adapter

this post-CFG pass:
  legacy/optional compatibility only
  canonical caller zeroを経てretire

weak copy/drop:
  B′ weak-token contractとのco-seal待ち
```

Authority SSOT:
- `docs/development/current/main/investigations/mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md`
- `docs/development/current/main/design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md`

## 0. 目的

Historical objective (not current authority):

- RCイベント（retain/release/weak_drop）を **1箇所**で挿入し、backend差と hidden root を減らす
- 「SSA last-use = 寿命」にならないよう、drop点を **binding scope / explicit drop / 上書き**に限定する

## 1. 置き場所（SSOT）

Historical proposal: `emit_frag()` で CFG が確定した後、codegen直前に
**1回だけ**走る “RC insertion pass”

- ここなら PHI/loop/early-exit/cleanup を全部見た状態で挿入できる
- lowering 各所に retain/release を散らさない（SSOTを壊さない）
- 現行規約: legacy passはcanonical `CopyOwned`/`DestroyOwned`を追加・削除・
  再分類しない。lowerer/backend/runtimeはverified Ownership SSA以外から
  implicit ownership eventを発火させない。

## 2. 入力と出力（概念）

入力:
- “CFG確定後” の IR（Frag / block graph）
- 変数スコープ境界（binding scope）の情報

出力:
- RCイベントが明示された IR（例: `rc.retain`, `rc.release`, `rc.weak_drop` の effect 列）

重要:
- RCイベント列は “見える化” であり、RCの実体（カウンタ値/Alive判定）は runtime（NyRT）に委譲する

## 3. drop点の規則（最小）

禁止（意味論を壊しやすい）:
- SSA last-use を根拠に release する（weak_to_strong で観測できて破綻し得る）

許可（意味論に沿う）:
- explicit drop: `x = null` の直前に “旧値の release”
- 上書き: `x = <new>` の直前に “旧値の release”
- スコープ終端: binding scope の終端で “そのスコープの binding が保持していた current value の release”

## 4. PHI / loop / early-exit の罠（最低限）

- PHI: “入力値” を間違って release すると破綻する。drop対象は binding の current value と一致する必要がある
- loop: back-edge とスコープ終端の相互作用（継続で生存する値/上書きされる値）を分類する
- early-exit（return/break/continue）: exit edge により “どのスコープが閉じるか” が変わるため、cleanupと統合する必要がある

## 5. 実装に切る時の最小タスク（3つ以内）

1) insertion pass の入口/出口を固定（どのIRを入力にし、何を出力するか）  
2) drop点の規則を最小実装（上書き/explicit drop/スコープ終端）  
3) verify（不変条件）を追加（PHI/edge数と drop点が矛盾しないこと）  

## 6. Stability note（2026-02-14）

- `phase29y_rc_insertion_overwrite_release_vm.sh` は overwrite release 契約を pin する standalone diagnostic gate として維持する。
- overwrite gate の emit step は `NYASH_JOINIR_DEV=0` / `HAKO_JOINIR_STRICT=0` / `NYASH_USE_NY_COMPILER=0` を固定し、RC insertion 契約だけを検証する（JoinIR dev strict 由来の stack overflow ノイズを分離）。
- 現在の phase29y lane gate 必須stepは `phase29y_rc_insertion_entry_vm.sh` を採用し、overwrite gate は強制 replay しない。
- overwrite gate の再昇格条件:
  1) `phase29y_rc_insertion_overwrite_release_vm.sh` が連続 3 回 PASS
  2) stack overflow 再発が 0
  3) lane gate へ再投入しても `phase29y_lane_gate_vm.sh` が green 維持
