---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / tokenizer facade wrapper design.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1196-BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-TOKENIZER-FACADE-WRAPPER-DESIGN-001

## Decision

When the tokenizer owner bundle moves, keep a main-crate wrapper instead of a
direct type re-export:

```text
main_facade_type=src/tokenizer/mod.rs::NyashTokenizer
inner_type=hakorune_frontend_parser::tokenizer::NyashTokenizer
wrapper_new_installs_runtime_host=1
wrapper_delegates_tokenize=1
```

Public main-crate API preserved:

```text
crate::tokenizer::NyashTokenizer::new(input)
crate::tokenizer::NyashTokenizer::tokenize()
crate::tokenizer::{Token,TokenType,TokenizeError}
```

The wrapper must not expose inner fields. Parser callers currently only require
`new` and `tokenize`, so the wrapper can keep a narrow surface.

## Shape

Future main-crate facade:

```text
pub struct NyashTokenizer {
    inner: hakorune_frontend_parser::tokenizer::NyashTokenizer,
}

impl NyashTokenizer {
    pub fn new(input: impl Into<String>) -> Self {
        crate::frontend_host::install_frontend_parser_host();
        Self { inner: hakorune_frontend_parser::tokenizer::NyashTokenizer::new(input) }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizeError> {
        self.inner.tokenize()
    }
}
```

The extracted crate owns tokenizer internals and private helper methods. Tests
that need private tokenizer helpers move with the extracted implementation.

## Next

```text
selected_next_task=BUILD-FRONTEND-TOKENIZER-OWNER-BUNDLE-MOVE-001
purpose=move NyashTokenizer and tokenizer impl modules into hakorune-frontend-parser, leaving main wrapper facade
implementation_allowed=1
```

Non-goals:

```text
do_not_move_parser_files=1
do_not_expand_main_facade_beyond_new_and_tokenize=1
do_not_add_runtime_dependency_to_frontend_parser=1
```
