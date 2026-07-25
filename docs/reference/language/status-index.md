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
| `throw` | `status_conflict` | `prohibited` | [scope-exit](scope-exit-semantics.md), [option](option.md), EBNF; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | `[freeze:contract][parser/throw_reserved]` | source/parser producers must remain zero; generated/internal shapes are fenced separately | Accepted target rejects both profiles; internal/generated shapes still require authority sync and are not source permission. |
| statement `try` | `status_conflict` | `status_conflict` | [grammar contract](grammar-contract.md), EBNF, registry; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | `[freeze:contract][parser/try_reserved]` | target rejects both language profiles; registry/parser sync pending | Accepted target has no source `try`; current Compat parser acceptance remains implementation drift, not permission. |
| postfix `catch` | `status_conflict` | `pending` | [semantic kernel](semantic-kernel.md), [failure/outcome relations](failure-outcome-relations.md), grammar contract; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | current ambient gates are evidence only | `RecoverableFailure` producer/ABI D0, then grammar/runtime rows | Accepted target is canonical, protects the preceding region, and never catches terminal `Fault`; physical authority sync is pending. |
| postfix `cleanup` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), EBNF, registry; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | current Stage-3 gates are evidence only | exact grammar/profile/backend rows required | Accepted target is canonical and independent of catch/object lifecycle; no broad live claim. |
| standalone `cleanup { ... }` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), EBNF, registry; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | `index:cleanup-production-missing` | registry/parser/runtime rows required | Accepted target is canonical; physical standalone production is not yet synchronized. |
| `local x = e cleanup { ... }` | `status_conflict` | `pending` | [scope-exit](scope-exit-semantics.md), EBNF, registry; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | `index:local-cleanup-production-missing` | registry/parser/runtime rows required | Accepted target is canonical; physical local-cleanup production is not yet synchronized. |
| `fini { ... }` / `local ... fini` | `status_conflict` | `status_conflict` | [scope-exit](scope-exit-semantics.md), [lifecycle](lifecycle.md), grammar contract, registry; [target decision](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md) | `index:fini-alias-authority-sync-pending` | `LANGUAGE-FINI-SCOPE-ALIAS-SUNSET-001` | Accepted target rejects Canonical and keeps a bounded Compat2025 cleanup alias; current CatchClause marker is legacy representation. |
| `box.fini()` | `canonical` | `guarded`/`status_conflict` | [lifecycle](lifecycle.md), [scope-exit](scope-exit-semantics.md) | `index:lifecycle-owner-check` | Lifecycle owner must name the supported profile | Semantic target is distinct from scope cleanup; production support is not claimed for every backend. |
| `co` / `nowait` / `await` | `topic_owned` | `guarded` concurrency profile | [concurrency semantics](../concurrency/semantics.md), stage profiles | `concurrency-profile-required` | Separate concurrency decision | Not a general Stage1/selfhost promise; no Language v1 registry row currently. |
| `Channel<T>` | `topic_owned` | `scaffold`/`deferred` by route | [concurrency semantics](../concurrency/semantics.md), stage profiles | `channel-route-gated` | Separate concurrency decision | Reference queue surface; broad Program/MIR/LLVM use remains gated. |
| `sync box` / `context` / `task_scope` | `topic_owned` | `scaffold`/`deferred` | [concurrency boundary](../concurrency/boundary-model.md), stage profiles | `sync-boundary-gated` | Separate concurrency decision | Do not infer parser-live or broad runtime support from reference docs. |
| `worker_scope` / `parallel` / raw `thread` / `lock<T>` / `worker_local` | `reserved`/`topic_owned` | `prohibited`/`deferred` | [stage profiles](stage-profiles.md), concurrency docs | `concurrency-substrate-only` | New language decision required | Design/substrate surfaces; no language-core activation. |
| `move` / `share` / `view` | `topic_owned` | `pending`/`prohibited` until exact row | [ownership](ownership.md), stage profiles | `OWN-GRAM-REJECT0` | Ownership grammar row required | Target semantics are not parser support; retain the existing reject-row boundary. |
| `Option<T>` / `Result<T,E>` constructors | `canonical` narrow rows | `live` narrow enum/prelude profile | [option](option.md), EBNF, stage profiles | `option-result-narrow` | Expand only by explicit capability row | `?`, `try`, and `throw` are not implied by enum support. |

## Known conflict set

The accepted target is now fixed, while physical authority pages and
implementation rows remain intentionally unsynchronized:

```text
source try rejected in both profiles versus current Compat parser row
postfix catch RecoverableFailure target versus no canonical producer/boundary ABI
cleanup/fini target naming versus current registry/AST encoding and Stage-3 gates
external source-try migration records are tool/evidence only, outside grammar
concurrency topic rows versus Language v1 registry rows
legacy EBNF/parser exception-shaped evidence versus the protected-region target
```

Resolution owner:
[LANGUAGE-TRYLESS-POSTFIX-CATCH-prime-r1](../../development/current/main/investigations/language-tryless-postfix-catch-task-order-2026-07-26.md).
`LANGUAGE-DOCS-POSTFIX-CATCH-D1-CLOSEOUT` synchronizes the authority prose;
later rows own grammar/registry/parser/runtime changes. No parser, runtime,
backend, or grammar-registry implementation change is authorized by this
index update.

Legacy parser/transport evidence is not permission to degrade an unsupported
handler to a no-op. The D1 decision is accepted; until the later grammar and
runtime rows close, unsupported handler/backend combinations must reject before
user-visible effects.

## Historical rule

[`LANGUAGE_REFERENCE_2025.md`](LANGUAGE_REFERENCE_2025.md) is a historical
snapshot. Its examples and status tables must not override this index or the
current topic SSOTs. When a historical example is useful, cite it as migration
evidence and link back here.
