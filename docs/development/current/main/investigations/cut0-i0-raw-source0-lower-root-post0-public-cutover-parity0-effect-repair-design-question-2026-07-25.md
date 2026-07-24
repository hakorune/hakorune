# RAW public cutover PARITY0 effect repair design question

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-EFFECT-prime-r1`

PARITY0-S0a の最初の empty Script 比較で、Legacy と Raw の root `main`
signature effect が一致しないことを確認した。

```text
Legacy main  = EffectMask::PURE (bits 0x0001)
Raw main     = EffectMask::READ (bits 0x0010)
```

原因は次の一箇所に限定される。

```text
Legacy authority:
  src/mir/builder/module_lifecycle.rs
  root main signature = EffectMask::PURE

Raw producer:
  src/mir/builder/raw_root_body_lowering.rs
  begin_raw_root_function_v1() = EffectMask::READ.add(ReadHeap)
```

empty body の `Const(Void)` と `Return` はどちらも PURE で、後段の
postprocess/semantic refresh は signature effect を再計算しない。したがって
これは invocation-local ID の差ではなく、公開 MIR 契約の意味差である。

## Q1 — parity snapshot の扱い

**A を採用する。** signature effects は PARITY0 の exact comparison field のまま保持する。
snapshot から除外、READ/PURE の別名化、Raw-specific normalization は禁止する。

## Q2 — 修理 authority

**A を採用する。** Legacy の root `main` contract を authority とし、Raw BODY0 の
root skeleton producer を `EffectMask::PURE` へ合わせる。命令列から effect を導出する
大規模 policy 変更や Legacy 側の再設計は別 row に分離する。

## Q3 — 最小実装

```text
raw_root_body_lowering.rs
  root main FunctionSignature.effects = EffectMask::PURE
  remove unused Effect import if applicable

raw_root_body_lowering_p0.rs or a new focused fixture
  empty/scalar root signature effect = PURE

PARITY0-S0a
  empty Script normalized Legacy-vs-Raw equality rerun
```

production consumer、public ingress policy、postprocess kernel、JSON、executor は変更しない。

## Q4 — fail-fast boundary

修理後も signature effect が一致しなければ PARITY0 の success matrix を拡張せず、
さらに別の `PARITY0-REPAIR` row を開く。snapshot の弱体化で通してはいけない。

## Acceptance

```text
Raw BODY0 empty/scalar root main signature effects = PURE
condition_fn effect parity unchanged
empty Script Legacy-vs-Raw snapshot equal
raw_root_body_lowering focused tests green
PARITY0 snapshot does not omit effects
production files < 800 lines
no JSON/executor/normal-entry/old-Raw change
```

## Non-claims

```text
effect inference from arbitrary instruction streams
Print/Call effect policy widening
normal compile_with_source cutover
JSON/executor/selfhost/fastmem/CUT0
old Raw retirement
```
