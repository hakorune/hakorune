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
| Home ownership: declaration `take`, result `from`, expression `share` | `topic_owned` | take/share accepted target; from provisional; production 0 | [ownership](ownership.md), [Home syntax D0](../../development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md), stage profiles | existing inactive-ownership reject boundary | release source I0 -> take declaration I0 -> capability/Home ABI rows; share after representation D0 | `take` is declaration-only contextual; `share` is a non-group postfix prefix. Neither is globally reserved; `share(...)` remains an ordinary call. Former `move/view/shared` syntax is historical. |
| contextual `release root` | `topic_owned` | parser/source carrier live; semantics 0 | [ownership](ownership.md), [Home syntax D0](../../development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md), [release I0](../../development/current/main/investigations/own-home-release-source-i0-implementation-task-2026-08-09.md) | `index:home-release-semantics-missing` | root capability/Home Flow I0 + reference receipts | Dedicated exact-root AST/source row only. `release(value)` remains an ordinary call; `release` is not globally reserved; no Home is ended until later semantic/physical rows close. |
| `@rune CallableContract(query)` | `topic_owned` | accepted target / production 0 | [callable contracts](callable-contracts.md), [runes](runes.md) | `index:callable-contract-query-production-missing` | ordered Box-method inventory -> parser parity -> declared contract -> body conformance | Whole-call query obligation; signature owns types/arity, physical ABI remains downstream. Existing `Contract(pure|readonly)` is not silently promoted. |
| ordinary Dynamic member invocation | `topic_owned` | selector-independent contract accepted; caller-zero semantic-envelope issuer live; production consumer 0 | [Dynamic invocation](dynamic-invocation.md), [failure/outcomes](failure-outcome-relations.md), [ownership](ownership.md) | `index:dynamic-invocation-envelope-production-consumer-missing` | Recipe Dynamic value/CallSlot -> provider admission/plan -> physical canary/cutover | One `OpaqueObservable`, synchronous non-detached but `MaySuspend`, borrowed-input, self-contained-result, Normal-or-Fault contract. Runtime tags and selectors do not choose semantics; no retry/fallback. |
| `Option<T>` / `Result<T,E>` constructors | `canonical` narrow rows | `live` narrow enum/prelude profile | [option](option.md), EBNF, stage profiles | `option-result-narrow` | Expand only by explicit capability row | Constructors are live independently; typed Result `?` remains pending and Option `?` is rejected. |

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
