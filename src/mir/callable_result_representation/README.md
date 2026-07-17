# Same-module callable result representation

This module seals one disconnected exact-`i64` sufficient-condition catalog
from the exact same-module callable declaration and source-target catalogs.

The S0b substrate owns:

- one result disposition for every static declaration key;
- exact parameter ordinals required to be `i64` at a call site;
- exact source-site call-result evidence;
- bounded String receiver composition with generated Core result rows;
- deterministic construction-local dependency solving;
- explicit `Unavailable` reasons for valid but unsupported source.

The result catalog is lifetime-bound to the exact declaration and target
catalogs. Their pointer identity is co-validated before proof construction;
equal-looking canonical keys from a foreign catalog are not sufficient. A
same-module call row borrows its exact CUT0 target row and never clones it into
a second target authority.

Canonical `SourcePathV1` is the only expression-site vocabulary. Nested call
arguments are proved child-before-parent. Required callee argument ordinals are
substituted through the caller's ordered facts, while non-required non-`i64`
arguments do not invalidate an otherwise exact result.

Qualified and current-owner `MethodCall` sites may consume the exact source
target catalog. A call without such a row may use only the bounded
`ExactStringOnSuccess` receiver fact together with one generated String Core
result row. Method spelling alone is never result authority. Bare
`FunctionCall` remains unavailable and is never target-guessed.

Pending dependency state is private to the monotone solver. Rows close in
canonical-key order until the construction is complete or stable; unresolved
direct or mutual cycles become `RecursiveDependency`. A final stable pass
seals call rows exactly once. No public mutable Pending row, declaration-order
dependency, callee-first lowering, retry, fallback, re-lowering, or SCC
inference is introduced.

This proof is representation-only. It does not claim call totality, purity,
termination, general String interpretation, general non-`i64` result typing,
or recursive result inference. It stores no `ValueId`, `MirType`, Builder,
final MIR metadata, physical-symbol parsing, runtime tag, or HMI-specific fact.

S0b remains disconnected: production producers, consumers, call-result
publication, lowering behavior, runtime behavior, backend behavior, and
ownership behavior all remain unchanged.

## SITE0 located legacy inputs

`located_legacy` borrows one canonical caller and its syntax only from a sealed
activation plan. It carries exact function-relative sites through the neutral
PATH0 role policy, but does not lower MIR, resolve targets, infer results, or
claim activation rows.

Located and unlocated syntax are distinct inputs. Synthetic syntax and every
descendant constructed from it remain unlocated; only a located expression can
expose an activation site. Production consumers remain zero through SITE0-L0.
