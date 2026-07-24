# RAW-PUBLIC-INGRESS0-S0 execution task

Decision: `RAW-PUBLIC-ADAPTER-prime-r1` Q2/Q3/Q5

Status: active implementation. This row adds one explicit Raw source entry;
the normal `compile_with_source` route and JSON routes remain unchanged.

## Scope

```text
compile_raw_with_source(ast, source_file)
  -> BareAst / module "main" / callable-Main Omitted
  -> complete Raw owner chain
  -> Raw publication
  -> compatibility adapter
  -> MirCompileResult
```

The caller cannot select policy, REPL, JSON, retry, or fallback. Every typed
rejection is inspected once, converted to the stable
`[raw-public/<stage>/rejected]` String transport, and discarded. No legacy
`build_module` retry is reachable from this entry.

## Internal order

```text
INGRESS-POLICY0  NarrowV1 policy and one source/config capture
INGRESS-CHAIN0   bind -> package -> eligibility -> BODY0 -> ROOTBATCH0
                 -> DRAIN0 -> FINAL0 -> POST0 -> COMMIT0 -> adapter
INGRESS-FAILURE0 typed rejection -> report -> discard(self)
INGRESS-G0       explicit consumer=1, normal/JSON routes unchanged, <800 lines
```

## Non-claims

```text
compile_with_source cutover
REPL support
Program(JSON v0), AST-JSON, executor, selfhost, fastmem
legacy Raw bridge retirement
caller-selected callable-Main policy
CUT0 activation
```

## Acceptance

```text
empty Script returns MirCompileResult
source-file hint is captured once
REPL rejects before source binding
unsupported or typed stage failure returns stable raw-public prefix
live Builder remains quiescent after success and failure
compile_raw_with_source never calls compile_legacy or build_module
```

`sunset_id = RAW-PUBLICATION-SUNSET-001`; normal-entry cutover remains a later
measured retirement row.
