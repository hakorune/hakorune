---
Status: SSOT
Scope: MirInstruction の call-site 正規化（BoxShape lane, no behavior expansion）
Decision: accepted (phase29y-safe lane)
Updated: 2026-08-25 (MIR-CALL-RETIREMENT-v1 accepted)
Related:
- docs/reference/mir/INSTRUCTION_SET.md
- docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md
- src/mir/instruction.rs
- src/mir/definitions/call_unified.rs
- src/mir/contracts/backend_core_ops.rs
---

# MIR Canonical Callsite Lane (SSOT)

## 目的

MIR 命令数そのものを急いで減らすのではなく、まず「call-site 表現の入口」を 1 本化して診断距離を短くする。

- ゴール: backend 手前で call 系表現を canonical へ寄せる
- 方針: BoxShape（責務整理）を先行、BoxCount（受理拡張）はしない
- 契約: fail-fast。曖昧 fallback を増やさない

## 非目標

- 新しい受理形の追加
- AST rewrite
- NewClosure retire（NCL-2 で `Call(callee=Closure...)` の shape 契約は固定済み。`NewClosure` 自体の retire はこの lane の非目標）
- NewBox の即時統合（`Call` の legacy `func` 必須を先に整理する必要がある）

## 実装上の前提（現状）

- `MirInstruction::Call` は `func: ValueId` + `callee: Option<Callee>` の過渡形
- `used_values()` は `callee=None` でのみ `func` を使用し、`callee=Some(Method{receiver})` は receiver を使用
- `ValueId::INVALID` が sentinel として利用可能

参照:
- `src/mir/instruction.rs`
- `src/mir/instruction/methods.rs`
- `src/mir/value_id.rs`

## 最終像（この lane で到達する形）

- backend 入口では call-site を `MirInstruction::Call` で観測できる
- `BoxCall` / `ExternCall` は canonicalization 後に backend へ流れない
- `Call { callee: None }` は backend 入口で freeze（ただし `func=<const-string>` 形は MCL-5 で `Call(callee=Global)` へ正規化）
- docs SSOT と実装 ledger は常に同期（既存テストを維持）

注:
- DebugLog→Debug 統合と Nop retire は別 lane に分離する（本 lane では扱わない）



## 実行手順（1タスク=1コミット）

### MCL-0: Canonicalization pass 入口を作る（挙動不変）

- 追加:
  - `src/mir/passes/callsite_canonicalize.rs`（新規）
  - `src/mir/passes/mod.rs` へ module 追加
- ルール:
  - pass は MIR module を走査し、命令差し替えのみを担当
  - backend 判定・reject はこのコミットで入れない（次コミットに分離）
- 受け入れ:
  - ビルド緑
  - 既存 test 緑

### MCL-1: `BoxCall -> Call(callee=Method)` 変換を追加

- 変換契約:
  - `BoxCall { dst, box_val, method, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Method{ box_name, method, receiver: Some(box_val), certainty, box_kind }), args, effects }`
- 注意:
  - `box_name/certainty/box_kind` が不明な場合は conservative 値を使う（runtime data + union など）
  - 変換不能なら即 freeze（silent keep しない）
- 受け入れ:
  - `used_values()` が receiver + args を維持
  - parity break を起こさない

### MCL-2: `ExternCall -> Call(callee=Extern)` 変換を追加

- 変換契約:
  - `ExternCall { dst, iface_name, method_name, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Extern("<iface>.<method>")), args, effects }`
- 受け入れ:
  - extern route の結果/副作用が既存と一致
  - legacy `ExternCall` が backend 入口まで残存しない

### MCL-3: backend 入口の fail-fast 契約を固定

- 追加契約:
  - backend 入口で `Call { callee: None }` を reject（freeze）
  - backend 入口で `BoxCall` / `ExternCall` 残存を reject（freeze）
- 受け入れ:
  - reject tag が安定
  - 既存 green ケースは維持

### MCL-4: docs / tests 同期

### MCL-5: `Call(callee=None, func=<const-string>)` を `Call(callee=Global)` へ正規化

- 変換契約:
  - `Call { callee: None, func: <const-string value-id>, args, ... }`
  - `=> Call { callee: Some(Callee::Global(<string>)), func: ValueId::INVALID, args, ... }`
- 目的:
  - Program(JSON v0) runtime route で `call-missing-callee` freeze を回避し、MCL lane の backend 契約へ合流させる。
- 受け入れ:
  - `mcl5_rewrites_legacy_call_with_const_string_func_to_global_callee` が green。


- 更新:
  - `docs/reference/mir/INSTRUCTION_SET.md`（必要なら運用注記のみ）
  - `docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md`（cohort 説明）
- テスト:
  - `instruction_diet_ledger_counts_match_docs_ssot` を維持
  - canonicalization 後に legacy call-site 命令が 0 であることを確認する unit test を追加

## post-canonical retire queue（次レーン）

詳細SSOT: `docs/development/current/main/design/mir-callsite-retire-lane-ssot.md`

- 固定順序:
  - 1) Rust 側で legacy call-site を canonical call に吸収（MCL-0..5 完了）
  - 2) `.hako` mirbuilder の新規出力を canonical call-site へ移行
  - 3) `BoxCall/ExternCall` を enum から retire
- キュー:
  - RCL-0 (docs-only): done（`mir-callsite-retire-lane-ssot.md` で契約固定）
  - RCL-1 (BoxCount): done（`.hako` mirbuilder の emit を `Call(callee=Method/Extern)` へ統一）
  - RCL-2 (BoxShape): done（strict/dev の stage1 selfhost MIR 受け口で legacy emit を fail-fast reject）
  - RCL-3 (BoxShape): done（min1/min2/min3 完了。`BoxCall/ExternCall` enum retire 済み）
  - RDN-0 (separate lane): `DebugLog/Nop` retire は callsite lane と分離

## NewClosure clean path（2段階 + 契約固定）

- NCL-0: done（`Call(callee=Closure)` は canonicalization pass で `NewClosure` へ正規化し、backend 境界では `call-closure-not-canonical` を fail-fast）
- NCL-1: done（`NewClosure.body` は `body_id -> module.metadata.closure_bodies` へ外出しし、canonical 形は `body=[]` を維持）
- NCL-2: done（shape 判定を SSOT 化。`dst=Some + args=[]` のみ canonicalize、それ以外は shape-specific fail-fast）

## Core Call target retirement successor (active)

MCL/RCLはbackend入口のcanonicalizationと旧instruction variant退役を閉じた。
core `MirInstruction::Call` 自体は、なお
`func: ValueId + callee: Option<Callee>`というmigration carrierである。
次のlaneは新しいCall authorityではなく、このSSOTのretirement successorとする。

```text
MIR-CALL-LEGACY-TARGET-CENSUS-D0
  -> MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0
  -> MIR-CALL-LEGACY-TARGET-RETIREMENT-R0
```

`CENSUS-D0`は全`callee: None`をcanonicalizer input、明示compatibility、test、
diagnostic、unreachableへ分類する。text countや`ValueId::INVALID`だけで
retirementを選ばない。`GUARD-I0`はまずselected native/canonical corridorだけで
`callee.is_some() == 100%`、legacy func authority/fallbackが0であることを固定する。
現状は`callee=Some`でもJoinIRやJSON transportが有効な`func`値を併記するため、
`func == ValueId::INVALID`をこのguardの不変条件にはしない。また、このcountを
証明するためだけの新しい`FunctionMetadata` semantic rowも追加しない。最初の
consumerはmodule/backend境界のpure structural censusでなければならない。

`MIR-CALL-RETIREMENT-v1`はend stateを次の一つへ固定する。

```text
Call { dst, callee: Callee, args, effects }
```

JSON-v0 compatibilityはcanonical MIR外のowner-private
`JsonV0CallInput`にだけ隔離し、exact targetを一度だけ`Callee`へ解決してからCallを
構築する。このinputはsource evidenceであって第二Call authorityではない。
`MirInstruction::LegacyCall`、`Option<Callee>`、`func`、default/sentinel Callee、
optimizer target再推論、backend string fallback/retryは最終形に残さない。

Direct MIR JSON-v0のlegacy `func`は、raw function draftから先に作る
owner-private immutable catalogだけをsource authorityにする。catalogは同一関数内の
直接`Const(String)`定義をValueIdへ対応付け、解決時に一意性・非String・foreign・
`ValueId::INVALID`を検証する。成功時は文字列をそのまま`Callee::Global`へ投影し、
module membership、arity suffix、Extern分類、alias追跡、optimizer/backend lookupは
行わない。`call`とnested `mir_call`は同じinput ownerを使い、nested nodeがtarget/
args/effects、outer nodeが`dst` overrideを所有する。

Program JSON-v0のgeneric `ExprV0::Call`は、top-level local defsから先に作る
owner-private immutable `ProgramCallTargetCatalog`を使う。import aliasのstatic/extern
producerは既存ownerとしてcatalog外にfenceし、post-lowering import mergeをmembership
authorityにしない。unique `(short name, arity)`はqualified `Callee::Global`へ一度だけ
投影し、ambiguous local candidateはtyped reject、候補のないsource nameは文字列を保った
exact `Global` terminalとする。`env.`/`nyash.`はnumeric arity suffixだけを除いて
`Callee::Extern`へ投影する。空名・重複qualified definitionはCall/Block publication前に
rejectする。catalogはmain/defs lowering前、targetはargument lowering前に確定し、
`func_map`、`maybe_resolve_calls`、target Const、Program bridge late issuer、import
merge scan、optimizer/backend/runtime lookupはauthorityではない。

このcanonical production boundaryはdirect MIR-v0、selected Rust VM/JSON/LLVMまでを
覆う。Program-v0はR3の別catalog owner、PyVM（daily route 0・diagnostic-only）、
reference-vm、Python/llvmliteは歴史/互換ownerとして`ParkedSealed`に分類する。これらは
現行R5の実装対象でもterminal closure edgeでもなく、canonical acceptanceへ再入場するのは
selected backend policyが明示的に再開した時だけとする。PyVMの物理削除は別SSOTの
removal conditionで扱う。

literal censusに加えて、`runner/mir_json_v0/call.rs`の入力依存stateも数える。
現HEADの選択境界では、Program-v0の旧literalは削除済みで、MIR-v0はresolve/reject
後にだけ発行するため、missing-callee publicationはzeroである。これは
`callee: None` の検索だけでなく、typed stale `func` と `Method(None)` の別edgeを
分類したうえで維持するzeroであり、runtime target authorityの消滅を意味しない。

field削除前に`Callee`のValueId operand ownerを一つにする。Method.receiver、
Value(value)、Closure.captures、Closure.me_captureをtarget operandとし、
Global/Extern/Constructorはtarget operandを持たない。Call argsは全variantで順序を
保ってoperandになる。escape判定はこのprojectionを再利用してよいが、別policyの
まま保つ。

R4a closed: `Callee` owns the ordered occurrence rewrite API
`rewrite_value_operands(&mut self, FnMut(&mut ValueId))`. The order is
Method.receiver, Value, Closure captures in stored order, then me; duplicates are
retained. The match is exhaustive with explicit empty Global/Extern/Constructor/
Method(None). SimplifyCFG is migrated and parity/guard evidence is green; `func`,
Call args, ownership, escape, JoinIR, and field deletion remain separate rows.

R4b closed: `used_values` has 56 direct non-test expressions across 37 files plus
one BasicBlock edge. The accepted law is typed Callee occurrences in the same
order, then args; legacy `None` keeps func once before args and duplicates remain.
The immutable exhaustive `Callee::for_each_value_operand` facet and `methods.rs`
Call arm are now green with owner/typed/legacy tests and the shared guard.

R4c closed (`4c6d9ce9a2`): `value_consumer` delegates Call membership once to
`MirInstruction::used_values`; focused refresh/fact-omission/legacy suite 5/5,
shared corridor/pointer/rustfmt/diff green, and 433 warnings remain baseline.
Its direct-set and per-instruction dedup policy stays local; typed `Callee`
targets are counted before args while legacy `None` keeps one `func` use.

R4d D0 accepted — `MIR-CALL-CANONICAL-OPERAND-ESCAPE-POLICY-D0`:
Decision: Callee enumerates target operands; escape assigns Call to Method.receiver/Value/args and Capture to Closure captures/me.
Source authority + canonical issuer: `MirInstruction::Call`/`Callee` -> `classify_escape_uses` -> DCE, escape, and FastMem consumers.
Non-authority: `used_values` generic uses, ownership SSA, FastMem allowlists, JoinIR/Query/CallLike, optimizer/backend text, PyVM/reference/Python.
Fail-fast boundary: invalid Closure shape remains the existing canonicalization reject; legacy `None.func` stays unclassified by the shared barrier and ordinary-use fail-closed in FastMem; no target re-inference.
Smallest next slice: update only `classify_escape_uses` Call membership, add the finite matrix tests, and extend the shared corridor guard; DCE/FastMem/VM policies remain consumers.
Non-claims: ownership activation, JoinIR remap, CallLike retirement, R5/R6, Method(None)/Closure/NewBox/Constructor finalization, warning cleanup; census is `classify_escape_uses` Call/Closure -> DCE/escape/FastMem, all other policies excluded.

### R4d accepted escape role matrix

| Call shape | projected values | shared escape role | compatibility note |
| --- | --- | --- | --- |
| `Some(Method { receiver: Some(v), .. })` | receiver | `Call` | instance receiver is the target operand |
| `Some(Method { receiver: None, .. })` | none | none | static form remains non-authoritative; qualified `Global` is the final form |
| `Some(Value(v))` | target value | `Call` | first-class callable target is a call barrier |
| `Some(Closure { captures, me_capture, .. })` | captures, then `me_capture` | `Capture` | pre-canonical constructor; same role as `NewClosure` |
| `Some(Global/Extern/Constructor)` | none | none for target | stored `args` still receive `Call` role |
| `None` legacy | `func` remains generic use only | no shared barrier | `used_values` preserves `func`; FastMem stays `ordinary_use` and fail-closed |
| every shape | `args` in stored order | `Call` | duplicates remain occurrences; each consumer keeps its own dedup policy |

Finite production census: one classifier Call arm, seven DCE local-field
expressions, one FastMem shared extraction plus three FastMem allowlist arms, and
one opt-in Rust escape consumer. The structural helper supplies occurrences;
each consumer retains its barrier/allowlist policy.

R4e D0 accepted — `MIR-CALL-CANONICAL-OWNERSHIP-POLICY-D0`:
Decision: do not mechanically apply generic Callee occurrences to Ownership SSA; use the explicit managed/unknown role matrix below.
Source authority + canonical issuer: `Callee` projection owns occurrences; Ownership SSA classification/ABI owns kinds and policy; no callee ABI issuer exists yet.
Non-authority: typed `func`/`INVALID`, `used_values`, variant spelling, backend/runtime inference, JoinIR/Query/CallLike, PyVM/reference/Python.
Fail-fast boundary: managed or unknown typed targets reject before liveness/witness seal; legacy `None` retains its `func` check once, typed `Some` never reads `func`; verifier activation remains zero.
Smallest next slice: replace only the Call arm in `verify_instruction_kinds`, add exact positive/negative/error-precedence tests, and extend the shared guard; no production witness installer.
Non-claims: ownership ABI/witness activation, managed-call support, R5/R6, backend changes, JoinIR/query/CallLike retirement, and warning cleanup; census is `ownership_ssa/verify.rs` Call scan -> liveness/witness and conditional backend preflight.

### R4e accepted ownership role matrix

| Call shape | target authority | ownership treatment | legacy field |
| --- | --- | --- | --- |
| `None` | legacy `func` | preserve current known-`None` requirement; args/dst keep current `None`-only policy | `func` is read once |
| `Some(Global/Extern/Constructor)` | no target operand | args/dst keep current `None`-only policy | ignored, including `INVALID` |
| `Some(Method { receiver: None, .. })` | no target operand | args/dst keep current `None`-only policy; static final form remains qualified `Global` | ignored |
| `Some(Method { receiver: Some(v), .. })` | receiver `v` | receiver, args, and dst must be known `None`; managed/unknown rejects before liveness | ignored |
| `Some(Value(v))` | target value `v` | target, args, and dst must be known `None`; managed/unknown rejects before liveness | ignored |
| `Some(Closure { captures, me_capture, .. })` | pre-canonical constructor operands | captures/me use generic `used_values` liveness; no managed-call target predicate; existing closure-shape canonicalization remains authoritative | ignored |

The matrix is a policy over the structural occurrence projection, not a new
ownership receipt. `ManagedCallOwnershipUnsupported` remains the fail-fast
terminal until a separately named managed-call ABI issuer exists.

R4f T0 selected — `MIR-CALL-CANONICAL-CALLLIKE-T0-R0`:
Decision: retire the private `CallLikeInst` metadata adapter; canonical
`MirInstruction::dst_value`/`used_values` remain the only Call metadata owner.
Source authority + canonical issuer: `MirInstruction::Call` plus the existing
`Callee` operand projection; `instruction_kinds` is observation-only.
Non-authority: `CallLikeInst`, duplicated receiver/func reconstruction, and
metadata-local target inference. Fail-fast boundary: `dst_via_meta` and
`used_via_meta` delegate/directly project canonical Call shape without changing
the Call schema or reading a typed legacy `func`. Smallest next slice: remove
the enum/impl, replace its two metadata arms with canonical delegation, add
parity tests and extend the shared guard. Non-claims: `func`/`Option<Callee>`
cutover, Query policy, JoinIR, backend activation, and warning cleanup.

Query D0 accepted — `MIR-CALL-CANONICAL-QUERY-POLICY-D0`:
Decision: treat `MirQuery` as an observation facade and do not let its Call arm
reconstruct target meaning. Source authority + canonical issuer: the existing
`MirInstruction::used_values`/`dst_value` plus `Callee` projection; Query only
projects the result to its read/write view. Non-authority: the local
`Callee::Method`/legacy-`func` match, Query variant spelling, JoinIR inference,
and backend/runtime fallback. Fail-fast boundary: `MirQueryBox::reads_of` and
`writes_of` must have one finite Call policy before R6; typed Value/Closure
operands are not optional decorations, and legacy `None.func` remains explicit
compatibility only. Smallest next slice: replace the one Query Call arm with
direct canonical delegation, add parity/negative tests, and extend the shared
guard; this is a non-production T0, not an I0 caller switch. Non-claims: R6
schema cutover, ownership ABI, JoinIR remap, PyVM/reference/Python, and warning
cleanup.

Query D0 finite state matrix:

| Query observation | Read authority/result | Write authority/result | Local terminal/fallback |
| --- | --- | --- | --- |
| typed `Some(Global/Extern/Constructor/Method(None))` | `MirInstruction::used_values`: args in stored order | `MirInstruction::dst_value`: `dst` or empty | project only; no target inference |
| typed `Some(Method(Some(v))/Value(v))` | `used_values`: target operands, then args | `dst_value`: `dst` or empty | project only; no receiver/name retry |
| typed `Some(Closure { captures, me_capture })` | `used_values`: captures, `me_capture`, then args | `dst_value`: `dst` or empty | preserve occurrence order/duplicates; no constructor reclassification |
| legacy `None` | `used_values`: `func` once, then args | `dst_value`: `dst` or empty | compatibility observation only; no retry |
| non-Call instruction | existing canonical instruction methods/arms | existing canonical instruction methods/arms | unchanged; Query does not issue semantic facts |

Census boundary: `MirQueryBox::reads_of`/`writes_of` -> the single non-test
`loop_form_intake` reader -> `loop_to_join` and toggle-gated skip/trim probes;
includes the one production impl and test-only alternate impl, excludes JoinIR
remap/merge, PyVM/reference/Python, and R6 schema ownership. The Query methods
return observations rather than rejects: malformed or unsupported target
meaning is rejected by its upstream canonical ingress, never retried here.

Query T0 closeout (`981ec1d583`): `MirQueryBox::reads_of(Call)` now delegates
to `MirInstruction::used_values`, and `writes_of(Call)` delegates to
`dst_value`. The finite matrix above is covered by two focused tests; the
shared corridor guard also rejects a local `callee`/`func` reconstruction.
This is an observation closeout, not a production caller switch.

R5 design decision accepted — `MIR-CALL-CANONICAL-TERMINAL-CLOSURE-D0`:

```text
Decision: close every selected terminal as a typed-Callee consumer, then cut
the core schema only after R5 evidence; split R5 into optimizer, selected Rust
interpreter, printer/JSON, and selected-native-backend owners.
Source authority + canonical issuer: typed producers or one compatibility
ingress issue Callee; MirInstruction::Call stores it; terminals only dispatch
or project that stored target.
Non-authority: func/ValueId::INVALID, target Const(String), optimizer scans,
backend symbol lookup, MirCall/CallFlags before the R6 gate, PyVM/reference/
Python, and warning counts.
Fail-fast boundary: malformed/missing legacy input rejects before Call publish;
terminal unsupported-target errors do not retry another target or by-name path.
Smallest next slice: R5a bounded optimizer target-issuer census and decision;
no code or production switch until its caller/producer boundary is closed.
Non-claims: R6 field deletion, JoinIR remap, Method(None), Closure/Constructor
shape retirement, normal-root cleanup, and non-selected backend activation.
```

Worker census result: R5 is not one atomic implementation. R5a is selected as
an optimizer-only I0; R5b is split so its first row only rejects the selected
Rust VM `None -> func` execution edge; R5c printer is separable but JSON
writer remains a compatibility-mixed design stop; R5d native is a design stop
because the selected pure-first route still has pattern fallback and its C
owners are at the 760/793-line boundary. PyVM/reference/Python/WASM and
`native_driver` remain `ParkedSealed`.

R5 finite terminal task matrix:

| Task | owner boundary | old edge to delete | acceptance evidence |
| --- | --- | --- | --- |
| R5a optimizer (selected) | optimizer-only schedule, CSE key, optimizer diagnostic | optimizer canonicalizer call, typed `func` key/scan, Const/String target issuer | optimizer caller/issuer/retry = 0; post-RC/JSON compatibility issuer remains outside; key and diagnostic parity |
| R5b-B0 Rust interpreter (next) | `handlers/calls`, both instruction execution loops | `None -> func` register load and module by-name execution | `None` is typed terminal reject; typed Global/Method/Extern parity; no field deletion or method fallback claim |
| R5c printer (later) | MIR printer observers only | legacy `func` rendering and typed-call dummy display | typed target projection parity; JSON remains separate |
| R5c JSON (design stop) | v1 writer, v0 projection, legacy wire/profile switches | mixed canonical/compatibility emission and `Method(None)` receiver reuse | profile split and egress authority decision before code |
| R5d native (design stop) | selected `ny-llvmc` pure-first route | missing structured callee abort boundary and pattern fallback | route/profile decision plus C-owner split before implementation |

R5a selected brief (closed at `e36f86e869`):

```text
Decision: retire only the optimizer's target-issuing/legacy-observer edges;
shared compiler post-RC and JSON compatibility canonicalizers stay outside.
Source authority + canonical issuer: typed producer or compatibility ingress
-> Callee -> MirInstruction::Call; optimizer stores/keys/diagnoses only.
Non-authority: optimizer func/INVALID, Const(String), value-type inference,
module membership, backend/runtime lookup, and CSE legacy target keys.
Fail-fast boundary: optimizer never repairs/retries a target; residual legacy
input remains an explicit compatibility observation until R6.
Smallest next slice: remove optimizer schedule call, make CSE keys Callee-based,
and remove diagnostic func->Const scan; add matrix tests and shared guard.
Non-claims: shared canonicalizer retirement, R5b/c/d, R6 fields, Method(None),
Closure/Constructor, JoinIR, PyVM/reference/Python, and warning cleanup.
```

R5b-B0 selected brief:

```text
Decision: selected Rust interpreter treats missing Callee as a terminal typed
reject; it does not load func or retry module lookup. Existing typed dispatch
remains the only execution path for this row.
Source authority + canonical issuer: typed producer or compatibility ingress
-> Callee -> MirInstruction::Call; interpreter consumes the stored Callee.
Non-authority: func, register String values, interpreter function-map lookup,
Method(None) fallback, plugin/method recovery, and non-selected backends.
Fail-fast boundary: before any `reg_load(func)` or module `functions.get`,
`callee=None` returns the stable `call-missing-callee` terminal error.
Smallest next slice: change `handlers/calls::handle_call` only, add a direct
negative test and preserve both instruction-loop callers and current fields.
Non-claims: `func`/Option deletion, Method(None), printer/JSON/native closure,
JoinIR, PyVM/reference/Python, warning cleanup, and any VM method fallback.
```

R5a closeout evidence: `e36f86e869` removes the optimizer late schedule, makes
typed CSE keys independent of stale `func`, and removes the diagnostic
`func -> Const(String)` scan. The callsite compatibility suite is 14/14,
CSE key tests are 3/3, and the optimizer filter includes the new diagnostic
test; the shared corridor guard, pointer guard, and `git diff --check` are
green. The optimizer filter also retains two unrelated known baseline freezes
(`mir/instance-constructor-source/cohort-missing`), so they are not attributed
to this row. Warning count remains the 433-item baseline.

R5b-B0 closeout evidence: commits `95427f2cd6` and `67dd7e400a` make
`handlers/calls::handle_call` reject `None` before `reg_load(func)` or module
lookup, and update the vm-reference fixture to the typed method carrier. The
negative test is 1/1, typed Method parity is 18/18, typed Global call-contract
coverage is 8/8, and typed Extern provider coverage is 4/4. The shared
corridor/pointer guards and feature lib check are green; feature test warnings
are the 437-item vm-reference baseline (the default lane remains 433). R5b
keeps the instruction fields and all typed dispatch/method-fallback boundaries
unchanged.

R5c printer-only selected brief:

```text
Decision: printer observers project the stored typed Callee; they do not read
func or reconstruct a target. This row changes display only.
Source authority + canonical issuer: MirInstruction::Call.callee is issued by
the existing producer/compatibility ingress and is the printer's sole target.
Non-authority: func/INVALID, Const(String), JSON wire profiles, backend lookup,
and any target classification or retry.
Fail-fast boundary: typed Call display succeeds from Callee; legacy None keeps
an explicit compatibility rendering and is not silently presented as typed.
Smallest next slice: remove the display observer that always prints func,
route typed/legacy rendering through one printer projection, and add parity
tests plus the shared corridor guard.
Non-claims: JSON writer, Method(None), Closure/Constructor, interpreter,
native/other backends, core field deletion, JoinIR, PyVM/reference/Python.
```

R5c JSON egress design stop (`MIR-CALL-JSON-EGRESS-D0`):

```text
Decision: JSON egress uses typed Callee as the sole meaning source; v1 writer
and owner-private v0 compatibility projection are separate profiles.
Source authority + canonical issuer: existing producer/JsonV0 ingress issues
MirInstruction::Call.callee; writer projects it without reclassification.
Non-authority: func/INVALID, Const(String), wire strings, environment switches,
backend lookup, backend_shape post-wire reclassification, and parked backends.
Fail-fast boundary: targetless None+INVALID, Method(None), unsupported
Global, and profile-incompatible Closure/Constructor reject before JSON publish;
no func fallback or retry.
Smallest next slice: profile authority, finite input matrix,
Method(None)/Closure/Constructor boundary, and post-wire mutation census.
Non-claims: R6 field deletion, Method(None) retirement, Closure/NewBox
integration, native implementation, or PyVM/reference/Python/native_driver.
```

The census boundary is `MirInstruction::Call` producer ->
`mir_json_emit::emitters::{mod,calls,helpers}` -> `root/io` -> CLI, ny-llvmc,
and compatibility loader. The finite target matrix is:

| Stored target | v1 profile | v0 compatibility profile | D0 blocker |
| --- | --- | --- | --- |
| `Global` / `Extern` | typed `mir_call` | `call` / `externcall` projection | Global(print) and `backend_shape` reclassification |
| `Method(Some(r))` | typed receiver | typed or `boxcall` projection | profile-specific receiver authority |
| `Method(None)` | nullable receiver | `boxcall(box=func)` today | old `func` receiver reuse must reject |
| `Value` | typed `mir_call` | legacy `call(callee)` | profile round-trip only |
| `Constructor` / `Closure` | `NewBox` / `NewClosure` projection | Call-shaped compatibility | construction-versus-call boundary |
| `None + valid func` | incompatible | explicit legacy `call(func)` | compatibility ingress only |
| `None + INVALID` | incompatible | targetless legacy shape | reject before publish |

The profile census includes `root.rs` schema selection, per-call dialect
switches (`NYASH_JSON_SCHEMA_V1`, `NYASH_MIR_UNIFIED_CALL`, and methodize),
`emit_call_with_optional_func`, `Method(None)` receiver reuse, unconditional
closure projection, and `backend_shape` mutation. Existing v0/print/methodize
parity is positive evidence; typed `func` decoration ignored, `None+INVALID`,
Method(None) fallback, Constructor/Closure mismatch, and post-wire target
reclassification are negative evidence. The accepted D0 boundary keeps the
current root/profile selector and compatibility owners unchanged while
retiring one typed-path decoration edge first.

R5c JSON D0 closeout — first fast row:

```text
Decision: v0 JSON projections of an explicit Callee must not emit the stale
numeric func decoration; legacy None and Method(None) compatibility remain.
Source authority + canonical issuer: stored Call.callee -> calls.rs typed v0
projection; root profile selection remains the existing compatibility owner.
Non-authority: func/INVALID for typed Global/Extern/Constructor/Value/Closure,
wire strings, backend_shape mutation, and target reclassification.
Fail-fast boundary: typed projection never reads func; only explicit legacy
None uses emit_call_with_optional_func, while Method(None) stays R6-gated.
Completed slice: MIR-CALL-JSON-TYPED-DECORATION-I0-R0; typed helper no longer
forwards func, v0 typed/legacy/Method receiver parity is 8/8, and the guard is green.
Non-claims: profile split D1, Method(None) retirement, Closure/NewBox,
backend_shape, native, R6, and PyVM/reference/Python/native_driver.
```

Exact old edge: `emit_call_with_callee_v0` forwards `func` to
`emit_call_with_optional_func`, which emits a numeric `func` whenever the
decoration is non-`INVALID`. The I0 removes that forwarding only for explicit
typed variants. `Method(Some(receiver))` retains receiver projection;
`Method(None)` retains its compatibility `receiver <- func` edge until R6.
Positive acceptance is v0 typed Global/Extern/Constructor/Value/Closure output
without numeric `func`, plus unchanged v1 output. Negative acceptance is stale
typed `func` ignored, explicit legacy `None` still emitted, and no change to
Method(None), root profile, or backend_shape behavior.

JSON profile D1 accepted decision:

```text
Decision: root selects exactly one JsonEgressProfile; CanonicalV1 is the mainline profile and CompatibilityV0{methodize} is the owner-private legacy profile. Mixed selectors reject.
Source authority + canonical issuer: root selector parses S/U/M once; stored Call.callee remains the semantic issuer; emitters only project the selected profile.
Non-authority: func/INVALID except explicit V0 legacy ingress, wire text, independent env reads, backend_shape mutation, post-wire lookup/retry, and parked backends.
Fail-fast boundary: invalid/mixed selector, targetless Call, and unsupported profile/target combinations reject before root or instruction publication; malformed explicit targets never retry through legacy fields.
Completed slice: MIR-CALL-JSON-PROFILE-I0-R0 landed at `db350b81c9`; the immutable profile is threaded from root through emitters/mod.rs to calls.rs, per-call selector reads are gone, root tests are 2/2, calls tests are 8/8, and the shared guards are green.
Next design stop: MIR-CALL-JSON-BACKEND-SHAPE-D1-CAPABILITY-AUTHORITY; no R4c, native, or R6 implementation is implied.
Non-claims: backend_shape removal, Method(None) retirement, Closure/NewBox decision, loader fallback split, native, or R6 core schema cutover.
```

Selector state matrix (`S=NYASH_JSON_SCHEMA_V1`, `U=NYASH_MIR_UNIFIED_CALL`,
`M=HAKO_MIR_BUILDER_METHODIZE`):

| S | U | M | outcome |
| --- | --- | --- | --- |
| Unset/On | Unset/On | valid or ignored | `CanonicalV1` |
| Unset | Off | valid (`Unset` defaults true) | `CompatibilityV0{methodize}` |
| Off | Unset/Off | valid (`Unset` defaults true) | `CompatibilityV0{methodize}` |
| On | Off | any valid | typed reject `mixed_profile` |
| Off | On | any valid | typed reject `mixed_profile` |
| Invalid | any | any | typed reject `invalid_selector` |
| any | Invalid | any | typed reject `invalid_selector` |
| any | any | Invalid | typed reject `invalid_selector` |

The first row includes `S=Unset,U=Unset`, which is the canonical default. `M`
is parsed for validity once; it is semantically ignored by `CanonicalV1` and is
carried only by `CompatibilityV0`. The profile is immutable after selection.
`backend_shape` remains an explicitly separate compatibility owner and is not
silently folded into this profile decision.

I0 acceptance:

```text
positive: one root profile reaches every production Call emitter; CanonicalV1
         preserves typed mir_call output; CompatibilityV0 preserves v0
         Global/Extern/Method/Value/Constructor/Closure projections.
negative: calls.rs/helpers.rs read no selector env; mixed/invalid selectors,
          targetless production Call, and profile mismatch fail before publish.
parity: root schema kind, callee, receiver, args, dst, and existing effects
        projection are unchanged for each accepted profile; backend_shape,
        loader retry/fallback, Method(None), and construction boundaries stay
        outside the I0 claim.
```

JoinIR operand-remap D0 result (`NoSafeSlice`; next design stop:
`MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1`):

```text
Decision: JoinIR Call collection/remap may delegate target ValueIds to the
  Callee operand projection, but no production edit is admitted until its
  named merge caller and shared lifecycle boundary are finite and observed.
Source authority + canonical issuer: Callee::for_each_value_operand and
  Callee::rewrite_value_operands own target occurrence order; MirInstruction
  ::Call owns args/dst; JoinIR only remaps IDs and never reclassifies targets.
Non-authority: local remap_callee matches, func/INVALID, generic used_values
  policy, target strings, backend lookup, and caller-zero/test fixtures.
Fail-fast boundary: collect/remap must be classified for every non-test
  JoinIR merge caller before delegation; an unowned or shared-lifecycle edge
  is NoSafeSlice, with no partial helper replacement or fallback.
Smallest next slice: read-only census of JoinIrIdRemapper Call arms and all
  direct merge/lifecycle callers; if the boundary closes, one R4c row replaces
  the local match and incomplete Call collection, with positive/negative/
  parity evidence and the shared corridor guard.
Non-claims: Method(None), Closure/Constructor shape, MirCall/CallFlags,
  mandatory-Callee schema, backend/native, PyVM/reference/Python, or warnings.
```

Finite D0 boundary: start at `src/mir/builder/joinir_id_remapper.rs` Call
collection/remap arms; include non-test direct consumers in
`builder/control_flow/joinir/merge/**` and value-lifecycle callers; exclude
phase test modules, disconnected/caller-zero fixtures, unrelated MIR
instructions, and all backend/native/parked routes. Completion requires a
finite owner/caller inventory, one lifecycle authority, and zero open or
reopened JoinIR remap blockers; otherwise the row remains `NoSafeSlice`.

D0 census result (read-only audit at current HEAD):

```text
collect_values_in_block -> merge/value_collector.rs plus two value-lifecycle
  owners; remap_instruction -> merge instruction and terminator rewriters.
The JoinIR merge graph has no non-test production caller (caller-zero), while
value_lifecycle.rs::verify_typed_values_are_defined and
value_lifecycle_definition.rs::prepare_transient_stale_value_facts_v1 have
different reachable/stale/retention and fail-fast semantics. Therefore a
shared Callee projection is observable, but a shared lifecycle authority is
not issued. R4c remains NoSafeSlice and no JoinIR code/test edit is allowed.
```

Next design task: `MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1`.

```text
Decision: classify caller-zero merge reachability and the two lifecycle
  owners before any Callee operand delegation; keep collection/remap policy
  separate from value retention/finalization policy.
Source authority + canonical issuer: JoinIR merge coordinator owns merge
  reachability; each lifecycle owner issues its own retention/fail-fast facts;
  Callee remains the sole target operand projection authority.
Non-authority: local remap_callee, typed func/INVALID, generic used_values,
  caller-zero fixtures, backend/runtime names, and lifecycle cross-inference.
Fail-fast boundary: before Phase 1-2 collection or Phase 4-5 rewrite, every
  non-test edge must be classified; missing caller/lifecycle issuer is
  NoSafeSlice and cannot be filled by a default or partial delegation.
Smallest next slice: finite D1 census of merge caller reachability, both
  lifecycle owners, direct remap/collection edges, and reopen triggers; only
  after D1 acceptance may R4c replace the local Callee match.
Non-claims: Method(None), Closure/Constructor shape, MirCall/CallFlags, R6,
  backend/native, PyVM/reference/Python, and warning cleanup.
```

D1 finite state matrix:

| State | Authority | Terminal | Fallback |
| --- | --- | --- | --- |
| `LiveMergeCaller` | merge coordinator | named production boundary | none |
| `CallerZero` | bounded source census | `NoSafeSlice` | no route invention |
| `VerifyLifecycle` | `value_lifecycle.rs` | separate retention contract | no stale-fact reuse |
| `StaleFactsLifecycle` | `value_lifecycle_definition.rs` | separate prepare/fail-fast contract | no verify reuse |
| `SharedBoundaryUnresolved` | D1 design owner | `NoSafeSlice` | no partial helper replacement |
| `TestOrDisconnected` | test/parked owner | `ParkedSealed` | excluded from production claim |

D1 result: the finite census is accepted. The JoinIR merge graph is
`CallerZero`, the two lifecycle owners remain separate policies, and the
Callee projection is reusable only as an occurrence helper; no production
caller or shared lifecycle issuer exists for an R4c switch. R4c is therefore
closed as `NoSafeSlice` rather than implemented. The next selected design stop
is `MIR-CALL-JSON-BACKEND-SHAPE-D1-CAPABILITY-AUTHORITY`.

Backend-shape D0 result (`NoSafeSlice`): the bounded audit found both typed
pre-wire mutation and post-wire JSON mutation, a v0/v1 profile mismatch, and
silent field/default handling. The selected terminal capability for raw
`externcall` was not proven, so deleting or moving either normalizer would
have changed authority without a safe replacement.

Feedback reconciliation at the current source boundary:

```text
used_values()       = closed by 8eca2dd048; it delegates every Callee operand
                      (Value, Method receiver, Closure captures/me) then args
escape projection   = closed by 4e71066e57; it reuses the same occurrence
                      projection while retaining its own barrier policy
JoinIR remap        = local Callee match remains; caller/lifecycle D1 census
                      closed as NoSafeSlice, so no production R4c switch exists
R6 design debt      = MirCall/CallFlags, Method(None), Closure construction,
                      and Constructor/NewBox remain one future schema gate
PyVM/reference/etc. = retired/diagnostic-only boundary; ParkedSealed and not
                      evidence for this selected production terminal
```

Backend D1 is accepted as a strict adapter boundary. `backend_shape` is not a
second semantic issuer and is not atomically retired in this row:

```text
Decision: retain one strict compatibility projection for the Program JSON-v0
  bridge, and fence the selected ny-llvmc terminal to structured canonical
  calls; raw externcall capability is not widened by this decision.
Source authority + canonical issuer: typed Callee plus root-owned
  JsonEgressProfile; `MirInstruction::call` owns the typed target, while the
  owner-local adapter only projects an explicitly validated compatibility
  payload once.
Non-authority: raw wire strings, func, backend symbol lookup, post-wire target
  inference, defaulted dst/args, dropped extra fields, retry/fallback, and
  PyVM/reference/Python/WASM behavior.
Fail-fast boundary: profile, structured callee, extern route, and arity are
  fixed before publish; malformed or profile-mismatched input rejects without
  retry or alternate target search.
Smallest next slice: `MIR-CALL-JSON-BACKEND-SHAPE-STRICT-ADAPTER-I0-R0` —
  remove typed pre-wire console mutation, require the complete v0 adapter
  shape, move the handoff projection inside the Phase0 profile fence, and add
  positive/negative/shared-guard evidence for the selected structured terminal.
Non-claims: JoinIR R4c, Method(None), Closure/Constructor, MirCall/CallFlags,
  R6 field deletion, generic raw externcall support, C backend expansion,
  native/PyVM/reference/Python/WASM, and warning cleanup.
```

Backend D1 finite state matrix:

| State | Authority | Terminal | Fallback |
| --- | --- | --- | --- |
| `TypedExternConsole` | typed Callee/profile owner | one strict bridge projection or native typed terminal | none |
| `TypedExternOther` | typed Callee | preserve typed target | no console mapping |
| `PostWireRecognized` | selected bridge owner | strict typed projection | no second mutation |
| `PostWireUnknown` | selected terminal capability | reject or preserve by explicit profile | no lookup |
| `MalformedWire` | JSON/profile validator | typed reject before publish | no default/retry |
| `RawExternCapabilityUnproven` | D1 authority owner | `NoSafeSlice` | no backend expansion |
| `DirectMirFirst` | direct route owner | unchanged, out of bridge census | excluded |

D1 result: the finite backend census is accepted. Canonical structured
`mir_call`/typed `Callee` is the selected terminal input; compatibility
`externcall` remains owner-local and is never treated as generic selected
terminal capability. The adapter must validate `op`, `func`, `dst`, and `args`
before the one console projection, preserve unknown compatibility routes
without target inference, and reject malformed/defaulted shapes.

Strict-adapter I0 is closed by `40559cceef`. Typed pre-wire backend mutation
was removed; the owner-local adapter now validates the complete v0 shape,
the handoff projection runs inside the Phase0 profile fence, and six focused
`backend_shape` tests plus runtime/source parity and the shared corridor guard
are green. The selected C terminal retains structured-callee routing, rejects
missing structured callee, and has no direct raw `externcall` arm or
fallback/retry.

The originally selected design stop was
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D0-CAPABILITY-AUDIT`; its result and the
next D1 co-seal task are recorded below:

```text
Decision: audit and choose the selected ny-llvmc structured terminal's exact
  capability boundary before any native switch.
Source authority + canonical issuer: typed Callee plus root-owned
  JsonEgressProfile; the selected ny-llvmc C route table is terminal
  capability authority.
Non-authority: raw externcall wire, PyVM/reference/Python/WASM, generic
  backend guesses, symbol lookup, fallback, and retry.
Fail-fast boundary: route/profile/arity/symbol/link evidence is complete
  before publish; unknown, wrong-arity, or missing structured callee rejects.
Smallest next slice: finite native D0 census of the selected terminal route
  matrix with positive/negative/parity evidence and reopen triggers; no code.
Non-claims: JoinIR R4c, R6 field cutover, MirCall/CallFlags, Method(None),
  Closure/Constructor, warning cleanup, and non-selected backends.
```

I0 acceptance matrix:

| Input | Authority | Terminal | Outcome |
| --- | --- | --- | --- |
| typed `Callee::Extern` console route | typed Callee + profile | structured `mir_call` projection | one canonical Global projection, parity green |
| valid compatibility `externcall` with exact `op/func/dst/args` | owner-local v0 adapter | compatibility projection only | known console maps once; unknown route is preserved |
| missing/non-array `args`, missing `dst`, non-string `func`, or extra field | profile validator | before publish | typed reject, no default/drop/retry |
| raw `externcall` at selected ny-llvmc terminal | selected terminal capability | structured-call preflight | reject as profile mismatch |

Native D0 result: `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D0-CAPABILITY-AUDIT`
is `NoSafeSlice` for a native switch. The finite census boundary is:

```text
typed Callee + JsonEgressProfile
  -> structured call/mir_call JSON
  -> selected ny-llvmc Boundary/pure-first C dispatcher
  -> LLVM object
  -> hako_llvmc_link_obj_v2 + explicit libnyash_kernel.a
  -> executable
```

Raw `externcall`, CompatibilityV0, PyVM/reference/Python/WASM, and
`native_driver` are outside this terminal and remain `ParkedSealed`. The
selected dispatcher has structured `call`/`mir_call` arms and rejects a
missing structured callee; transport allowlists containing `externcall` are
not evidence of a C terminal capability.

Native D0 finite matrix:

| Input family | Current evidence | D0 terminal | Missing proof / fallback |
| --- | --- | --- | --- |
| exact structured Global/Extern plan | route table and several kernel exports | candidate native route | exact plan/arity/result/symbol/link co-seal required |
| `Global("print")` | direct string/scalar C surfaces | candidate native route | one owner for the arity and result contract |
| exact Extern shell route | route id/core op/arity/dst/value checks | candidate native route or explicit trap | full object→archive→executable census |
| generic Method family | helper symbols exist | `NoSafeSlice` | arity is split across family policies/registry |
| Constructor Map/Array | birth symbols exist | `NoSafeSlice` | producer/plan arity authority is not fixed |
| seven planned String routes | kernel/C surfaces exist | `NoSafeSlice` | C plan table has no rules; fallback is forbidden |
| twelve analysis routes | explicit unsupported capability | preflight reject | preserve typed reject boundary |
| missing/unknown/wrong-arity/malformed | structured preflight | typed reject | no lookup/retry |
| raw `externcall` at selected terminal | no direct C arm | profile-mismatch reject | no post-wire mutation |

The projection census currently reports Rust route specs `47`, generated C
capability rows `35`, C plan emit rules `28`, and Rust test expectations `29`.
The seven missing String plan rules are projection drift, not permission to
add a C fallback. Source/kernel exports alone do not prove that a selected
executable resolves every route: the required evidence is

```text
object undefined symbol
  -> libnyash_kernel.a defined symbol
  -> final executable defined symbol
```

for every accepted native row. Existing explicit archive linking through
`hako_llvmc_link_obj_v2` is retained, but its current smoke evidence does not
cover the full generic Method/Extern/Constructor/String set. Nearby C owners
are also at the physical boundary (`760`, `778`, `793` lines), and the generic
method owner is already `1104` lines; any future implementation requires a
path-preserving split before semantic changes.

Native D1 result: `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-D1-CAPABILITY-COSEAL`
is accepted as the capability boundary, but it does not authorize a native
switch or make unproved routes positive. The authority split is fixed:

```text
root JsonEgressProfile
  = selects CanonicalV1 versus owner-private CompatibilityV0 only
typed Callee + GlobalCallRoute / constructor_call_routes /
generic_method_routes / ExternCallRouteSpec
  = issues route, arity, result, symbol and proof facts
selected C registry and emit table
  = validates and emits the exact issued plan; it never reclassifies or looks up
hako_llvmc_link_obj_v2 + explicit libnyash_kernel.a
  = owns the physical link edge
```

Every selected row must be `NativePositive`, `ExplicitTrap`, or
`PreflightReject`. Unknown or missing Global/Constructor/Method plans are
`PreflightReject`; only an existing plan-declared trap (currently the
hostbridge route) is `ExplicitTrap`. The seven String rows, generic Method,
Constructor, Global print arity/result ownership, and route-wide link proof
remain unaccepted until their facts are co-sealed. Rust route specs currently
enumerate 47 rows, while generated C has 35 capability rows, 28 emit rules,
and the old test observer expects 29; the generator gap and old count are
drift evidence, not acceptance.

The next execution row is the behavior-invariant
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-C-OWNER-SPLIT-I0-R0`:

```text
Decision: split the selected 778-line C extern shell owner path-preservingly
  into rule/validation and emission children before semantic capability work.
Source authority + canonical issuer: no authority changes; the existing Rust
  route plans and C table remain byte-for-byte behavior owners.
Non-authority: new route rows, C name lookup, arity inference, raw externcall,
  fallback/retry, and link policy changes.
Fail-fast boundary: include topology must compile with identical symbols and
  route table behavior; no semantic call output may change.
Smallest next slice: move the existing rule table/validator and emitter body
  into children, keep the parent include path, add line-cap/compile evidence.
Non-claims: native switch, route projection co-seal, seven String routes,
  Global/Method/Constructor semantics, link proof, R6, and warning cleanup.
```

The C owner split is closed by `03b06622eb`. The parent facade still includes
the same two children in the same order, the rule/validation and emitter
contents were moved without semantic edits, `build_hako_llvmc_ffi.sh` and
`cargo check -p nyash-llvm-compiler --bin ny-llvmc` are green, the existing
`env.now_ms/0` JSON route test is green, and the shared corridor guard now
checks both child owners. The facade is 2 lines; the children are 465 and 312
lines. A temporary Hako source probe was not acceptance evidence: the current
MIR compiler stopped at `PhysicalHeader(CompletionNotValue { batch_slot: 1 })`,
so no AST workaround or native link claim is made.

The next design stop is
`MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-EXTERN-NOW-MS-LINK-PROOF-D0`:

```text
Decision: design one production-generated CanonicalV1 JSON to
  object→explicit archive→executable proof for exact `extern.env.now_ms/0`;
  never hand-author a semantic fixture or bypass the route issuer.
Source authority + canonical issuer: ExternCallRouteSpec and its existing
  route metadata/lowering-plan projection; JsonEgressProfile selects
  CanonicalV1, and hako_llvmc_link_obj_v2 owns the explicit archive edge.
Non-authority: Hako probes, hand-authored JSON, the hand-built route test,
  raw externcall, C name lookup, kernel export alone, fallback/retry, parked.
Fail-fast boundary: exact route/arity=0/result_value/symbol/return shape
  must be co-sealed before C emission; malformed plan or missing archive
  rejects before object/executable publication.
Smallest next slice: D0-A capture production JSON; D0-B derive malformed
  plan inputs (row deletion and arity=1); D0-C census undefined object,
  one archive definition, and final executable definition; D0-D record
  explicit nonzero rejection with no fallback or partial publication.
NoSafeSlice until production JSON is captured, the generic publication seam
  is selected without a new semantic receipt, C validates source_symbol too,
  and missing-plan/tuple failures are route-specific rejects.
Non-claims: native switch, seven String routes, Global/Method/Constructor
  expansion, R4c/R6, warning cleanup, Hako repair, or parked backends.
```

The read-only D0 audit keeps this row `NoSafeSlice` for five finite reasons:
(1) the existing route test builds a hand-made MIR and no production-generated
CanonicalV1 JSON artifact has been captured; (2) the Dynamic-V2 static
publication receipt is descriptor-specific and cannot be reused as EnvNowMs
semantic evidence; (3) the Hako probe stops at `PhysicalHeader` and is not a
repair target; (4) the route metadata has no typed capability receipt and C
does not yet compare `source_symbol`; and (5) missing-plan handling is
nonzero only through a generic unsupported path, not a route-specific
fail-fast rejection. These are design blockers, not negative evidence against
the route. The first accepted I0 may reuse the existing object/archive/exe
publication shape, but it must not mint a new semantic receipt or broaden the
route family.

Accepted prerequisite I0: `MIR-CALL-LLVM-TRAP-DECLARATION-COSEAL-I0-R0`:

```text
Decision: give the selected C module one physical llvm.trap declaration owner
  and emit it at most once per generated LLVM module; fence C1 declarations to
  the existing selected-Dynamic profile.
Source authority + canonical issuer: the C pure-compile module prepass owns
  the declaration-once bit; `program.selected_dynamic` is the only C1 profile
  selector, while MirCallNeedFlags/typed plans only request capabilities.
Non-authority: EnvNowMs route or symbol metadata, fn_metadata presence alone,
  individual trap call sites, opt/mem2reg, kernel exports, linker behavior,
  raw externcall, and parked backends.
Fail-fast boundary: declaration census must be one (or zero when no trap call
  is emitted) before `opt -passes=mem2reg`; duplicate declarations reject and
  no object is published.
Smallest next slice: keep C1 declarations behind `program.selected_dynamic`,
  route all prescan/hostbridge/C1 trap declarations through one once-helper,
  preserve existing trap call sites and all non-trap declarations.
Non-claims: EnvNowMs link proof, route/ABI/archive changes, Dynamic activation,
  static receipts, R6, warning cleanup, or any non-selected backend.
```

The bounded owner census found two active declarations in the ordinary
EnvNowMs path: boxed-sum/prescan declarations and unconditional C1 declarations
from non-null function metadata. A third hostbridge declaration is conditional
and must use the same once-helper. The prerequisite is now closed as a
behavior-preserving physical refactor;
Before the next I0, exact Extern D0 remained unaccepted until the
route-specific acceptance evidence was co-sealed.

I0 evidence: `build_hako_llvmc_ffi.sh`, the selected C1 physicalizer smoke, the
quick `ny-llvmc` check, the shared Call corridor guard, and the pointer guard are
green. The same production-generated CanonicalV1 JSON now emits an object with
an undefined `nyash.env.now_ms`, the explicit `libnyash_kernel.a` contains one
`llvm-nm` definition, and the Boundary executable contains one final definition.
The dumped optimized module has zero trap declarations when no trap call is
needed; the helper therefore enforces the allowed zero-or-one census. Derived
missing-plan and arity=1 inputs reject nonzero with no object, and a missing
explicit archive rejects before executable publication. The exact D0 route
specificity/source-symbol acceptance decision remained separate at that point.

Accepted bounded next slice: `MIR-CALL-EXTERN-PLAN-COSEAL-I0-R0`:

```text
Decision: make the selected C generic/same-module prepasses a fail-fast
  projection of an already-issued structured Extern plan; they must not
  classify or recover a missing route by name.
Source authority + canonical issuer: Rust ExternCallRouteSpec/classifier
  issues Callee::Extern, ExternCallRoute, and the CanonicalV1 lowering-plan
  row; C reads that row and compares the original structured callee name with
  source_symbol, then checks the existing physical emit-rule tuple.
Non-authority: C alias/name lookup, hand-written EnvNowMs acceptance, runtime
  symbol lookup, key_value/INVALID inference, raw externcall, compatibility
  replay, and parked backends.
Fail-fast boundary: the generic entry prescan and
  same_module_function_prepass_call_instruction, before body emission, LLVM
  optimization, llc, object publication, or archive/executable linking.
Smallest next slice: require source_symbol in the C view; for structured
  Extern, reject missing/malformed plans with the original callee name, and
  reject source-symbol, route-tuple, arity/args, or dst/result mismatches using
  the existing unsupported-shape diagnostic; keep valid EnvNowMs emission
  unchanged.
Non-claims: EnvNowMs-specific missing-plan classification, alias projection,
  other Extern routes, link-proof expansion, native switch, R6, or warnings.
```

Finite D0-D input matrix (boundary: structured MIR `Call` Extern payload at
the generic entry prescan or `same_module_function_prepass_call_instruction`
-> prepass failure or the existing emit rule; excludes raw `externcall`,
CompatibilityV0, and parked backends):

| Input state | Authority evidence | Required terminal |
|---|---|---|
| plan present, `callee.name == source_symbol`, tuple/arity/args/dst/result exact | Rust route row + C projection check | existing EnvNowMs call emission |
| plan present, source symbol differs/missing | Rust row is not the callsite identity | typed nonzero prepass reject |
| plan present, route tuple/proof/shape differs | C projection no longer matches issued route | typed nonzero prepass reject |
| plan present, arity or args count differs | plan arity and MIR args disagree | typed nonzero prepass reject |
| plan present, dst/result differs or is absent for scalar result | plan result relation disagrees | typed nonzero prepass reject |
| structured Extern plan missing or non-Extern plan at site | no route product to consume | generic `extern_call_missing_plan` reject with callee name; no C reclassification |
| raw `externcall`, CompatibilityV0, unknown non-selected route | outside selected boundary | existing reject/parked terminal; no new native path |

The missing-plan row is intentionally callsite-scoped rather than
EnvNowMs-specific: classifying it as EnvNowMs in C would require an alias
projection or a new C authority. Alias projection is a separate future slice,
not part of this I0.

Exact Extern D0 acceptance — `MIR-CALL-JSON-BACKEND-SHAPE-NATIVE-EXTERN-NOW-MS-LINK-PROOF-D0`:

```text
Decision: accept the selected CanonicalV1 EnvNowMs route only when the Rust
  issued ExternCallRoute/lowering-plan row and the original MIR callee name
  agree at the selected C prepass; no C alias classification or fallback is
  part of the accepted route.
Boundary: production MIR JSON capture -> generic entry/same-module prepass
  -> object -> one kernel archive definition -> executable definition; derived
  malformed rows terminate at prepass before object publication.
Positive: production env.now_ms/0 reaches an object with U nyash.env.now_ms,
  the selected archive has one T nyash.env.now_ms definition, and the linked
  executable has one T definition and runs with a timestamp result.
Negative: derived missing-plan, arity=1, source_symbol mismatch, and
  result_value mismatch each return nonzero with no object; diagnostics retain
  callee_symbol=env.now_ms/0 and use extern_call_* reasons.
Guard: C shim rebuild, quick ny-llvmc check, selected Dynamic smoke, pointer
  guard, canonical corridor guard, source/check line caps, and diff check are
  green; the executable's timestamp-derived process status is not a parity
  claim.
Non-claims: native switch, other Extern route expansion, R6 field deletion,
  MirCall/CallFlags cleanup, Method(None)/Closure/Constructor redesign,
  JoinIR lifecycle, warnings, or parked backends.
```

The exact D0 finite boundary is now `Exhausted` with no open blocker. The next
design stop is the R6 core-schema decision; it must first resolve the
MirCall/CallFlags split, required Method receiver, Closure construction
boundary, and Constructor/NewBox boundary before any field deletion.

Feedback reconciliation and deferred task queue (not selected):

```text
R4b status: HEAD already delegates Call used_values to Callee::for_each_value_operand; Value, Method receiver, Closure captures/me, args order, duplicates, and legacy None parity are covered by the shared tests. Escape, value-consumer, ownership, and query now reuse the occurrence projection where their separate policies allow it.
R4c task (not selected): `MIR-CALL-JOINIR-CALLER-LIFECYCLE-BOUNDARY-D1` must first name a live merge caller, classify both lifecycle owners, and prove that `Callee::rewrite_value_operands` can replace the local remap without changing retention policy. Until then JoinIrIdRemapper's duplicate match is parked; no code or production switch.
R6 D0/D1: the four-boundary schema decision is accepted as a design stop;
`MIR-CALL-R6-CORE-SCHEMA-D1` now owns the finite reader/writer cut/compat/park
matrix. No field deletion or type cleanup is selected until D1 names every
issuer and acceptance edge.
Backend exact-Extern D0 is the selected row; its D0-A/B/C/D tasks and five NoSafeSlice conditions are recorded above. Backend-shape strict-adapter I0 and native D1/C owner split are closed.
Post-R7 cleanup: normal-root mode/projection sum, MainObserved naming, identity-based syntax loan, and builder.rs production/compatibility/test barrel census remain separate cleanup rows; PyVM/reference/Python/native_driver remain ParkedSealed.
```

R5 row rules: each task owns one old edge and reuses the shared corridor
guard; no new per-row shell guard, no fixture-only acceptance, and no R6 field
editing. The census includes direct callers, dynamic construction, wire
writers, and selected backend preflight; literal `callee: None = 0` is not
sufficient evidence.

R6 core-schema design-stop audit (D0 accepted; D1 selected):

The worker premise audit closed the four authority boundaries without opening a
production switch. The final physical core remains:

```text
Call { dst: Option<ValueId>, callee: Callee, args: Vec<ValueId>, effects }
```

Six-line brief:

```text
Decision: keep the mandatory-Callee physical Call; MirCall/CallFlags are not
  semantic authority unless a non-default flag consumer is found.
Source authority + canonical issuer: route resolver/CallTarget issues Callee
  once; MirInstruction::call is the sole physical issuer; NewBox/NewClosure
  owners issue construction, not calls.
Non-authority: func/INVALID, Option/Callee defaults, Method(None), target Const,
  flags:{}, wire text, printer output, optimizer/backend lookup or retry.
Fail-fast boundary: before block.add_instruction and before wire/object publish;
  missing receiver, malformed construction, unresolved legacy, and profile
  mismatch reject with no alternate-target retry.
Smallest next slice: MIR-CALL-R6-CORE-SCHEMA-D1, a finite reader/writer census
  and cut/compat/park matrix; no code, fixture, or field deletion.
Non-claims: R6 implementation, JoinIR remap, JSON wire change, backend switch,
  PyVM/reference/Python/native_driver, and warning cleanup.
```

Finite D0 boundary: selected Rust `CallTarget`/resolver issuance ->
`MirInstruction::Call` -> selected JSON-v1/v0 bridge -> Rust interpreter and
selected backend terminals. A fresh HEAD census finds 20 direct
`MirInstruction::Call` literals plus the one canonical helper definition
(`MirInstruction::call`): **21 writer definitions**. Helper callers are tracked
by owner family below and are not double-counted as literals. Within this
boundary, possible `callee: None` publication is **0**: the three historical
Program JSON-v0 literals were removed before HEAD and MIR JSON-v0 resolves or
rejects before publication. The old 49/4 numbers are historical ledger values,
not current completion evidence. The boundary excludes
PyVM/reference/Python/native_driver, C native D0, the JoinIR lifecycle boundary,
and the complete Constructor ABI; a literal `callee: None` search alone still
does not classify typed stale `func` or `Method(None)` edges.

Current direct-writer ledger (no duplicate helper-caller counting):

```text
canonical helper: src/mir/instruction/methods.rs:22
builder literals: builder_emit.rs:140; calls/build.rs:357;
  calls/materializer.rs:57,95; calls/unified_emitter/compat_entrypoints.rs:22,42;
  calls/unified_emitter/physical_terminal.rs:94;
  exprs_call.rs:33; normal_module_transaction/physical_thunk.rs:79;
  ssa/phi_input_materializer/edge_rematerialization.rs:240;
  utils/boxcall_emit.rs:231
shared/direct literals: ssot/method_call.rs:71; ssot/extern_call.rs:26;
  canonical_direct_call.rs:78
compat ingress literals: runner/mir_json_v0/module.rs:453;
  runner/json_v1_bridge/parse/mir_call.rs:90,159,265,290,315
helper ingress families: runtime_method_call; extern wrapper; JSON-v0
  lowering expression/statement/module helpers; each resolves an exact Callee
  before the helper publishes a Call and is counted only once at its writer.
```

Helper-family owner anchors are finite and explicit: `collection_literals.rs`,
`exprs_qmark.rs`, `rewrite/special.rs`, `calls/emit.rs`, `decls.rs`,
`control_flow/plan/lowerer/effect_emission.rs`, `new_expression.rs`,
`function_call_preflight_route.rs`, `print_stmt.rs`,
`debug_method_routing.rs`, `task_scope_stmt.rs`, `method_call_terminal.rs`,
`json_v0_bridge/lowering/expr/call_ops.rs`, `expr.rs`, `stmts.rs`,
`expr/block_expr.rs`, and `mir_json_v0/module.rs`. Any new helper caller is a
cutover blocker rather than an implicit extension of this ledger.

D1 consumer/terminal ledger (the 21 writers above are not re-counted here):

| owner | observed edge | disposition |
|---|---|---|
| `unified_emitter/physical_terminal.rs:75-103` | rebuilds `func: INVALID` + `Option<Callee>` and drops flags | cut; thin `MirInstruction::call` becomes the sole issuer |
| `instruction/methods.rs:15-28,209-225` | typed operand projection plus legacy `None -> func` | retain typed projection; cut legacy branch |
| `passes/callsite_canonicalize/pass.rs:94-143` | Closure -> `NewClosure`; `None+func` -> `Global` | retain construction boundary; cut target inference |
| `passes/simplify_cfg/flow.rs:486-498`, `ownership_ssa/verify.rs:112-147` | legacy `func` target/operand readers | cut after typed projection owner is named |
| `passes/cse.rs:102-116`, `optimizer/core.rs:243-255` | `call_legacy_{func}` key reconstruction | cut on selected path; diagnostic-only readers park |
| `query.rs:97`, `value_consumer.rs:104`, `escape_barrier.rs:63-80` | Callee occurrence projection | retain; policy remains owner-local |
| `mir_json_emit/root.rs:26-84,112-117` | schema/profile selector and contradiction rejection | retain as profile authority, never target authority |
| `mir_json_emit/emitters/mod.rs:313-340` | forwards `func` + `Option<Callee>` | V0 facade compat only; canonical path cut |
| `mir_json_emit/emitters/calls.rs:9-203` | V1 typed projection; V0 legacy wire and `receiver.unwrap_or(func)` | V1 reject/retain; V0 compat; receiver fallback reopen |
| `mir_json_emit/helpers.rs:25-102` | typed serialization, nullable receiver, unconditional `flags:{}` | typed projection retain; Method(None)/flags reopen |
| `mir_json_v0/call.rs:8-130,248-267`, `catalog.rs:6-127` | explicit/name/func one-shot resolve or typed reject | owner-private compat; no missing Call publication |
| `json_v1_bridge/parse/mir_call.rs:15-327` | typed ingress, dummy `func=0`, Constructor/NewBox, Closure split | ingress retain; dummy cut; construction reopen |
| `mir_json_v0/module.rs:420-475`, `common_util/core_bridge.rs:148-194` | compatibility Method and post-wire Const reclassification | compat/park; no selected semantic authority |
| `backend/mir_interpreter/handlers/calls/{mod,method}.rs` | missing-callee preflight; Method(None) registry/by-name recovery | preflight retain; recovery reopen/cut |
| `contracts/backend_core_ops/allowlists.rs`, `runner/product/llvm/mod.rs` | None/Closure preflight before selected native object | retain as negative guard; shrink after core cut |
| `lang/c-abi/shims/*mir_call*`, `constructor_call_route_plan.rs` | C by-name constructor/method classification; dual Constructor route | park/reopen; no new downstream authority |
| `joinir_id_remapper.rs`, `edge_rematerialization.rs` | `func`/`Option` transport and rewrite | Parked NoSafeSlice until caller/lifecycle owner exists |

This ledger is the D1 cut/compat/park inventory. A path that cannot be placed
in one row is a `CutoverBlockerOpen`, not a reason to widen the boundary or
restore a default target.

D1 is not yet accepted. The open blockers are typed stale-`func` observation
(`user_box_method_publication.rs` included), real `Method(None)` producers and
receiver recovery, public `MirCall`/`CallFlags` with lossy non-default flags,
Closure V0/runtime parity, and the dual Constructor/NewBox physical route.
JoinIR/SSA rematerialization stays `ParkedSealed` only because its caller and
lifecycle boundary is outside this selected R6 census; its observable reopen
trigger is a live merge caller plus named lifecycle owner. It is not evidence
for or against the selected core cutover.

Four-boundary design consultation (D1 policy; implementation still stopped):

| boundary | selected policy / canonical issuer | compatibility and fail-fast |
|---|---|---|
| `MirCall` / `CallFlags` | `CallTarget`/`CalleeResolver` issues target; `MirInstruction::call` issues physical Call. Keep public `MirCall`/`CallFlags` as owner-local transport until API consumers are proven absent. | default flags are discarded decoration; non-default or conflicting flags reject before `physical_terminal`; flags never classify Constructor/Closure |
| `Method(None)` | verified static callable catalog (identity + exact arity) issues qualified `Callee::Global`; instance calls issue `Method { receiver: ValueId }`. Existing `current_static_box` text and formatter are not authority. | JSON-v0 optional receiver resolves once or rejects; methodize and VM receiver/args[0] recovery are cutover blockers; no `StaticMethod` variant |
| Closure / Value | descriptor (`params/captures/me`, `dst`, empty construction args) issues `NewClosure`; an existing closure value issues `Callee::Value(ValueId)`. | Closure wire is compatibility projection only; mixed descriptor/value, missing dst, runtime args, or `Call(Callee::Closure)` at selected terminal rejects before publication |
| Constructor / NewBox | constructor syntax/resolver/JSON verifies `box_type`, `dst`, arity and args, then the existing `NewBox` owner publishes construction. `Callee::Constructor` is pre-core/compat input only. | V0 Constructor-shaped Call resolves once to `NewBox` or rejects; backend/C name scans and `Call(Constructor)` execution are forbidden |

The matrix is a policy Decision, not implementation evidence. The four
production issuers are still open: Method(None) has real builder/V0 producers,
Closure V0 parser/runtime parity is incomplete, Constructor has V0 Call and
V1/NewBox routes, and public `MirCall`/`CallFlags` can be observed outside the
selected builder. Therefore R6a remains a `NoSafeSlice` until the following
bounded rows close their own old edge and acceptance:

```text
D1-A  CallFlags public/selected-consumer census; non-default conflict reject
D1-B  verified static catalog -> qualified Global; Method(None) issuer count 0
D1-C  Closure descriptor/value wire discriminator and NewClosure parity
D1-D  Constructor input -> NewBox sole selected physical owner
```

Each row must keep the same fail-fast boundary (before block/wire/object
publication), add no new semantic receipt, and preserve the selected
non-closure/non-constructor call parity. A row is not accepted merely because
its local test passes: its production issuer count, old-edge deletion, and
positive/negative/parity evidence must be observable in the shared corridor
guard. If a public flag consumer, third Constructor owner, ambiguous Closure
wire, or missing static catalog is found, the row returns to `NoSafeSlice`.

| state / edge | source authority | D0 terminal classification |
|---|---|---|
| `Global`, `Extern`, `Value`, `Method(Some(receiver))` | route resolver / `CallTarget` | canonical callable; preserve target operands then ordered args |
| `Method(None)` | legacy/static producers and v0 projection | compatibility/transitional only; static must become qualified `Global`; core reject until migrated |
| `Callee::Closure` with `dst=Some,args=[]` | closure construction owner | pre-core input -> `NewClosure`; runtime closure call is `Callee::Value` |
| Closure with runtime args or missing destination | no valid construction relation | typed reject before publication |
| `Constructor` | constructor resolver / route family | pre-core input until one `NewBox` or typed-Call owner is selected; no generic reclassification |
| `NewBox`, `NewClosure` | existing construction owners | independent physical instructions; not Call target fallback |
| `None + legacy name/func` | JSON-v0 owner-local catalog | resolve once to exact `Callee` or typed reject; never publish missing Call |
| missing, `INVALID`, duplicate, foreign, non-String, malformed explicit target | none | typed reject before block/wire/object publication; no retry |
| `MirCall.flags` / `CallFlags` | current builder intermediate only | physical terminal consumes no flag; non-default consumer would reopen the design |

Exact edge classification for D1:

```text
cut       = semantic func readers, Option<Callee> terminal readers, target-Const
            reconstruction, Method(None) recovery, and any backend/optimizer
            by-name fallback once their canonical replacement is named;
compat    = owner-local JSON-v0 raw draft/catalog only, with one resolve point;
park      = JoinIR remap until a live caller and both lifecycle owners are named;
reopen    = public MirCall/CallFlags use, non-default flag semantics, a second
            Constructor/NewBox issuer, or a Closure wire/runtime consumer.
```

The audit found `MirCall.flags` is created and then discarded by the physical
terminal; no production field reader was found. This permits a future private
staging cleanup, but does not authorize deleting the public `MirCall`/
`CallFlags` exports in D1. `Method(None)` still has real producers and a
by-name/recovery consumer, while Closure and Constructor have split V0/V1
construction paths. Those are schema blockers, not mechanical compiler fixes.

Selected next design task: `MIR-CALL-R6-CORE-SCHEMA-D1`. It must attach the 21
writer definitions, the named helper families, and the zero missing-callee
publication state to `cut`, `compat`, `park`, or `reopen`; name the single
issuer for Method/static, Closure/NewClosure, and Constructor/NewBox; and
record positive/negative/parity acceptance before any R6a implementation.
The current blocker is typed stale-`func`/Method(None), public
MirCall/CallFlags semantics, and dual construction ownership—not a missing
callee literal. Until D1 is accepted, do not change `Option<Callee>`, `func`,
`Method(None)`, `Callee::Closure`, `MirCall`, or `CallFlags` in code.

Post-R7 normal-root cleanup (parked, separate lane):

```text
NORMAL-ROOT-PROJECTION-SUM-D0
  combine mode + root projection into one App/ProgramRuntime sum; remove bool drift checks
NORMAL-ROOT-SOURCE-NOMENCLATURE-D0
  Parser MainObserved -> compiler static-main admission -> App; rename the source-backed lowering view
NORMAL-ROOT-SYNTAX-LOAN-D0
  replace index/name AST rediscovery with an admitted opaque identity loan
```

These rows do not reopen normal-root production or the 2000-line manifest.

retirement seriesの固定順は次。

```text
R1  exact qualified Program JSON-v0 producers
R2  owner-private MIR JSON-v0 input state
R3  Program/MIR JSON-v0 exact pre-core resolution; late issuer retirement
R4  Callee operand/remap SSOT and semantic consumer migration
R5  optimizer/interpreter/printer/JSON/selected backend terminal closure
R6  Call core schema atomic cutover
R7  impossible-state guard and docs/reference closeout
```

loader、interpreter、optimizer、printer/JSON、backend、fixture/reference callerが
R2-R5で閉じるまでR6を選ばない。valid explicit calleeはlegacy decorationより優先し、
malformed explicit callee、missing/ambiguous/non-String/foreign legacy relationはCall
publication前にtyped rejectする。reject後に別target sourceをretryしてはならない。

active rowの選択は`CURRENT_STATE.toml`とrolling workstream cardだけが行う。
R1より後の行は前行のevidenceなしに自動起動しない。cleanup上位順は
`mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md`が保持する。

## 作業分離ルール

- BoxShape lane と BoxCount lane を混ぜない
- DebugLog/Nop/NewBox/NewClosure はこの lane で触らない
- 1コミットに 1変換（または 1契約）だけを入れる
- fast gate FAIL 状態で新しい fixture/case を増やさない

## 受け入れコマンド（最小）

```bash
cargo test instruction_diet_ledger_counts_match_docs_ssot -- --nocapture
cargo test mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture
cargo check --bin hakorune
```

必要に応じて:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## 作業役へのハンドオフ文（そのまま貼れる版）

「MIR call-site canonicalization lane を MCL-0 から順に実施してください。  
各コミットは 1タスクのみ。BoxShape 専用で、受理拡張（BoxCount）は禁止。  
`BoxCall/ExternCall -> Call(callee=...)` を backend 手前で正規化し、backend 入口で legacy 残存を fail-fast 化してください。  
NewBox/NewClosure/DebugLog/Nop の整理はこの lane の対象外です。」
