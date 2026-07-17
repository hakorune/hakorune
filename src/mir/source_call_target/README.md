# Source call target

This module owns disconnected, pre-Builder source-call target proofs.

## AST-BIND0-S0 exact source site

`VerifiedSourceMethodCallSiteV1` is the sole exact caller/body/site co-seal.
Its constructor accepts only the complete declaration catalog, one canonical
caller key, and one function-relative `SourceExprSiteV1`. The catalog supplies
the body; the shared resolved-semantics projector supplies the exact borrowed
node; only `ASTNode::MethodCall` is admitted. Receiver, method, arguments,
receiver site, and checked arity are derived from that node and can never be
supplied independently.

Sites that cross a nested `FunctionDeclaration` or `Lambda` callable boundary
reject before projection. The top-level catalog key must never be paired with
a nested callable body that has its own semantic owner.

The product is lifetime-bound and non-Clone. It owns no AST, body, lexical
fact, import alias, reserved-route decision, target, ABI, effect, result
representation, MIR, or runtime authority. S0 remains disconnected with zero
production consumers. Lexical Bound/ProvenUnbound observation belongs to the
following L0 row.

Q0 and M0 admit two route-disjoint source shapes:

```text
qualified receiver.method(arguments)
  + verified import alias view
  + exact lexical-binding observation
  + reserved-route disposition
  + complete same-module declaration catalog
  -> canonical static callable key

ASTNode::MethodCall { object: ASTNode::Me, method, arguments }
  + caller canonical key in the complete declaration catalog
  + caller namespace = StaticBoxMethod
  + exact target lookup under caller.owner()
  -> canonical current-owner static callable key
```

The final catalog is keyed by caller canonical key and function-relative
`SourceExprSiteV1`. Import aliases are copied into one sorted immutable view
and checked against the same declaration catalog before they may participate.
The mutable Builder import map is never a sealed authority.

The current-owner route consumes the existing catalog by value and extends it;
it never creates a second target catalog. The caller key is the sole owner
authority. `current_static_box`, function-name splitting, MIR symbols, Builder
state, and runtime tags are not authorities. Parser spellings `me` and `this`
both normalize to `ASTNode::Me`, so M0 proves the canonical semantic receiver,
not preservation of the original token spelling.

Q0 and M0 deliberately have no production producer or consumer. They do not
own:

- bare-call resolution;
- builtin, Core, extern, or value-call routing;
- argument evaluation or result representation;
- MIR symbol parsing, emission, runtime behavior, or fallback.

Imported aliases preserve the current Builder precedence over a same-spelled
local binding. Direct canonical receiver spellings require an explicit
`Unbound` lexical fact. Reserved fastmem/MIR/REPL receiver routes fail closed.

Future route families may add variants to the final target vocabulary, but
they must keep route-disjoint sealers and reject duplicate caller/site rows
across variants. They must not turn this module into a replay of the complete
Builder call router.
