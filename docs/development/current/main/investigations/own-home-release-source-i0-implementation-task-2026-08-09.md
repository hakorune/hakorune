---
Status: implementation preflight; production 0
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-syntax-d0-design-task-2026-08-09.md`
Design: `docs/development/current/main/investigations/own-home-source-event-d0-design-task-2026-08-09.md`
Reference: `docs/reference/language/ownership.md`
---

# OWN-HOME-RELEASE-SOURCE-I0

## Objective

Implement exactly one parser/source-carrier shape:

```hako
release root
```

The row proves only that both parsers can classify the same contextual
statement and that the existing rich body transaction can issue one AST-free
source carrier.  It does not prove that `root` owns a Home or that release is
legal.

## Required preflight census

Before editing code, identify and record the exact owners for:

```text
Rust statement dispatch and same-line lookahead
Rust frontend AST syntax representation
Hako parser statement/source-carrier representation
AST JSON/compat exhaustive-match impact
ParserResolverBodyTransactionV1 syntax lease
body source root/item ordinal identity
Rust/Hako normalized parity fixture
```

If the only available route requires encoding release as an ordinary Call,
text scan, JSON repair, or downstream name match, stop with `NoSafeSlice` and
open a BoxShape D0.  Do not implement a compatibility shortcut.

## Accepted parser shape

```text
statement head IDENT("release")
+ no line terminator
+ IDENT root
+ statement end
```

Contextual recognition commits after the exact head/root shape.  Extra tokens
such as `.`, `[`, `(`, `+`, or `,` then produce a stable release syntax error;
there is no ordinary-expression fallback after partial recognition.

The following remain ordinary syntax:

```text
release(root)
release (root)
obj.release()
release = other
local release = 1
Build.release()
```

`release` followed by a line terminator is not Home syntax.  Existing ordinary
grammar decides the separated tokens.

## Source carrier

The parser-private issuer returns one owned, non-`Clone`-by-authority carrier:

```text
ReleaseStatementSourceV1
  parser provenance
  exact instance-method source site
  body statement ordinal
  root identifier spelling/site
```

It is issued only inside:

```text
ParserResolverBodyTransactionV1::with_direct_method_syntax
```

The callback consumes rich syntax and returns only AST-free data.  The carrier
has no `HomeRoot`, `BindingRef`, availability, terminality, fini, cleanup,
Builder, MIR, or runtime fields.

## Implementation structure

Prefer responsibility-local files and keep every source file below 800 lines:

```text
Rust parser statement classifier / tests
frontend AST syntax node or exact parser-private syntax relation
parser body-source carrier / tests
Hako parser equivalent / parity fixture
structural guard
```

Do not append a large negative matrix to an already large dispatcher; split
tests and the contextual classifier into small owner files when necessary.

## Required tests

```text
positive:
  release root
  exact parser provenance, method site, body ordinal, root spelling

ordinary:
  release(root)
  release (root)
  obj.release()
  local release = 1
  release = root
  Build.release()

not contextual across newline:
  release\nroot

stable contextual rejects:
  release root.field
  release root[index]
  release root()
  release root + other
  release root, other
  release me

authority rejects:
  foreign parser provenance
  duplicate carrier
  source-site/body-ordinal mismatch
```

Rust and Hako parsers must emit the same normalized classification.  One-sided
activation is a failing gate.

## Structural guards

```text
no TokenType::RELEASE
no FunctionCall("release") desugaring
no raw source-text scan
no AST-only post-hoc reconstruction
no resolver/MIR/backend name match
no default/empty Verified product
no parser fallback after contextual commitment
no source file over 800 lines
```

## Same-slice documentation closeout

The implementation commit must update:

```text
docs/reference/language/EBNF.md
docs/reference/language/ownership.md
docs/reference/language/status-index.md
docs/reference/language/quick-reference.md
src/parser/README.md
the Hako parser/source-carrier README
CURRENT_STATE.toml / 10-Now.md / task map
```

Only then may the release row be described as parser-live.  Home-demand,
Home Flow, DropPlan, performance, and production execution remain 0.
