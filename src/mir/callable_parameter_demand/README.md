# Callable parameter demand

This module owns one source-bound semantic relation:

```text
complete parser callable-parameter source catalog
+ exact parser declaration syntax loan
+ canonical resolved callable owner forests
  -> complete VerifiedCallableParameterDemandCatalogV1
```

The first cohort maps every parser-sealed `Ordinary` parameter to
`HomeDemandV1::Handle`. Zero-parameter declarations remain present as empty
declaration rows, so downstream consumers cannot confuse an empty parameter
list with a missing declaration.

The final catalog retains both the parser catalog and the resolved forests.
It is non-Clone and exposes borrow-scoped declaration views only; there is no
`into_parts`, row selector, name lookup, or caller-supplied `BindingRef`.

## Boundary

This module verifies exact declaration coverage, static/instance root mode,
parameter ordinal, owner, binding kind, source origin, diagnostic name, and
duplicate-free `BindingRef` identity before issuing any demand.

It does not own or infer:

- `Take`, `Home`, `SharedHome`, or result Home relations;
- receiver ABI or instance callable contracts;
- Dynamic carrier classification or lifecycle;
- Recipe, JoinSig, CFG, SSA, PHI, MIR, or physical ABI;
- production route selection, retry, or fallback.

Any future transfer syntax must be admitted by its source-backed semantic
issuer. It must not be added here as another spelling-to-demand match arm.
