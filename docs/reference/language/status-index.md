# Language feature status index

Status: Current navigation / evidence index
Decision: `LANGUAGE-DOCS-STATUS-SSOT-D0`
Scope: discoverability only; this page does not create grammar or runtime meaning.

## How to read this page

Grammar status and support status are separate axes.

```text
grammar:
  canonical | compatibility_only | reserved | rejected | topic_owned

availability/profile:
  live | guarded | transport | scaffold | metadata-only | pending | deferred |
  prohibited | historical | status_conflict

`topic_owned`, `scaffold`, and `status_conflict` are navigation-index values:
they describe ownership or unresolved availability, not additional parser
acceptance. A row may use `status_conflict` on either axis while the decision
stop is open.

During an accepted authority migration, prose such as `canonical target` or
`authority_sync_pending` records the selected destination while
`status_conflict` continues to describe the unsynchronized physical
registry/parser/docs. Those annotations never mean `live`.

`accepted target / production 0` means that the language meaning and its
authority are fixed, but no production issuer or consumer is active yet. It
does not authorize implementation, backend parity, or a compatibility
fallback.

Only bracketed values such as `[freeze:contract][parser/throw_reserved]` are
existing implementation tags. Other gate/reject values in the table are
`index:` labels for this documentation stop and must not be passed to a parser
or runtime as if they were active switches.
```

Authority order:

1. [semantic contract charter](semantic-contract-charter.md),
   [semantic kernel](semantic-kernel.md), [grammar contract](grammar-contract.md), and
   [`grammar/language-v1-registry.toml`](../../../grammar/language-v1-registry.toml)
   own spelling and normalization.
2. Topic SSOTs own semantic meaning and boundary law.
3. [Stage profiles](stage-profiles.md) own Stage0/Stage1 support.
4. [Concurrency semantics](../concurrency/semantics.md) and
   [boundary model](../concurrency/boundary-model.md) own concurrency profiles.
5. Historical documents are evidence only.

Parser acceptance and examples are implementation evidence, not authority.
When rows disagree, keep `status_conflict` and open a design decision; do not
silently select the most permissive document.

Historical examples for every row are collected in
[LANGUAGE_REFERENCE_2025.md](LANGUAGE_REFERENCE_2025.md); individual rows do
not inherit permission from that snapshot.

## Current index

| Feature | Grammar status | Availability/profile | Authority / evidence | Gate/reject tag | Promotion/retirement | Current note |
| --- | --- | --- | --- | --- | --- | --- |
| bare `panic(String)` | ordinary call grammar; semantic-reserved target | accepted target / production 0 | [failure/outcomes](failure-outcome-relations.md), [call evaluation](function-call-evaluation.md), semantic kernel | `index:panic-terminal-fault-production-missing` | typed source issuer -> common exit transaction -> generic Fault terminal -> selected runtime ABI -> old name route retirement | `panic(String) -> Never` is noncatchable terminal Fault; Never is not source-spellable, cleanup still runs, and bare exit is separate. |
| `throw` | `status_conflict` | `prohibited` | [Result/exit C′](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md), EBNF | `[freeze:contract][parser/throw_reserved]` | source/parser producers zero; generated/internal shapes separately fenced | Rejected in both target profiles; physical retirement pending. |
| statement `try` | `status_conflict` | `status_conflict` | [Result/exit C′](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md), grammar contract, registry | `[freeze:contract][parser/try_reserved]` | reject both profiles; registry/parser sync pending | Current Compat parser acceptance is migration drift, not permission. |
| postfix Result `?` | `status_conflict` | `pending` | [Result/exit C′](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md), [option](option.md), semantic kernel | `index:typed-result-qmark-production-missing` | verified propagation/exit plan, exact consumer, legacy QMark retirement, DOC0 | Accepted only for `Result<T,E>` inside `Result<U,E>` with exact `E`; Option/custom Try/implicit conversion rejected. |
| postfix `catch` | `status_conflict` | `prohibited` target / transport drift | [Result/exit C′](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md), semantic kernel, registry | current ambient gates are evidence only | source/carrier/runtime retirement in C′ R0 | Catch and `RecoverableFailure` are rejected target surfaces; July decision is historical. |
| postfix `cleanup` | `status_conflict` | retirement pending | [scope-exit](scope-exit-semantics.md), EBNF, registry | current syntax-3 gates are evidence only | C′ R0 | Not canonical; migrate to standalone cleanup. |
| standalone `cleanup { ... }` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), [Result/exit C′](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md) | `index:cleanup-production-missing` | dedicated AST/exit plan and backend rows | Sole accepted lexical cleanup target; production not synchronized. |
| `local x = e cleanup { ... }` | `status_conflict` | retirement pending | [scope-exit](scope-exit-semantics.md), EBNF, registry | `index:local-cleanup-retirement-pending` | C′ R0 | Not canonical; declaration sugar must not create a second registration rule. |
| scope `fini { ... }` / `local ... fini` | `status_conflict` | retirement pending | [scope-exit](scope-exit-semantics.md), grammar contract, registry | `index:fini-alias-authority-sync-pending` | retire before Box-hook activation | Historical cleanup aliases only; not a retained Compat authority. |
| Box-member `fini { ... }` | `topic_owned` | accepted target / production 0 | [lifecycle](lifecycle.md), [C′ lifecycle](../../development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md) | `index:cprime-fini-hook-production-missing` | Home C′ I0/R0 + DOC0 | Non-callable terminal Home hook; parent hook precedes reverse verified-owning-field release. |
| direct `box.fini()` / `fini(...)` method | `status_conflict` | retirement/rejection pending | [lifecycle](lifecycle.md), C′ lifecycle | `index:direct-fini-retirement-pending` | caller/catalog/parser retirement in Home C′ R0 | Rejected target; `close()`/`shutdown()` remain ordinary methods. |
| `co` / `nowait` / `await` | `topic_owned` | `guarded` concurrency profile | [concurrency semantics](../concurrency/semantics.md), stage profiles | `concurrency-profile-required` | Separate concurrency decision | Not a general Stage1/selfhost promise; no Language v1 registry row currently. |
| `Channel<T>` | `topic_owned` | `scaffold`/`deferred` by route | [concurrency semantics](../concurrency/semantics.md), stage profiles | `channel-route-gated` | Separate concurrency decision | Reference queue surface; broad Program/MIR/LLVM use remains gated. |
| `sync box` / `context` / `task_scope` | `topic_owned` | `scaffold`/`deferred` | [concurrency boundary](../concurrency/boundary-model.md), stage profiles | `sync-boundary-gated` | Separate concurrency decision | Do not infer parser-live or broad runtime support from reference docs. |
| `worker_scope` / `parallel` / raw `thread` / `lock<T>` / `worker_local` | `reserved`/`topic_owned` | `prohibited`/`deferred` | [stage profiles](stage-profiles.md), concurrency docs | `concurrency-substrate-only` | New language decision required | Design/substrate surfaces; no language-core activation. |
| Home ownership: declaration `take`, result `from`, expression `share` | `topic_owned` | take/share accepted target; bounded Rust forms connected (see [Ownership forms measured](#ownership-forms-measured)); from provisional; Hako parity unverified | [ownership](ownership.md), [Home syntax D0](../../development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md), stage profiles | existing inactive-ownership reject boundary | release source I0 -> take declaration I0 -> capability/Home ABI rows; share after representation D0 | `take` is declaration-only contextual; `share` is a non-group postfix prefix. Neither is globally reserved; `share(...)` remains an ordinary call. Former `move/view/shared` syntax is historical. |
| contextual `release root` | `topic_owned` | parser/source carrier live; semantics 0 | [ownership](ownership.md), [Home syntax D0](../../development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md), [release I0](../../development/current/main/investigations/own-home-release-source-i0-implementation-task-2026-08-09.md) | `index:home-release-semantics-missing` | root capability/Home Flow I0 + reference receipts | Dedicated exact-root AST/source row only. `release(value)` remains an ordinary call; `release` is not globally reserved; no Home is ended until later semantic/physical rows close. |
| `@rune CallableContract(query)` | `topic_owned` | accepted target / production 0 | [callable contracts](callable-contracts.md), [runes](runes.md) | `index:callable-contract-query-production-missing` | ordered Box-method inventory -> parser parity -> declared contract -> body conformance | Whole-call query obligation; signature owns types/arity, physical ABI remains downstream. Existing `Contract(pure|readonly)` is not silently promoted. |
| Box method namespace / name collision | ordinary method declaration and delegate exposure grammar | accepted target; direct duplicate rejection live; delegate forwarding production 0 | [callable contracts](callable-contracts.md), [call evaluation](function-call-evaluation.md), [delegation](field-visibility-and-delegation.md), EBNF | `index:delegate-forwarding-production-missing` | exact resolver relation -> unique delegate forwarder relation -> Builder/MIR consumer | One flat visible-method namespace per nominal Box. No Box-method overload by arity or static/instance kind; aliases must be unique. Same names on different Boxes are valid. FreeStatic `name/arity` identity is separate. |
| ordinary unannotated declared-instance call effect | ordinary method-call grammar | accepted `OpaqueObservable` effect / resolver facts issuer landed / physical production 0 | [callable contracts](callable-contracts.md), [function result](function-exit-and-entry-result.md) | `index:declared-instance-call-effect-production-missing` | exact resolver declaration relation -> source effect issuer -> package carriage -> physical projection | Explicit no-annotation default on the effect axis only. Query takes precedence; MIR/backend observations and the Dynamic envelope cannot issue it. |
| ordinary Dynamic member invocation | `topic_owned` | selector-independent contract accepted; caller-zero semantic-envelope issuer live; production consumer 0 | [Dynamic invocation](dynamic-invocation.md), [failure/outcomes](failure-outcome-relations.md), [ownership](ownership.md) | `index:dynamic-invocation-envelope-production-consumer-missing` | Recipe Dynamic value/CallSlot -> provider admission/plan -> physical canary/cutover | One `OpaqueObservable`, synchronous non-detached but `MaySuspend`, borrowed-input, self-contained-result, Normal-or-Fault contract. Runtime tags and selectors do not choose semantics; no retry/fallback. |
| Dynamic `+` / `<` operator contracts | `topic_owned` | contract, canonical issuer, and exact V9/V17 lifecycle co-seal live; physical consumer 0 | [Dynamic operators](dynamic-operators.md), [types](types.md), [ownership](ownership.md) | `index:dynamic-carrier-ingress-lifecycle-missing` | initial V1/B0 ingress disposition -> rebind/cleanup -> physicalization | Add borrows operands and publishes a non-aliasing carrier only on Normal; Less publishes trivial Bool. Fault publishes no result or rebind. Initial plain-parameter ingress may not inherit V17's end obligation. Runtime/provider inference and invocation-envelope reuse are forbidden. |
| `Option<T>` / `Result<T,E>` constructors | `canonical` narrow rows | `live` narrow enum/prelude profile | [option](option.md), EBNF, stage profiles | `option-result-narrow` | Expand only by explicit capability row | Constructors are live independently; typed Result `?` remains pending and Option `?` is rejected. |

## Ownership forms measured

Measured on commit `c1739a89c2` (2026-11-05) with the `cargo quick-build --bin hakorune` binary, one source per row, `--backend mir --emit-exe`, then running the EXE. Only the Rust route is measured: the Hako parser cannot be run today (`cargo build --features vm-reference` does not compile at this commit: E0004 in `src/backend/mir_interpreter/handlers/type_ops.rs`; see also the Hako parser line in `docs/development/current/main/investigations/app-boxtorrent-mini.md`), and the `.hako` parser change for `Shared(...)` is source-only.

Results: `connected` = the EXE is built and its exit and output match the expectation (for a row with no accept/reject expectation, it is built and ran; a `reject from 3b` row that still builds is also `connected`, because ADR-002 keeps the current behaviour until 3b); `refused-named` = the refusal text names the form or its cause; `refused-generic` = refused, but the text does not name the form (`artifact-source-unavailable`, `ResolverDeferred`, `unsupported-cohort` and similar); `parser-refused` = the parser stops; `against-frozen` = the frozen grammar rejects the form (or expects another result) and the compiler accepts it. A row whose frozen expectation is `reject` and whose result is any refusal is a correct refusal. Basis: `L56`/`L58` are lines of ADR-002, `rule N` is a numbered rule of the frozen grammar block, `3a`/`3b` are migration steps. Every row is a small standalone program; several rows combine a form with the smallest surrounding code that can run, so a generic refusal can come from that surrounding code and the row says what it spells.

**The frozen grammar accepts 129 of these 149 forms; 34 of those 129 are connected.** The other rows are rejections (16), rules that change at 3b (3) and one open form (1: `from a, b`, whose acceptance ADR-002 still leaves open).

| Group | connected | refused-named | refused-generic | parser-refused | against-frozen | total | accepted by the frozen grammar | of those connected |
|---|---|---|---|---|---|---|---|---|
| `move` | 3 | 8 | 1 | 4 | 0 | 16 | 11 | 3 |
| declaration `take` | 1 | 3 | 2 | 0 | 0 | 6 | 5 | 1 |
| expression `take` | 0 | 0 | 6 | 0 | 0 | 6 | 5 | 0 |
| `edit` | 0 | 3 | 4 | 0 | 0 | 7 | 5 | 0 |
| `share` | 3 | 5 | 2 | 1 | 0 | 11 | 9 | 3 |
| `copy` | 1 | 3 | 1 | 0 | 0 | 5 | 4 | 1 |
| `from` | 0 | 2 | 1 | 1 | 0 | 4 | 3 | 0 |
| `T?` | 1 | 1 | 5 | 0 | 0 | 7 | 7 | 1 |
| field types (store / read / write after publication) | 16 | 1 | 25 | 0 | 0 | 42 | 42 | 16 |
| mixed field types in one box | 3 | 2 | 2 | 0 | 0 | 7 | 7 | 3 |
| object field reads | 0 | 0 | 3 | 0 | 0 | 3 | 3 | 0 |
| `new T { f: x }` construction | 0 | 1 | 3 | 0 | 0 | 4 | 2 | 0 |
| pattern and `match` | 0 | 0 | 2 | 2 | 0 | 4 | 4 | 0 |
| function types and closures | 0 | 0 | 0 | 3 | 0 | 3 | 3 | 0 |
| containers | 0 | 1 | 3 | 0 | 0 | 4 | 4 | 0 |
| rules that change in 3b | 0 | 0 | 4 | 0 | 0 | 4 | 2 | 0 |
| String field reads | 6 | 1 | 7 | 0 | 2 | 16 | 13 | 6 |
| **all** | 34 | 31 | 71 | 11 | 2 | 149 | 129 | 34 |

### `move`: 3 connected of 16 (3 of the 11 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| local y = move x | accept, exit 3 [rule 2] | connected | `exit 3` |
| y = move x | accept, exit 4 [rule 2] | refused-named | `move-not-supported` |
| f(move x) to take param | accept, exit 5 [rule 4] | connected | `exit 5` |
| new H(move x) with birth(take) | accept, exit 0 [rule 2] | connected | `exit 0` |
| new H(move x) inside if | accept, exit 0 [rule 2] | refused-named | `move-not-supported` |
| me.f = move q in birth(q: P) (plain formal, not take) | reject [ownership.md:59 (user decision 2026-11-05: move into a plain formal is refused; tag BirthStoreOwnershipMismatch)] | refused-named | `BirthStoreOwnershipMismatch` |
| h.keep = move y (after publication) | accept, exit 0 [L63 (owned field assignment is a transfer point), rule 6, L58 item 4] | refused-named | `move-argument-unplanned` |
| return move x | accept, exit 6 [rule 5] | refused-named | `move-not-supported` |
| (move x).m() consuming receiver | accept, exit 7 [rule 4] | refused-named | `UnsupportedReceiverTransfer` |
| match move e | accept, exit 7 [L57 (match move consumes)] | refused-generic | `ResolverDeferred` |
| move x.f (direct field of owner) | accept, exit 8 [rule 6] | parser-refused | `Unexpected token DOT, expected [freeze:contract][parser/move_exact_root_required] move req` |
| e = move n.left (replacement from inside) | accept, exit 0 [rule12] | parser-refused | `Unexpected token DOT, expected [freeze:contract][parser/move_exact_root_required] move req` |
| move x? (error) | reject [L56] | parser-refused | `parse` |
| move img.intoBytes() (error) | reject [L56] | parser-refused | `Unexpected token DOT, expected [freeze:contract][parser/move_exact_root_required] move req` |
| second move of a moved local | reject [rule 2] | refused-named | `UseAfterMove` |
| read after move | reject [rule 2] | refused-named | `UseAfterMove` |

### declaration `take`: 1 connected of 6 (1 of the 5 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| birth(take x: T) stored in field, move local | accept, exit 0 [3a] | connected | `exit 0` |
| birth(take x: T) stored in field, fresh new T actual | accept, exit 0 [rule 3: fresh value passes directly] | refused-generic | `argument-source-unavailable` |
| birth(take x: T) not stored | accept, exit 0 [3a] | refused-named | `TakeFormalNotStored` |
| ordinary method take param | accept, exit 1 [rule 4] | refused-generic | `borrowed-view/declared-object-class` |
| receiver take m() declared, never called | accept, exit 0 [L56] | refused-named | `UnsupportedReceiverTransfer` |
| receiver take m() called plainly x.m() | reject [rule 4 (implied)] | refused-named | `UnsupportedReceiverTransfer` |

### expression `take`: 0 connected of 6 (0 of the 5 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| take x (nullable local) | accept, exit 2 [L56] | refused-generic | `misparsed as identifier take + separate statement; then ResolverDeferred` |
| take me.f | accept, exit 0 [L56] | refused-generic | `misparsed as identifier take + separate statement; then ResolverDeferred` |
| take a[i] | accept, exit 0 [L56 (named-refuse until connected)] | refused-generic | `misparsed as identifier take + separate statement; then ResolverDeferred` |
| return take me.f | accept, exit 0 [L56] | refused-generic | `misparsed as identifier take + separate statement; then ResolverDeferred` |
| take(p) ordinary call | accept, exit 3 [L56] | refused-generic | `artifact-source-unavailable` |
| take on non-nullable T place | reject [L56] | refused-generic | `misparsed as identifier take + separate statement; then ResolverDeferred` |

### `edit`: 0 connected of 7 (0 of the 5 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| edit param declared, never called | accept, exit 0 [rule 4] | refused-named | `UnsupportedTransfer` |
| rewrite(edit x) | accept, exit 5 [rule 4] | refused-generic | `ResolverDeferred` |
| receiver edit m() | accept, exit 5 [rule 4] | refused-named | `UnsupportedReceiverTransfer` |
| edit me.s[i] as argument | accept, exit 0 [L58 item 5, L57 (edit me.stmts[i])] | refused-generic | `ResolverDeferred` |
| e = make(v) through edit param | accept, exit 9 [L56] | refused-generic | `ResolverDeferred` |
| e = fold(move e) (error) | reject [L56] | refused-generic | `ResolverDeferred` |
| call omits edit for an edit param | reject [rule 4] | refused-named | `UnsupportedTransfer` |

### `share`: 3 connected of 11 (3 of the 9 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| share new T() | accept, exit 3 [rule 2] | connected | `exit 3` |
| share new T() then call a method on the Shared local | accept, exit 3 [rule 2/rule3 (read through handle)] | refused-generic | `root-instance-call` |
| share move x | accept, exit 3 [rule 2] | connected | `exit 3` |
| share h (increase) | accept, exit 3 [rule 2] | connected | `exit 3` |
| share f() (function result) | accept, exit 3 [L56] | refused-named | `UnsupportedResultAnnotation` |
| share x of plain Box (error) | reject [rule 2] | refused-named | `SharedOperandUnsupported` |
| share(x) ordinary call | accept, exit 3 [L56] | refused-generic | `artifact-source-unavailable` |
| Shared<T> parameter | accept, exit 3 [3a bound excluded] | refused-named | `UnsupportedDeclaredType` |
| Shared<T> result | accept, exit 3 [3a bound excluded] | refused-named | `UnsupportedResultAnnotation` |
| Shared<T> field | accept, exit 0 [3a bound excluded] | refused-named | `SharedOperandUnsupported` |
| Shared(new T()) retired | reject [rule 2] | parser-refused | `Unexpected token IDENTIFIER("Shared"), expected [freeze:contract][parser/shared_constructo` |

### `copy`: 1 connected of 5 (1 of the 4 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| copy of i64 | accept, exit 4 [rule 2] | connected | `exit 4` |
| copy of Bool | accept, exit 1 [rule 2] | refused-named | `copy_contract_unsupported` |
| copy of String | accept, exit 0 [rule 2] | refused-named | `copy_contract_unsupported` |
| copy of Box without copy contract | reject [rule 2] | refused-named | `copy_contract_unsupported` |
| copy(x) ordinary call | accept, exit 4 [rule] | refused-generic | `artifact-source-unavailable` |

### `from`: 0 connected of 4 (0 of the 3 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| get(): T from me | accept, exit 5 [rule5] | refused-named | `UnsupportedResultAnnotation` |
| f(a: P, b: P): P from a | accept, exit 6 [rule5] | refused-generic | `nullable-ordinal` |
| from a, b | undecided [L104: starting to accept `from a, b` is still open (後続 row)] | parser-refused | `[freeze:contract][parser/from-support-unavailable] from requires exactly one receiver or p` |
| name(): String from me | accept, exit 0 [rule5] | refused-named | `UnsupportedResultAnnotation` |

### `T?`: 1 connected of 7 (1 of the 7 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| field T? set in birth | accept, exit 0 [3b] | refused-named | `TakeFormalDeclarationUnavailable` |
| formal T? with null actual | accept, exit 0 [3b] | refused-generic | `formal-declaration-unavailable` |
| literal null store in birth | accept, exit 0 [3b] | connected | `exit 0` |
| T? field without birth (default null) | accept, exit 0 [3b] | refused-generic | `instruction-unsupported` |
| read T? field | accept, exit 0 [rule/読み借用] | refused-generic | `formal-declaration-unavailable` |
| h.c == null | accept, exit 1 [] | refused-generic | `artifact-source-unavailable` |
| move after null check | accept, exit 2 [L56] | refused-generic | `EdgeArgumentsForbidden` |

### field types (store / read / write after publication): 16 connected of 42 (16 of the 42 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| i64 field: store via birth(f: i64) | accept, exit 0 [rule/field] | connected | `exit 0` |
| i64 field: read after store (birth(f: i64)) | accept, exit 3 [rule/field] | connected | `exit 3` |
| i64 field: write after publication (birth(f: i64)) | accept, exit 3 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| Bool field: store via birth(f: Bool) | accept, exit 0 [rule/field] | refused-generic | `artifact-source-unavailable` |
| Bool field: read after store (birth(f: Bool)) | accept, exit 1 [rule/field] | refused-generic | `artifact-source-unavailable` |
| Bool field: write after publication (birth(f: Bool)) | accept, exit 1 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| String field: store via birth(f: String) | accept, exit 0 [rule/field] | connected | `exit 0` |
| String field: read after store (birth(f: String)) | accept, exit 0 [rule/field] | connected | `exit 0` |
| String field: write after publication (birth(f: String)) | accept, exit 0 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| StringBox field: store via birth(f: StringBox) | accept, exit 0 [rule/field] | connected | `exit 0` |
| StringBox field: read after store (birth(f: StringBox)) | accept, exit 0 [rule/field] | connected | `exit 0` |
| StringBox field: write after publication (birth(f: StringBox)) | accept, exit 0 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| String? field: store via birth(f: String?) | accept, exit 0 [rule/field] | refused-generic | `artifact-source-unavailable` |
| String? field: read after store (birth(f: String?)) | accept, exit 0 [rule/field] | refused-generic | `print-expression` |
| String? field: write after publication (birth(f: String?)) | accept, exit 0 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| T field: store via birth(take f: P) | accept, exit 0 [rule/field] | connected | `exit 0` |
| T field: read after store (birth(take f: P)) | accept, exit 0 [rule/field] | refused-generic | `ResolverDeferred` |
| T field: write after publication (birth(take f: P)) | accept, exit 0 [3a excludes (public write)] | refused-generic | `ResolverDeferred` |
| T? field: store via birth(take f: P?) | accept, exit 0 [rule/field] | refused-named | `TakeFormalDeclarationUnavailable` |
| T? field: read after store (birth(take f: P?)) | accept, exit 0 [rule/field] | refused-generic | `ResolverDeferred` |
| T? field: write after publication (birth(take f: P?)) | accept, exit 0 [3a excludes (public write)] | refused-generic | `ResolverDeferred` |
| untyped field: store via birth(f) | accept, exit 0 [rule/field] | connected | `exit 0` |
| untyped field: read after store (birth(f)) | accept, exit 5 [rule/field] | refused-generic | `artifact-source-unavailable` |
| untyped field: write after publication (birth(f)) | accept, exit 5 [3a excludes (public write)] | refused-generic | `artifact-source-unavailable` |
| i64 field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | connected | `exit 0` |
| i64 field: read after store (unannotated birth(n)) | accept, exit 3 [rule/field] | connected | `exit 3` |
| i64 field: write after publication (unannotated birth(n)) | accept, exit 3 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |
| Bool field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | refused-generic | `artifact-source-unavailable` |
| Bool field: read after store (unannotated birth(n)) | accept, exit 1 [rule/field] | refused-generic | `artifact-source-unavailable` |
| Bool field: write after publication (unannotated birth(n)) | accept, exit 1 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |
| String field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | connected | `exit 0` |
| String field: read after store (unannotated birth(n)) | accept, exit 0 [rule/field] | connected | `exit 0` |
| String field: write after publication (unannotated birth(n)) | accept, exit 0 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |
| StringBox field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | connected | `exit 0` |
| StringBox field: read after store (unannotated birth(n)) | accept, exit 0 [rule/field] | connected | `exit 0` |
| StringBox field: write after publication (unannotated birth(n)) | accept, exit 0 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |
| String? field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | connected | `exit 0` |
| String? field: read after store (unannotated birth(n)) | accept, exit 0 [rule/field] | refused-generic | `print-expression` |
| String? field: write after publication (unannotated birth(n)) | accept, exit 0 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |
| untyped field: store via unannotated birth(n) { me.f = n } | accept, exit 0 [rule/field] | connected | `exit 0` |
| untyped field: read after store (unannotated birth(n)) | accept, exit 5 [rule/field] | refused-generic | `artifact-source-unavailable` |
| untyped field: write after publication (unannotated birth(n)) | accept, exit 5 [L58 item 4 / public write] | refused-generic | `artifact-source-unavailable` |

### mixed field types in one box: 3 connected of 7 (3 of the 7 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| String + i64 | accept, exit 3 | refused-generic | `IncompleteOrdinaryNewCoverage` |
| String + T | accept, exit 0 | refused-named | `TakeBirthSourceUnconnected` |
| String + T (plain birth, T stored by assignment) | accept, exit 0 | refused-generic | `object-field-set-invalid` |
| String + untyped (Text) field | accept, exit 0 | connected | `exit 0` |
| String + untyped holding an object | accept, exit 0 | refused-named | `move-argument-unplanned` |
| T + i64 | accept, exit 2 | connected | `exit 2` |
| untyped only holding Text, read print(a.v) | accept, exit 0 [ownership.md (Text-only untyped field)] | connected | `exit 0` |

### object field reads: 0 connected of 3 (0 of the 3 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| local x = h.keep | accept, exit 7 [rule/field read borrow] | refused-generic | `unsupported-cohort` |
| local x = h.keep; x.val() | accept, exit 7 [rule/field read borrow] | refused-generic | `root-instance-call` |
| h.keep.val() | accept, exit 7 [rule/field read borrow] | refused-generic | `artifact-source-unavailable` |

### `new T { f: x }` construction: 0 connected of 4 (0 of the 2 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| new T { f: x } | accept, exit 0 [L58 item 4, 2 example] | refused-generic | `OverridesNotCovered` |
| new T(a) { f: x } | accept, exit 0 [L58 item 4, 2 example] | refused-generic | `artifact-source-unavailable` |
| new T { } with a field missing | reject from 3b (current behaviour stays until then) [L58 item 4 (required-field check starts at 3b)] | refused-generic | `OverridesNotCovered` |
| new T { f: x, f: y } duplicate | reject [L58 item 4 (duplicate is an error)] | refused-named | `duplicate-field` |

### pattern and `match`: 0 connected of 4 (0 of the 4 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| ordinary match with _ arm (expression) | accept, exit 7 [L58] | refused-generic | `ResolverDeferred` |
| match on Option with variants Some(v) / None | accept, exit 3 [L57 (patterns), option.md] | refused-generic | `ResolverDeferred` |
| lowercase identifier binds the whole value | accept, exit 5 [L57] | parser-refused | `Unexpected token RETURN, expected _ => <expr> in match at line 3` |
| unbound fields written as .. in a record pattern | accept, exit 0 [L57 (.. keeps disposal responsibility)] | parser-refused | `Unexpected token RANGE, expected record pattern field name at line 2` |

### function types and closures: 0 connected of 3 (0 of the 3 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| fn(Expr): Bool parameter type | accept, exit 0 [L57 (function types)] | parser-refused | `Unexpected token FN, expected type name in static box method parameter type annotation at ` |
| closure fn[delta, edit total](...) | accept, exit 0 [L57 (closure)] | parser-refused | `Unexpected token LBRACK, expected LBRACE at line 6` |
| update(key, edit action) | accept, exit 0 [L57 (Map edit update)] | parser-refused | `Unexpected token COLON, expected parameter name at line 2` |

### containers: 0 connected of 4 (0 of the 4 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| get(i): T? from me | accept, exit 0 [L57 (containers)] | refused-named | `UnsupportedResultType` |
| edit push(take v: T) with a moved local | accept, exit 0 [L57 (containers)] | refused-generic | `array-holder/append-source` |
| array pop | accept, exit 0 [L57 (containers)] | refused-generic | `unowned-lifecycle-site` |
| array remove | accept, exit 0 [L57 (containers)] | refused-generic | `unowned-lifecycle-site` |

### rules that change in 3b: 0 connected of 4 (0 of the 2 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| Box == Box | reject from 3b (current behaviour stays until then) [L58 (rejecting Box == Box starts at 3b; until then the current behaviour stays)] | refused-generic | `unsupported-cohort` |
| record declaration | reject from 3b (current behaviour stays until then) [L58 (record is removed at 3b; until then it stays)] | refused-generic | `birth-global-legacy-stopped` |
| release x (the `release root` statement) | accept, exit 0 [EBNF release_stmt (parser and source live, semantic 0)] | refused-generic | `ResolverDeferred` |
| sync box | accept, exit 0 [L102 (one sync box example is connected before the switch)] | refused-generic | `new-source-drift` |

### String field reads: 6 connected of 16 (6 of the 13 accepted forms)

| Form (spelling) | Frozen expectation | Measured | Tag or exit |
|---|---|---|---|
| print(t.f) | accept, exit 0 [rule1/field read] | connected | `exit 0` |
| print(me.f) inside a method | accept, exit 0 [rule1/field read] | connected | `exit 0` |
| print(t.f + "x") | accept, exit 0 [rule1/field read] | connected | `exit 0` |
| local s = t.f + "x" | accept, exit 0 [rule1/field read] | connected | `exit 0` |
| local n = t.f.length() | accept, exit 3 [rule1/field read] | connected | `exit 3` |
| local s = t.f | accept, exit 0 [rule1/field read] | refused-generic | `artifact-source-unavailable` |
| return t.f.length() | accept, exit 3 [rule1/field read] | refused-generic | `artifact-source-unavailable` |
| print(t.f.length()) | accept, exit 0 [rule1/field read] | refused-generic | `print-expression` |
| if t.f.length() > 2 | accept, exit 0 [rule1/field read] | refused-generic | `artifact-source-unavailable` |
| t.f == "abc" comparison | accept, exit 0 [rule1/field read] | refused-generic | `print-expression` |
| new Tag(t.f) | accept, exit 0 [rule1/field read] | refused-generic | `argument-source-unavailable` |
| read inside a loop | accept, exit 0 [rule1/field read] | refused-generic | `facts-absent` |
| read inside an if | accept, exit 0 [rule1/field read] | connected | `exit 0` |
| get() { return me.f } unannotated | reject [rule5 (borrow return needs from me)] | against-frozen | `exit 0` |
| get(): String { return me.f } annotated, no from | reject [rule5] | refused-named | `UnsupportedResultAnnotation` |
| print(t.get()) with unannotated get | reject [rule5] | against-frozen | `exit 0` |


### exact-i64 static call composition (S2, S2b): 12 connected of 17 measured forms

Every row was built to an EXE with the S2 binary and, when connected, run; `base` is the binary of the commit before S2. The callee is `a(x: i64): i64 { return x }` unless noted; "with Home" means the caller holds a live `new P()` object.

| Form (spelling) | base | S2 | Tag or exit |
|---|---|---|---|
| `local r = Main.a(Main.b(1))`, no Home | refused-named `borrowed-static/source-unobserved` | connected | `exit 0`, output 1 |
| same, with Home | refused-named `borrowed-static/source-unobserved` | connected | `exit 0`; a Fault inside the callee ends the Home once |
| `local x = Main.b(1)` then `Main.a(x)`, with Home | refused-named `static-execution/live-home` | connected | `exit 0` |
| `print(Main.a(1))`, with and without Home | refused-named `qualified-preflight` | connected | `exit 0` |
| `local r = Main.a(1) + Main.b(2)`, with Home | refused-named `qualified-preflight` | connected | `exit 0`, output 3 |
| `local r = Main.a(1) + 1`, no Home | refused-named `qualified-preflight` | connected | `exit 0`, output 2 |
| `return Main.a(x) + 1`, with Home | refused-named `qualified-preflight` | connected | `exit 5` |
| `Main.b(x + 1)` argument, with Home | refused-named `static-execution/live-home` | connected | `exit 0`, output 6 |
| `local r = Main.a(1) * 2`, with Home | refused-named `qualified-preflight` | refused-named | `object-return/normal-projection-exit` |
| `Main.b(x - 1)` argument (`local r`, `print(...)`, `Main.b(x-1) + Main.b(x-2)`; `x - y` too) | refused-named | connected (S2b) | `exit 0`, output 4 |
| `Main.a(Main.b(1) + 1)` argument, with and without Home | refused-named | connected (S2b) | `exit 0`, output 2 |
| `r = Main.a(4)` Assignment, with and without Home | refused-named `object-return/normal-projection-exit` | connected (S2b) | `exit 0`, output 4 |
| `r = Main.a(4) + r` Assignment | refused-named | connected (S2b) | `exit 0`, output 6 |
| `return Main.a(x) + 1` inside a callee | refused-named | refused-named | `published-lifecycle-program/instruction-unsupported` (the callee emits a plain Call) |
| `if Main.a(1) < 3 { ... }`, with Home | refused-named | connected (S2c) | `exit 0`, output 7 |
| `if Main.a(1) < 3 { ... }`, no Home | refused-named | connected (S2c) | `exit 0`, output 7 |
| `if Main.a(Main.b(1)) < 3`, `if 3 > Main.a(1)`, `if Main.a(1) < Main.b(2)`, `if Main.a(k) + 1 < 3`, `if Main.a(1) < 3 and Main.a(2) < 3` | refused-named | connected (S2c) | `exit 0`, output 1 / 1 / 1 / 2 / 1 |
| `if Main.a(2) == 2`, `if Main.a(5) < 3 or Main.a(1) < 3`, the same `if` inside an `else` branch, `if Main.a(x) < 3` inside a callee | refused-named | connected (S2c) | `exit 0`, output 1 / 1 / 4 / 1 |
| `if Main.a(0 - 5) < 0` (an expression argument inside a condition; `print(Main.a(0 - 5))` connects) | refused-named | refused-named | `local-commit/bool-condition/unowned-call-binding` |
| `if me.a(1) < 3` in a static `main` (current-owner call in a condition) | refused-named | refused-named | `published-lifecycle-program/instruction-unsupported` |
| `loop(Main.a(i) < 3)` | refused-named | refused-named | `object-return/normal-projection-exit` |
| callee body `return x - y` over two parameters, `print(Main.b(5, 2))` (`return y` connects) | refused-named | refused-named | `published-lifecycle-program/scalar-source-coverage` |
| terminating recursion `down(n: i64): i64 { if n < 1 { return 7 } return Main.down(n - 1) }` | refused-named | refused-named | `local-commit/root-exit-source-missing` |
| instance method `add(x: i64): i64 { local y = x return y }` result (S3a: a local's class is its initializer's class; also `local y = x + 1 return y * 2`, `local y = me.base return y + x`) | refused-named | connected | `exit 3` (was `local-commit/artifact-source-unavailable`; the group totals above are not recomputed) |
| static callee `local y = x return y`, `return y + y`, `local y = 0 if x > 2 { y = x } return y`, an inner-block `local y = 100` shadowing, `print(Main.a(Main.a(3)))` | connected | connected | output 4 / 8 / 4 and 0 / 100 then 4 / 3 |
| static callee `local y = x y = y + 1 return y`, `print(Main.a(4))` | refused-named | connected (S3a) | `exit 0`, output 5 |
| instance `f(x: i64): i64 { local y = x + 1 return y * 2 }`, `local q = c.f(1) local r = c.f(q)` | refused-named | connected (S3a) | `exit 10`, output 10 |
| static callee `local y = Main.b(x) return y`; `local y` then `y = x` | refused-named | refused-named | `published-lifecycle-program/instruction-unsupported` |
| static callee `local y = x * 2 local z = y - 1 return z + y` | refused-named | refused-named | `published-lifecycle-program/scalar-source-coverage` |
| callee local updated in a `loop` and returned | refused-named | refused-named | `callable-semantic-lowering/incomplete-consumption` |
| instance `local y = me.v return y + x` over an untyped field, or `local z = y * x` printed directly as `print(c.f(5))` | refused-named | refused-named | `local-commit/print-expression/source-unavailable` |
| `t(x: i64): bool { return x > 1 }` | refused-named | refused-named | `UnsupportedResultAnnotation` (a `bool` result kind is not in the callable header) |

A call's `I64`, `Bool` or `Unit` result is also checked against the callee's declared return type by the independent verifier (`mir/invoke/call-result-type-drift`). Object Homes are checked from the published MIR alone (`ownership_ssa/published_homes.rs`): every Normal and Fault exit must End each Home exactly once; functions that use handle results, text, a Home with several copies, edge arguments or a call with a managed operand are not modeled and make no claim. Map Homes alive across such a call stay refused (`local-call-prior-homes-unsupported`). The canonical qualified-preflight route is retired (`qualified-route-retired`); none of the 229 measured inputs reached it.

### declared `Bool` result (series bool-result)

A function result declared `: Bool` is a result-contract row and takes the route the same function takes with no declaration; the declaration is proved against the body (rule document `mirbuilder-result-contract-bool-2026-11-07.md`). Measured by EXE with the series binary; `undeclared` is the same function without `: Bool`.

| Form | undeclared | declared `: Bool` | Tag or output |
|---|---|---|---|
| `json-stream-aggregator` with `boolField(line, key): Bool` and `ingestLine(line): Bool` | connected | connected | the same 9 output lines and `Result: 0` |
| static `t(s): Bool { if s == "a" { return true } return false }`, `local b = P.t(line) if b {...}` from an instance method (Text input) | connected | connected | `11` |
| the same with `P.t(line + "!")` or `local x = line + "!"` as the argument | connected | connected | `11`, same End count |
| `if P.t(line)` as the condition, `return s == "a"`, `local c = s == "a" return c` | refused-named | refused-named | `source-use-completion-pending`, `forbidden-operand`, `cohort-unfinished` |
| a forwarding callee `u(s) { return P.t(s) }`, an instance `c.t(line)` or `me.t(line)`, a mutual recursion | refused-named | refused-named | `cohort-unfinished`, `original-input-not-ready`, `source-use-completion-pending` |
| i64 input `t(x: i64): Bool`, called from `main` as `local b = Main.t(4)` | refused-named | connected (see the next section) | `1` / `0` |
| i64 input `t(x: i64): Bool`, called as `if Main.t(4)` or `print(Main.t(4))` | refused-named | refused-named | `borrowed-static/source-unobserved` before; now `physical-boundary/threading-branch-unvalidated` and `print-expression/source-unavailable` |
| `t(s): Bool` whose exits return `1`/`2`, or `true` and `2` | refused-named | refused-named | `PhysicalHeader DeclaredResultDisagrees` (declared) |
| `: bool`, `: Boolean`, `: BoolBox` | refused-named | refused-named | `UnsupportedResultAnnotation` |
| `bad(x: i64): i64 { return true }` (a declared `i64` whose exit proves a Bool) | refused-named | refused-named | `PhysicalHeader DeclaredResultDisagrees` (was `terminal-scalar-source-missing`) |
| a `Bool` parameter `t(b: Bool)` | refused-named | refused-named | `ParameterContract UnsupportedDeclaredType` (unchanged) |
| `print(4 > 1)` | refused-named | n/a | `print-expression/source-unavailable` (unchanged) |

### Bool result of a static call over exact-i64 input (series bool-claim)

A static scalar call claim carries the callee's result kind (I64 or Bool) from its contract row, so a `: Bool` or undeclared Bool callee over exact-i64 input takes the I64 callee's route (rule document `mirbuilder-result-contract-bool-2026-11-07.md`, R12 to R18, section 6). Measured by EXE with the series binary against the previous main; `undeclared` is the same function without `: Bool`.

| Form | previous main | undeclared | declared `: Bool` | Tag or output |
|---|---|---|---|---|
| `t(x: i64): Bool { return x > 1 }`, `local b = Main.t(4)` then `if b`, also `local c = Main.t(0)` | refused-named | connected | connected | `1` `0` |
| the same with `==`, `<` over two formals, `x + 1 > 3`, an argument expression `k - 5` | refused-named | connected | connected | the expected `1` `0` |
| a body `local b = x > 1; return b`, or `if x > 1 { return true }; return false` | refused-named | connected | connected | `1` `0` |
| a body `if ... { return true } else { return false }`, `t(): Bool { return 1 > 0 }`, `local c = b` after the call | refused-named | connected | connected | `1` |
| mutual or self recursion of `: Bool` callables through `local k = Main.odd(n - 1); return k` | refused-named | refused-named | connected | `even(10)` is `1`, `even(7)` is `0`; undeclared recursion stays `selected-header-missing` |
| a Bool callee over an unannotated formal given an integer, `class_id(p) { return true }` | refused-named | connected | connected | `1` (same as the I64 callee) |
| `u(x: i64): Bool { return Main.t(x) }`, or `return Main.odd(n - 1)` | refused-named | refused-named | refused-named | `root-exit-source-missing` (the i64 callee `u(x: i64): i64 { return Main.f(x) }` is refused by name on the previous main too) |
| `return x > 1 && y < 3` | refused-named | refused-named | refused-named | `static-execution/completion-not-ready` (no value writer for a short-circuit) |
| `if Main.t(4)`, `print(Main.t(4))` | refused-named | refused-named | refused-named | `physical-boundary/threading-branch-unvalidated`, `print-expression/source-unavailable` |
| instance `c.t(4)` and `me.t(4)` over exact-i64 input | refused-named | refused-named | refused-named | `local-commit/artifact-source-unavailable`, `object-return/normal-projection-exit` |
| `t(x: i64): Bool { return x + 1 }` called from `main` | refused-named | n/a | refused-named | `callable-main/qualified-route-retired`; with one exit `return true` it is `PhysicalHeader DeclaredResultDisagrees` |
| `Main.t(4) + 1`, `Main.t("a")`, `Main.t(true)` | refused-named | refused-named | refused-named | `object-return/normal-projection-exit`, `borrowed-static/source-actual-unavailable`, `callable-main/qualified-route-retired` |
| `main()` that has a `new` and returns a Bool local, `local s = new Store(); local b = true; return b` | refused-named | refused-named | n/a | `artifact-root-bool-result-unsupported` (was `IncompleteOrdinaryNewCoverage`; a Bool literal return in `main` already had this name). Without a `new` it is `main-arguments/root-completion`, as before |

### declared `String` result (series owned-string)

A `: String` declaration reads the one declared-result table (`DeclaredResultKindV1`) and takes the same Text result contract as an undeclared function, so both reach the existing `OwnedString` call result (rule document `mirbuilder-result-contract-string-2026-11-07.md`, S1 to S9). Measured by EXE with the series binary against the previous main; `undeclared` is the same function without `: String`.

| Form | previous main | undeclared | declared `: String` | Tag or output |
|---|---|---|---|---|
| `t(): String { return "abc" }`, `local s = Main.t()` then `print(s)`, or `print(Main.t())` | refused-named | connected | connected | `abc` (was `PhysicalHeader UnsupportedResultAnnotation`) |
| `u(): String { return Main.t() }`, or `local a = Main.t(); return a` in `u` | refused-named | connected | connected | `abc`, `x` |
| `local s = Main.t(); local u = Main.t(); print(s + u)` | refused-named | connected | connected | `abcabc` |
| a body `local s = "ab"; return s + "cd"`, and `s.length()` on the received String | refused-named | connected | connected | `abcd`, exit `1` |
| instance `g.t()` and `me.t()` | refused-named | connected | connected | `hi` |
| `: String` that returns `new Leaf(1)`, `null`, an i64 local or `1 < 2` | refused-named | n/a | refused-named | `callable-main/qualified-route-retired` (the name hides the disagreement, as for `: Bool`) |
| instance `get(): String { return me.v }` over an i64 field | refused-named | n/a | refused-named | `PhysicalHeader DeclaredResultDisagrees` |
| `t(x: String): String { return x + "d" }` | refused-named | refused-named | refused-named | `borrowed-static/source-actual-unavailable` for both (was `UnsupportedResultAnnotation` when declared) |
| `: String?`, `: StringBox` | refused-named | n/a | refused-named | `ParameterContract UnsupportedResultType`, `UnsupportedResultAnnotation` |
| `local t = move s` or `s = Main.t()` on a received String | refused-named | refused-named | refused-named | `MoveUnsupported`, `borrowed-static/local-source-missing` (the same undeclared) |
| any of the connected forms in a file that also declares an unused box with methods | refused-named | refused-named | refused-named | `published-lifecycle-physical-abi/check-uncovered-function`, declared or not, also on the previous main |

### declared object result (series owned-object)

A `: P` declaration, where name resolution finds `P` as a box of the source, reads the one declared-result table (`DeclaredResultKindV1`) and takes the same object result contract as an undeclared function, so both reach the existing `Handle` call result (rule document `mirbuilder-result-contract-object-2026-11-07.md`, O1 to O9). The series covers instance and `me` calls only. Measured by EXE with the series binary against the previous main; `undeclared` is the same function without `: P`.

| Form | previous main | undeclared | declared `: P` | Tag or output |
|---|---|---|---|---|
| instance `mk(): Leaf { return new Leaf(me.k) }`, `local a = m.mk()` then `a.get()` | refused-named | connected | connected | exit/result `7` (was `PhysicalHeader UnsupportedResultAnnotation`) |
| a body `local a = new Leaf(..); return a`, also with `a.get()` read before the return | refused-named | connected | connected | the same output declared or not |
| both branches of an `if` return `new Leaf(..)` | refused-named | connected | connected | `7` |
| two received objects added, another box built and added, a received value printed | refused-named | connected | connected | `14`, `9`, `7` |
| `run() { local a = me.mk(); return 1 }` | refused-named | connected | connected | `1` |
| `: Leaf` that returns `1`, `new Other()`, or `: Other` that returns `new Leaf(..)` | refused-named | n/a | refused-named | `borrowed-result/result-contract-mismatch`, `lexical-instance-call/handle-result-mismatch` |
| `: Leaf` that returns `null` on one path | refused-named | refused-named | refused-named | `local-commit/artifact-source-unavailable` declared, `root-instance-call-new-missing` undeclared |
| `: i64` that returns `new Leaf(..)`; `: Nope` with no such box | refused-named | n/a | refused-named | `object-return/result-cohort-result-contract`; `UnsupportedResultAnnotation` |
| `: Leaf?` | refused-named | n/a | refused-named | `ParameterContract UnsupportedResultType` |
| static `mk(v: i64): Leaf` received by `local a = Main.mk(5)` | refused-named | refused-named | refused-named | `main-import-view/selected-header-missing` undeclared, `callable-main/qualified-route-retired` declared: static calls have no route for an object result (connected since series static-result-kind, below) |
| `return m.mk()` forwarding; `local a = me.mk()` then `a.get()`; a call that can fault after the receive | refused-named | refused-named | refused-named | `admission-function-not-birth`; `borrowed-result/source-not-i64`; `local-commit/root-call-binding-contracted` (the same undeclared) |
| `m.mk(5)` with an argument, `a.get() + 1`, `a = m.mk()`, `next(): Maker`, `return me.f`, returning a `take` formal | refused-named | refused-named | refused-named | `borrowed-result/source-not-i64`, `borrowed-nullable/local-initializer`, `borrowed-entry/finite-domain-unclosed` (the same undeclared) |

### Nullable object results received from a call

An instance or qualified-static factory may return `new P(..)` or `null`; a
root-scope local can receive that result, compare it with `null`, and call its methods. A method
call on the null outcome raises TypeError before entering the method. Its
Fault path releases the caller's previously acquired objects, while the null
outcome owns no payload to release. Early returns preserve the same cleanup
law for nullable and non-null object results.

| Form | Measured EXE result |
|---|---|
| `local a = s.find(3); if a != null { print(a.get()) }` | prints `3`, exit 0 |
| `if a == null { return 1 }; return a.get()` after a receive | non-null exit 3; null exit 1 |
| `return a.get()` without a guard | non-null exit 3; null TypeError (Fault exit 70) |
| two received nullable locals checked separately | exit 3 |
| qualified-static `local a = Main.find(3)` | prints `3`, exit 0 |
| `find(k: i64): P?`, instance / qualified static | prints `3` / exit 3 |
| `empty(): P? { return null }`, received and compared with null | prints `0`, exit 0 |
| `P?` declaration with only a fresh-object return | received method result exit 9 |
| non-null receive followed by an early return guarded by a scalar condition | exit 3, with or without the result annotation |

Assignment to the received local, aliases, a receive inside `if`, discarded
object calls, forwarding nullable results, Array storage, nullable String
results and Array element borrows remain outside this measured boundary.
`P?` accepts an object-only or null-only body as well as a mixed body; the
unannotated null-only body does not infer an object type. A nonnullable `P`
declaration returning null is refused. A nullable receiver call with a `take`
argument is still unsupported. Calling a typed-field getter through an
always-null result also remains unsupported by the current compiler; an
always-null call to a literal-result method reaches TypeError correctly.

Also measured at the audit (2026-11-08, unseen inputs): `null == a` with the
operands reversed works like `a == null`. `a == void` is refused by the C
backend (`published-lifecycle-v4/unsupported-cohort`), although the language
makes `void` and `null` the same value, because the builder emits `const void`
there. Still refused on both binaries: a method call with an argument on the
received result (`a.plus(10)`), `local isnil = a == null`, and a factory that
returns `new P(a + b)` (`object-return/callee-exit`, also for non-null
results).

### static call result bound in a plain function (series static-call-context)

An owner that binds, in a `local`, the result of a static call with only i64 formals (or none) joins the typed static execution, whether it is `main` or not (rule document `static-call-context-rule-2026-11-07.md`, C1 to C8). Measured by EXE with the series binary against the previous main.

| Form | previous main | this series | Tag or output |
|---|---|---|---|
| a `main()` with nothing else, `local a = Main.f(5); return a`, with `f(v: i64): i64` declared or without the result declaration | refused-named | connected | `Result: 6` (was `qualified-route-retired`, `selected-header-missing`) |
| `g(v: i64): i64 { local a = Main.f(v); return a + 1 }` called by `print(Main.g(5))`; a chain `h` to `g` to `f` | refused-named | connected | `7`, `8` (was `instruction-unsupported` for the raw `Call`) |
| two received values added, a received value as the next call's argument, a receive inside `if`, `return a + 1`, a callee in another static box | refused-named | connected | `14`, `7`, `6`, `7`, `7` |
| the same bodies with a `new` or a `print` in `main` | connected | connected | the same output |
| `return Main.f(5)` | refused-named | refused-named | `main-arguments/root-completion` |
| a callee whose body has `*` | refused-named | refused-named | `static-execution/completion-not-ready` (was `qualified-route-retired`) |
| a function with `local a = Main.f(1)` that a bare call reaches | refused-named | refused-named | `DirectCall LifecycleSourceMismatch` |

### object result of a static call (series static-result-kind)

One table decides a call's result kind from the callee's contract row, for instance, bare and static calls; the typed static execution carries a box `Handle` result, and a local a static call initializes takes the callee's result class like an instance call (rule document `static-call-result-kind-2026-11-08.md`, K1 to K14). Measured by EXE with the series binary against the previous main.

| Form | previous main | this series | Tag or output |
|---|---|---|---|
| `mk(v: i64): Leaf { return new Leaf(v) }`, `local a = Main.mk(5)` then `return 0` or `return a.get()`, declared or without `: Leaf`; a zero-argument `mk(): Leaf` | refused-named | connected | `0`, `5`, `4` (was `qualified-route-retired`, `selected-header-missing`) |
| two receives added, `a.get() + 1`, `a.set(9)` then `a.get()`, two classes, a callee in another static box, two `return`s under `if` in the callee, `mk(x, y): Pair` | refused-named | connected | `5`, `6`, `9`, `5`, `9`, `50`, `7` |
| a receive inside a static `g(v: i64): i64` that returns `a.get()`; a call that can fault after the receive | refused-named | connected | `7`, `8` |
| zero-argument `t(): String` received and printed or concatenated; instance `m.mk()` then `a.get()` | connected | connected | the same MIR |
| `t(x: i64): String` received | refused-named | refused-named | `selected-header-missing` (stopped: the Text observation gate and the header row, rule document §3; connected since series static-text-entry, below) |
| an expression argument, `Main.mk(n + 1)` or `Main.mk(Main.f(2))` | refused-named | refused-named | `borrowed-static/source-route-identity` (the i64 result `Main.f(n + 1)` is connected) |
| `local b = a`; a receive inside `if` | refused-named | refused-named | `root-instance-call-new-missing`; `root-call-binding-contracted` (the instance forms have the same names) |
| `a + 1`, `a = Main.mk(6)`, `print(a)`, `a.value` | refused-named | refused-named | `static-execution/completion-not-ready` |
| `print(Main.mk(5) + 1)`, `return Main.mk(5).get()`, `print(Main.mk(5))`, an untyped formal `mk(v)`, `: Leaf` that returns `new Other()` | refused-named | refused-named | `qualified-route-retired` |
| `: i64` that returns `new Leaf(v)`; `: Leaf?` | refused-named | refused-named | `ResultOwnershipMismatch`; `ParameterContract UnsupportedResultType` |
| the received object passed to `show(x: Leaf)` | refused-named | refused-named | `borrowed-view/declared-object-class` (the same for a `new` object) |
| a callee `local x = new Leaf(v); return x`; `g(v): Leaf` that returns the received object | refused-named | refused-named | `handle-result-terminal-mismatch`; `static-execution/result-source` |
| a receive inside `loop` | refused-named | refused-named | `callable-loop/route-not-front-selected` |

### String result of a static call with arguments (series static-text-entry)

A static call whose formals are all i64 (or none) and whose result is a String takes the qualified static input finish, and the received String lives in the same Text residence as an instance or zero-argument receive; the zero-argument-only Text path is gone (rule document `static-call-result-kind-2026-11-08.md`, K15 to K22). Measured by EXE with the series binary against the previous main.

| Form | previous main | this series | Tag or output |
|---|---|---|---|
| `t(x: i64): String { return "abc" }`, `local s = Main.t(1)` then `print(s)`, declared or without `: String`; `print(Main.t(1))` | refused-named | connected | `abc` (was `selected-header-missing`) |
| two receives joined (`s + u`), `print(Main.t(1) + Main.t(2))`, `local s = Main.t(1) + "z"`, a callee that branches on its argument, a callee in another static box, two arguments | refused-named | connected | `abcabc`, `ab`, `abz`, `posneg`, `zz`, `ab` |
| an argument that is an expression or a call, `Main.t(n + 1)`, `Main.t(Main.f(2))`; a callee `return "n=" + x` | refused-named | connected | `big`, `a`, `n=5` |
| a receive inside `g(v: i64): i64` or `g(v: i64): String` that returns it; a forwarding `u(x) { return Main.t(x) }`; a receive that is not used | refused-named | connected | `abc` and `5`, `abc`, `ab` |
| `print(Main.t(1) + 1)` | refused-named | connected | `abc1`: `String + <non-void>` joins as text (quick-reference) |
| zero-argument `t(): String` forms | connected | connected | the same output and MIR |
| `return s + 1` | refused-named | refused-named | `admission-retained-root-result-missing` |
| declared `x: String` formal, `Main.t("abc")` or a String local | refused-named | refused-named | `borrowed-static/source-actual-unavailable` (stopped: a declared String formal has its own two-lane ABI and no caller emits it yet; the undeclared `t(x)` is connected) |
| a receive inside `if`; `s = Main.t(2)`; `local u = s`; an i64 static call after the receive; a String and a Handle in one function | refused-named | refused-named | `IncompleteOrdinaryNewCoverage`; `IncompleteOrdinaryNewCoverage`; `forwarded-copy/root`; `local-commit/direct-scalar/source`; `root-call-binding-contracted` |
| `s.length()` on the received String; a receive inside `loop` | refused-named | refused-named | `root-instance-call-new-missing`; `selected-header-missing` |

### Remaining before 3b

The order follows ADR-002 (3a continuation, then 3b). Status is only `done`, `partial` or `not started`; evidence is a row above or a commit. No effort estimate is given.

| Step | Status | Evidence |
|---|---|---|
| `take` declaration through call entry, End and independent verification | partial | `birth(take x: T)` stored with `me.f = move x` is connected, and so is a `take` parameter fed by `move`; a non-stored `take` formal, a fresh `new T` actual, an ordinary-method `take` with a fresh actual and a receiver `take` are refused. `move` into a plain (non-`take`) formal is rejected, as decided |
| `from` (single support) | partial | the spellings in the `from` group are refused; `from a, b` stops in the parser (open in ADR-002). The `ownership_from_read` and `ownership_from_receiver` smoke cases in `tools/smokes/v2/profiles/integration/apps/canonical_app_exe_cases.sh` are the connected `from` shapes and were not re-measured here |
| `edit` / `take` declarations (receiver, argument, closure), types, `copy`, initialization | partial | `copy` of an i64 is connected; `edit` declarations are refused by name; receiver `take`/`edit` are refused by name; function types and closures stop in the parser; field types: i64, String and StringBox store and read, `String?` stores through an unannotated formal, Bool and `T`/`T?` do not; `T?` has one connected shape (literal `null` store) |
| `share` | partial | local initializers `share new T()`, `share move x`, `share h` are connected; `Shared<T>` as parameter, result and field is refused by name; a method call on a Shared local is refused; `Shared(...)` is rejected by the Rust parser (commit `4949c16b4c`) and by the `.hako` parser (commit `c1739a89c2`); Hako `share` parity is unverified |
| `take place` | not started | expression `take` is not parsed: `local y = take x` is read as the identifier `take` followed by a separate statement |
| enum ownership class, `match` re-borrow, `match move`, field `move` | not started | `match` on `Option` and `match move` are refused without a named reason; a lowercase binder and `..` in a record pattern stop in the parser; `move x.f` stops in the parser |
| array `pop` / `remove`, `push(take v)`, `get(i): T? from me` | not started | refused (`get(i)` by name, the others without a named reason) |
| `update` | not started | the call spelling with a closure stops in the parser |
| `sync box` (one example) | not started | refused without a named reason |
| 9 comparison tasks (the "書き比べの9課題" of ADR-002 line 102) | not started | the nine tasks are not recorded in the repository and no commit runs them |
| 3b structure cleanup (A): `PartialEq` for parameter contracts, one `HomeDemandV1` construction point | done | commit `f6c8754902` (Consolidate parameter contract equality and transfer demand projection); `docs/development/current/main/investigations/app-boxtorrent-mini.md:19` records "整理A完了" with the suite and cohort unchanged; `home_demand()` lives in `src/mir/callable_parameter_contract/model.rs` |
| 3b structure cleanup (B): merge the common part of the three `main0` loop lowerers, split the big files by responsibility | not started | the three `main0` families (`continue`, `derived_predicate`, `in_body_step`) still exist as three parallel sets of eight files each in `src/mir/compiler/` (`main0_*_{recipe_coseal,root_selection,semantic_program,source_map,source_map_issue,syntax_facts}.rs` and tests); no commit merges them |
| 3b switch (non-null default, Box `==` rejection, `record` removal, required-field check, app edits, enforcement, 19 forms) | not started | the rows that change at 3b still show today's behaviour (Box `==`, `record` and a missing field are refused without a named reason) |

## Known conflict set

The accepted target is now fixed, while physical authority pages and
implementation rows remain intentionally unsynchronized:

```text
source try rejected in both profiles versus current Compat parser row
typed Result ? target versus current dynamic/arbitrary-object QMark route
catch/RecoverableFailure rejection versus current registry/AST/runtime transport
single cleanup and Box fini-hook target versus current registry/AST encoding
external source-try migration records are tool/evidence only, outside grammar
concurrency topic rows versus Language v1 registry rows
legacy EBNF/parser exception-shaped evidence versus the Result-only target
```

Resolution owner:
[LANGUAGE-RESULT-EXIT-C-PRIME0-D0](../../development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md)
and [C′ lifecycle](../../development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md).
Their I0/R0 rows own grammar/registry/parser/runtime changes and their mandatory
DOC0 rows close the implementation-backed reference. No parser, runtime,
backend, or grammar-registry implementation change is authorized by this
index update.

Legacy parser/transport evidence is not permission to degrade an unsupported
handler to a no-op. The C′ Decision is accepted while physical retirement is
pending; until its grammar and runtime rows close, unsupported
handler/backend combinations must reject before user-visible effects.

## Historical rule

[`LANGUAGE_REFERENCE_2025.md`](LANGUAGE_REFERENCE_2025.md) is a historical
snapshot. Its examples and status tables must not override this index or the
current topic SSOTs. When a historical example is useful, cite it as migration
evidence and link back here.
