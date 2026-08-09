# HOME-CONTEXTUAL-HTRIVIA-PARITY-R0

Status: parked P0; required before Take/Share parser activation
Date: 2026-08-10
Scheduling: parked behind the current Dynamic execution lane; required before
Take/Share parser activation

## Goal

Close the accepted `HTRIVIA` target for contextual Home syntax in Rust and
Hako without making `take`, `share`, or `release` global keywords.

## Change

Add one small Hako scanner owner, for example:

```text
lang/src/compiler/parser/scan/parser_horizontal_trivia_box.hako
```

Do not add another facade method to the already large `parser_box.hako`.
Release, and later Take/Share, import this scanner directly. Its only helper
has the semantic contract:

```text
skip_horizontal_trivia_no_line_terminator
  accepts: repeated space/tab and `/* ... */` with no CR/LF
  rejects/stops: newline, CRLF, comment containing a line terminator
```

The generic `skip_ws` helper is forbidden here. Rust tokenization already
discards block comments while retaining token line numbers in
`crates/hakorune_frontend_parser/src/tokenizer/{engine,whitespace}.rs`; the
implementation must prove the equivalent behavior rather than add a duplicate
Rust scanner.

Use this one helper for live Release and later Take/Share contextual lookahead.

## Shared corpus owner

Update both:

```text
grammar/language-v1-grammar-contract-corpus/ownership.toml
grammar/language-v1-registry.toml
```

Required rows:

```text
release /* same line */ root  -> Release
release /* line\nbreak */ root -> not contextual Release
release(root)                 -> ordinary Call
release (root)                -> ordinary Call
release\nroot                   -> separated ordinary syntax
local release = root          -> ordinary binding
release = root                -> ordinary assignment
root.release()                -> ordinary method
release root.field/root()/root+x/me -> stable existing rejects
```

Expected diagnostics/classification remain stable:

```text
ordinary/non-selected contextual rows:
  parser/release_contextual_not_selected

projected/call/trailing-token/me rows:
  parser/release_exact_root_required
```

The same corpus family later gains the accepted Take/Share matrix; inactive
syntax rows remain target-only until their named I0.

## Acceptance

- Rust and Hako normalized classification/rejection agree for every row;
- the same-line comment positive and multiline-comment negative are both
  present in the shared corpus, not parser-local tests only;
- `release(...)`, identifiers, assignments, and methods retain ordinary
  parsing permanently;
- `docs/reference/language/EBNF.md`, ownership reference, parser READMEs,
  focused tests, corpus, and implementation update in the same commit;
- no Home capability/availability/cleanup meaning is issued;
- all touched source files remain below 800 lines.

Focused gates include:

```text
python3 tools/language_v1/hako_corpus_batch.py \
  --bin target/debug/hakorune \
  --include-registry-row-fixtures release_statement Canonical

python3 tools/language_v1/hako_corpus_batch.py \
  --bin target/debug/hakorune \
  --include-registry-row-fixtures release_statement Compat2025

LANGV1_GRAMMAR_FULL=1 \
  bash tools/checks/language_v1_grammar_contract_substrate_guard.sh
```

The registry fixture-id arrays list every new corpus row.

## Stop

Any need to cross a line terminator, reserve a global token, scan source again
after parsing, or classify Home capability returns to D0.
