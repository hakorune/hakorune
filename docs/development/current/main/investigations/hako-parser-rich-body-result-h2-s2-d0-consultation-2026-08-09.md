---
Status: closed accepted Decision; implementation 0
Date: 2026-08-09
Decision: accepted after live-owner and transaction audits
Parent: `HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0`
Predecessor: `HAKO-PARSER-PARAMETER-LIST-PRODUCT-H2-S1` closed
Next: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-R0`
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-D0

## Decision

H2-S2 extends the existing live grammar owners to return one same-pass rich
result. It does not add a sibling body parser.

```text
ParserControlBox.parse_block
  -> ParserBox.parse_stmt2
  -> ParserStmtCoreBox.parse
  -> exact return arm
  -> ParserBox.parse_expr2
  -> ParserExprPrecedenceBox.parse_expr2
  -> ParserExprBox.parse_factor_in_context2
  -> ParserExprBox.parse_number2
  -> ParserNumberScanBox integer lexing
```

The compatibility JSON and typed source carrier are projections of the same
parse decisions. JSON is never decoded into source truth.

## First exact cohort

```hako
box TextLike {
    length(): i64 {
        return 0
    }
}
```

The exact body cohort is:

```text
one direct instance method
one block
one explicit Return statement
one unsuffixed decimal integer literal
optional semicolon
```

The existing vocabulary is complete. No new syntax kind is authorized.

```text
node 0: expr / LiteralInt(0)
node 1: stmt / Return(Present), child = node 0
list 0: stmt list [node 1]
node 2: root / SourceBody, list = list 0
root = node 2

node count = 3
list count = 1
```

`Block`, `MethodBody`, and `ExpressionStatement` are not added merely to make
this canary pass.

## Owner-preserving rich-result boundary

```text
typed integer lexical parts
  -> typed LiteralInt node
  -> compatibility Int JSON

ParserStmtCoreBox.parse_product
  -> Typed Return(Present)
  -> CompatOnly
  -> ParseError

ParserControlBox.parse_block_product
  -> parse every statement exactly once
  -> all Typed: seal exact SourceBody tree
  -> any CompatOnly: typed tree publication = 0
  -> any ParseError: typed tree publication = 0

legacy parse / parse_block
  -> compatibility projection from the rich result
```

`ParserNumberScanBox.scan_int` currently exposes a JSON-plus-position string.
The implementation must first introduce a private lexical result containing
the parsed integer value, suffix/kind information, and next position. Both
typed source emission and compatibility JSON project from that result.

The exact rich path must prove that at least one decimal digit was consumed.
It must not inherit a synthetic `0` for missing input.

The return grammar must distinguish before JSON projection:

```text
return       -> Return(Absent)
return 0     -> Return(Present, LiteralInt(0))
```

The current compatibility JSON may continue to project absent return as its
legacy default, but the typed source authority may not collapse the two.

## Method-bound body result

A bare numeric `node_ref` is not method source identity. The parser-private
body result co-seals:

```text
exact method source site
+ sealed HakoSourceTreeV1
+ root ParserNodeProductV1
+ compatibility fragment
+ next position after the closing brace
```

Only the selected live block grammar entry may issue this result. Arbitrary
constructors, `from_ast`, `from_json`, source substring rescans, and body
lookup by method name or ordinal are forbidden.

## Unpublished direct-method transaction

After the rich body result lands, one parser-private transaction temporarily
owns the complete direct-method draft:

```text
ParserDirectMethodTransactionV1
  exact method source site
  header payload
  ParserParameterListProductV1
  method-bound body result
        |
        +-- Typed exact body -> unpublished ParserBoxMethodDraftV1
        +-- CompatOnly       -> poison; draft 0
        +-- ParseError       -> poison; draft 0
```

The transaction completion is not a semantic/source seal. H3 remains the sole
declaration/source publication point and the sole inventory-ordinal issuer.
Do not introduce `SealedMethod`, a method sealer, or a second declaration
sealer.

Explicit-method arity is derived from the sealed parameter-list product. The
current caller-supplied scalar arity is compatibility debt and is retired at
the H2-I0/H3 cutover; both truths must not remain permanently.

Failure poisons the live method cursor/session. There is no ordinal rollback,
retry, partial draft insertion, or fallback. The complete unpublished parse
session is discarded.

## Dispositions

For the bounded row:

```text
Typed:
  exact one-statement `return <unsuffixed decimal integer>` body

CompatOnly:
  syntactically valid body outside the exact Typed cohort
  carries compatibility projection only
  never admitted by H2/H3

ParseError:
  malformed delimiter, malformed return, no parse progress, or lexical error
  carries no typed tree
```

There is no conversion from `CompatOnly` to `Typed`, and no mixed body may
publish a partial tree.

## Negative matrix

```text
empty body
bare return
multiple statements
return variable / call / binary / group / unary
float or suffixed integer
control statement
missing closing brace
integer scan with no digit / synthetic zero
parse progress zero
foreign method, tree, root, or parser session
forward, cycle, or unreachable node
missing or duplicate parameter/body attachment
body before parameters
double finish or post-close mutation
cursor/session finish while a method is live or poisoned
```

Valid non-cohort syntax becomes explicit `CompatOnly`; malformed syntax becomes
`ParseError`. Neither publishes a method draft in the bounded H2 path.

## Ordered implementation series

This is one BoxShape refactor series. It does not activate a new accepted
language form.

```text
H2-S2-R0
  split the 787-line ParserBox facade behavior-neutrally
  create room for a tiny rich-product delegation entry

H2-S2-S0
  typed integer lexical parts
  old scan_int becomes one-way JSON projection

H2-S2-S1
  ParserStmtCoreBox same-pass statement product
  exact Return(Present, LiteralInt) Typed row

H2-S2-I0
  ParserControlBox same-pass block product
  exact SourceBody seal + compatibility projection

H2-S3-I0
  method-bound body result + unpublished direct-method transaction
  negative paths leave declaration draft count unchanged

H2-I0
  connect the bounded ordinary Box direct-method grammar once

H3-I0
  sole final declaration/source seal
  derive explicit arity and retire scalar explicit-draft ingress
```

Each implementation slice updates focused tests, the owner README, this task
receipt or its child card, the current pointers, and any landed reference in
the same commit. All touched source files remain below 800 lines; new files
target at most 760 lines.

## First implementation row

Only `H2-S2-R0` opens next. It is behavior-neutral and may only move cohesive
ParserBox facade helpers into a dedicated owner, preserve imports/API/output,
and add a focused facade guard. It must not yet add rich products, parser
acceptance, source vocabulary, or a method connection.

## Guard requirements

The full H2-S2 guard eventually proves:

```text
exact live block entry is called
one statement dispatch per source statement
exact 3-node / 1-list Typed tree
next position is after the closing brace
compatibility JSON equals the existing Return(Int) projection
bare return remains typed-distinct from return 0
mixed Typed/CompatOnly publishes no partial tree
FuncScanner / StageB / JsonParser imports = 0
saved-source or substring body rescan = 0
sibling return-keyword parser = 0
arbitrary verified/body wrapper constructor = 0
all touched source files < 800 lines
```

## Nonclaims

```text
Take syntax or Home meaning
general method-body coverage
selected build-gate/static/interface/constructor/generated bodies
resolver FunctionOwner/body Facts/conformance
target / source-bound call / Recipe / CallSlot
Builder / MIR / CFG / PHI / runtime
production activation or fallback
```

## Closeout

The design question is closed as `accepted`. The missing work is relational
and result plumbing around existing grammar owners, not a new syntax algebra.
If any implementation row requires a second parse, JSON reconstruction,
source rescan, partial tree publication, or a second seal, stop and record
`NoSafeSlice` rather than widening the row.
