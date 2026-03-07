# Phase 100: Pinned Read‑Only Captures（設計メモ）

## 目的

JoinIR lowering のスコープ解決（`ConditionEnv → LoopBodyLocalEnv → CapturedEnv`）の前提を崩さずに、
「ループ外で定義した local（動的式でも可）」をループ内で receiver として参照できるようにする。

例（問題の形）:

```nyash
local s = "a" + "b"   // 条件式とは無関係
loop(i < n) {
  local ch = s.substring(i, i + 1)  // receiver 解決が必要
}
```

## 方針（SSOT）

- “Pinned local” は **新しい Pattern ではなく**、ループに入ってくる **read‑only 入力**として扱う。
- SSOT は増やさず `CapturedEnv` に統合し、`CapturedKind` で区別する:
  - `Explicit`: 従来の capture
  - `Pinned`: ループ内参照のために必要な read‑only local

## Fail‑Fast 契約

- `Pinned` は loop body 内で **再代入されない**（assignment target に現れたら拒否）。
- loop entry 時点で host 側 ValueId が存在する（`variable_map` に無い場合は拒否）。
- 初期化が loop より後にある等、支配関係が曖昧な場合は拒否（理由付き）。
- init 式の副作用有無は問わない（評価はホスト側で1回、Pinned は値を渡すだけ）。

## Loop Canonicalizer との関係（混ぜない）

- Phase 100 は「ループ外の値が loop 内で見えない」問題（capture/wiring）を解く。
- Loop Canonicalizer は「ループ内の不変式を外に出す」AST 変換であり、別ユースケース。

## 予定（段階投入）

この Phase は「自由度を上げるほどバグが出やすい領域」なので、段階投入で進める。

- **P1（read‑only）**: ループ外 local を `Pinned` として `CapturedEnv` に取り込み、receiver 解決を通す。
  - Fail‑Fast: loop body 内で再代入される変数は pinned 禁止（理由付き）
- **P2（mutable）**: loop body 内で再代入される外側変数は `Pinned` では扱わず、LoopState（carrier/env）に昇格して運ぶ。
  - 初期は “更新形を限定” して shape guard を作り、未対応は Fail‑Fast（曖昧に通さない）
- **P3（hoist）**: ループ内の不変式の巻き上げは Loop Canonicalizer 側で扱う（別ユースケースとして分離）

## P1: Pinned Local Captures (Explicit/Pinned) - 完了 ✅

Implemented CapturedKind enum to distinguish:
- **Explicit**: Traditional captures (condition variables, carriers)
- **Pinned**: Phase 100 read-only loop-outer locals used as method receivers

**Status**: P1-1 through P1-5 complete - pinned receiver functionality validated with fixture+smoke tests.

Example (now works):
```hako
local s = "a" + "b" + "c"    # Dynamic construction (loop-outer)
loop(i < 1) {
    local ch = s.substring(i, i+1)  # Pinned receiver works!
    print(i)
    i = i + 1
}
```

**Smoke test**: phase100_pinned_local_receiver_vm.sh

### Implementation Details

**Search Order (SSOT)**:
1. ConditionEnv (loop-outer scope)
2. LoopBodyLocalEnv (body-local variables - Phase 226 cascading)
3. CapturedEnv (pinned loop-outer locals)
4. CarrierInfo (mutable carriers)

**Key Components**:
- `CapturedKind` enum: Distinguishes Explicit vs Pinned captures
- `PinnedLocalAnalyzer`: Identifies loop-outer read-only locals
- `loop_body_local_init.rs`: Updated receiver resolution with full search order

### Test Coverage

- **Fixture**: `apps/tests/phase100_pinned_local_receiver_min.hako`
- **Smoke Test**: `tools/smokes/v2/profiles/integration/apps/phase100_pinned_local_receiver_vm.sh`
- **Regression**: Phase 96 and Phase 94 smoke tests pass

## P2: Mutable Accumulators (s = s + x form only)

**Constraint**: Mutable variables are LIMITED to accumulator pattern.

**Allowed form**:
- `s = s + x` where x ∈ {Const, BodyLocal, Captured, Pinned, Carrier}

**Fail-Fast (未対応)**:
- Any other mutation: `s = f(...)` / `s = x + s` / `s += x` / multiple updates / conditional updates
- Reason: Shape explosion prevention, safe accumulator support (JSON/CSV building)

**Example** (now works):
```hako
local out = ""
loop(i < 2) {
    local ch = "a"
    out = out + ch    # Accumulator form → LoopState carrier
    i = i + 1
}
print(out.length())   # Output: 2
```

**Implementation**:
- **P2-1**: MutableAccumulatorAnalyzer (AST shape detection only)
- **P2-2**: loop_break wiring (ScopeManager delegates read-only check)
- **P2-3**: Lowering (carrier update emission)
- **P2-4**: Integration test (fixture + smoke, length-based validation)

**LLVM EXE parity**: P2 accumulator を LLVM EXE smoke でも固定（phase100_mutable_accumulator_llvm_exe.sh）

## P3: String Accumulator (out = out + ch, Variable RHS only)

**Constraint**: String accumulator は最小形のみサポート。

**Allowed form**:
- `out = out + ch` where ch ∈ {Variable (string-ish)}
- RHS must be Variable (not Literal, not MethodCall)

**Fail-Fast (未対応)**:
- Literal RHS: `out = out + "x"` (P3.1 で解禁予定)
- MethodCall RHS: `out = out + substring(...)` (P3.2 で解禁予定)
- Non-string-ish RHS: 型判定は既存の box facts に委譲

**Example** (now works):
```hako
local out = ""
local s = "abc"
loop(i < 3) {
    local ch = s.substring(i, i+1)    # body-local string 1文字
    out = out + ch                    # String accumulator
    print(out.length())               # Output: 1, 2, 3
    i = i + 1
}
```

**Implementation**:
- **P3-1**: AccumulatorKind::{Int, String} 追加（型判定は委譲）
- **P3-2**: StringAccumulatorEmitter 専用箱（JoinIR lowering）
- **P3-3**: loop_break wiring（string carrier 昇格、emitter 分岐）
- **P3-4**: Fixture + smokes（VM + LLVM EXE）
