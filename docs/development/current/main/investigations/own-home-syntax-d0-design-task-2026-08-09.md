---
Status: accepted language target; parser production 0
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-source-event-d0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
Reference: `docs/reference/language/ownership.md`
---

# OWN-HOME-SYNTAX-D0

## Decision

Hakorune exposes three ownership-changing requests in three distinct source
positions:

```text
declaration: take node: Node  -> destination requests one Home
expression:  share node       -> request one independent owner
statement:   release node     -> request ending one current whole-root Home
```

`take`, `share`, and `release` remain ordinary `IDENT` spellings.  None is a
global lexer keyword and no `TokenType::TAKE`, `SHARE`, or `RELEASE` is added.
Parser recognition is contextual, same-line, and syntax-only.  It never proves
Home capability, availability, sharing representation, or terminality.

This Decision accepts the language target only.  Rust and Hako parser
production remains 0 until each bounded implementation row lands with parity.

## Canonical grammar target

`HTRIVIA` means horizontal whitespace or a comment that contains no line
terminator.

```ebnf
param
  := take_param
   | ordinary_param

take_param
  := TAKE_CTX HTRIVIA IDENT HTRIVIA ':' type_ref

ordinary_param
  := IDENT (HTRIVIA ':' type_ref)?

TAKE_CTX
  := IDENT("take")
     when parser is at parameter head and the same-line lookahead is
     IDENT("take") HTRIVIA IDENT HTRIVIA ':'

unary_expr
  := ordinary_prefix unary_expr
   | share_expr
   | postfix_expr

share_expr
  := SHARE_CTX HTRIVIA non_group_postfix_expr

non_group_postfix_expr
  := non_group_primary postfix_tail*

non_group_primary
  := IDENT
   | 'me'
   | new_expr

SHARE_CTX
  := IDENT("share")
     when parser is at expression-prefix position, the next same-line token
     is in FIRST(non_group_primary), and that token is not '('

stmt
  := release_stmt
   | existing_stmt

release_stmt
  := RELEASE_CTX HTRIVIA IDENT stmt_end

RELEASE_CTX
  := IDENT("release")
     when parser is at statement head and the next same-line token is IDENT
```

The canonical grammar does not add `this`; Hakorune source uses `me` and the
existing `this` spelling is compatibility/deprecation input.  `new_expr` may
be syntactically observable, but a later Home verifier may reject redundant
or unsupported `share new ...` semantics.

## Take

`take` is a declaration contract, not an expression or source type.

```hako
adopt(take node: Node) { }
```

`adopt` is an ordinary callable name.  The parameter declaration requires an
explicit type and the contextual head does not cross a line terminator.

```text
take node: Node  = contextual Home-demand parameter target
take: Node       = ordinary parameter named take
take             = ordinary untyped parameter named take
take(node)       = ordinary call
obj.take()       = ordinary method
local take = x   = ordinary binding
```

At parameter head, same-line `take IDENT` without the required `:` is a stable
malformed-take diagnostic.  It is not repaired into two ordinary parameters.

Call-site `take`, `return take x`, `take place_expr`, default-valued take
parameters, and consuming receivers are outside HomeV1.

The parser declaration handoff owns the typed `Take` syntax row.  A body
observer must never rediscover it from parameter text or AST names.

## Share

`share` is one contextual expression prefix over one non-group postfix
operand:

```hako
share node
share me.root
share makeNode()
share array[index]
return share node
adopt(share node)
```

`adopt(share node)` is ordinary call composition.  The name `adopt` has no
language authority.

Postfix binds inside `share`; infix operators bind outside it:

```text
share obj.field() = Share(MethodCall(obj, field))
share obj + y     = Add(Share(obj), y)
- share obj       = Minus(Share(obj))
share - obj       = ordinary subtraction using a variable named share
```

`share(` is permanently ordinary call syntax, regardless of whitespace:

```text
share(obj)   = ordinary call
share (obj)  = ordinary call
```

This syntax family will never reinterpret `share (expr)` as grouped ownership.
A future grouped ownership form must use a non-conflicting spelling selected
by another language Decision.  Until then, bind a complex expression to a
local and share that local.

## Release

`release root` remains the sole accepted explicit early Home-end spelling.

```text
release root       = contextual statement target
release(root)      = ordinary call
release (root)     = ordinary call
obj.release()      = ordinary method
local release = 1  = ordinary binding
Build.release()    = ordinary method
```

After contextual recognition, exactly one identifier and statement end are
required.  `release root.field`, `release root()`, and `release root + x` are
syntax errors; the parser does not fall back to an ordinary expression after
partially recognizing the contextual statement.

## No-line-terminator rule

Contextual recognition never crosses a line terminator:

```text
take <newline> node: Node
share <newline> node
release <newline> root
```

These token sequences are not ownership syntax.  Existing ordinary grammar
decides whether the separated tokens form valid ordinary statements; the Home
parser does not force an ownership-specific error.

Rust and Hako parsers must use a horizontal-trivia check for these boundaries.
An existing generic whitespace skipper that consumes newlines is not valid for
contextual Home recognition.

## Authority boundary

| Layer | Owns | Does not own |
| --- | --- | --- |
| Lexer | identifiers, punctuation, source position | Home vocabulary classification |
| Declaration parser/handoff | typed `Take` modifier site and parameter source identity | Home demand or ABI |
| Body parser transaction | typed `Share`/`Release` source carrier and exact source path | capability, alias, availability |
| Resolver/Home issuer | resolved type/root, capability, destination relation | re-parsing spelling |
| Home Flow | path-sensitive state transition and End/Forward witness | source grammar or MIR |

`share` and `release` body carriers use the existing
`ParserResolverBodyTransactionV1::with_direct_method_syntax` boundary.  The
callback may return only owned AST-free products.  `take` stays in the
declaration handoff and is not emitted from that body callback.

## Required parser parity matrix

```text
take node: Node   contextual parameter
take: Node        ordinary parameter
take(node)        ordinary call
take node         stable malformed-take error
take\nnode: Node   not contextual take

share node        contextual prefix
share(node)       ordinary call
share (node)      ordinary call
share - node      ordinary subtraction
share[0]          ordinary indexing
share\nnode       not contextual share

release root      contextual statement
release(root)     ordinary call
release (root)    ordinary call
release\nroot     not contextual release
```

Both parsers must produce the same normalized classification.  One-parser
activation, raw-string reconstruction downstream, and silent fallback are
forbidden.

## Ordered implementation tasks

```text
1. OWN-HOME-SYNTAX-D0
   this accepted target; production remains 0

2. OWN-HOME-RELEASE-SOURCE-I0
   exact `release IDENT` statement carrier through the existing body
   transaction; Rust/Hako parity and same-slice reference receipt

3. OWN-HOME-TAKE-DECL-SYNTAX-I0
   typed parameter modifier in the declaration handoff; no Home ABI issuance

4. OWN-HOME-CAPABILITY-TAXONOMY-D0
   Create/Consume/Share/Forward/Escape/End meanings and dispositions

5. OWN-HOME-ABI-HOME-DEMAND-I0
   resolve the take declaration into one exact Home-demand capability

6. CALLABLE-BODY-HOME-FLOW-LINEAR-I0
   Available -> End/Forward for the first linear owning-root cohort

7. OWN-HOME-SHARE-REPRESENTATION-D0
   select Shared representation and destination/lifetime compatibility

8. OWN-HOME-SHARE-EXPR-SYNTAX-I0
   typed non-group postfix share carrier; no runtime/materialization fallback

9. CALLABLE-BODY-HOME-FLOW-CFG-D0
   branch/loop/backedge/Maybe-join authority

10. CALLABLE-BODY-EXECUTION-COSEAL
    only after the required axis issuers are complete
```

## Explicit non-claims

This Decision does not implement parser productions, add AST variants, issue
Home ABI/Flow products, choose Shared representation, or activate Builder,
MIR, runtime, backend, DropPlan, fallback, retry, or production routes.
