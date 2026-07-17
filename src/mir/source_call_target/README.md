# Source call target

This module owns disconnected, pre-Builder source-call target proofs.

Q0 admits one route only:

```text
qualified receiver.method(arguments)
  + verified import alias view
  + exact lexical-binding observation
  + reserved-route disposition
  + complete same-module declaration catalog
  -> canonical static callable key
```

The final catalog is keyed by caller canonical key and function-relative
`SourceExprSiteV1`. Import aliases are copied into one sorted immutable view
and checked against the same declaration catalog before they may participate.
The mutable Builder import map is never a sealed authority.

Q0 deliberately has no production producer or consumer. It does not own:

- current-owner or bare-call resolution;
- builtin, Core, extern, or value-call routing;
- argument evaluation or result representation;
- MIR symbol parsing, emission, runtime behavior, or fallback.

Imported aliases preserve the current Builder precedence over a same-spelled
local binding. Direct canonical receiver spellings require an explicit
`Unbound` lexical fact. Reserved fastmem/MIR/REPL receiver routes fail closed.

Future route families may add variants to the final target vocabulary, but
they must keep route-disjoint sealers. They must not turn this module into a
replay of the complete Builder call router.
