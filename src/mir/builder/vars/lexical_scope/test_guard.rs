//! Compatibility guard for legacy test fixtures.
//!
//! Production lowering uses the lifetime-safe `try_with_lexical_scope`
//! transaction in the parent module.  The existing test fixtures predate
//! that API and rely on a drop guard while continuing to borrow the builder;
//! keep this adapter test-only until those fixtures are migrated.

#![allow(unsafe_code)]

use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) struct LexicalScopeGuard {
    builder: *mut MirBuilder,
}

impl LexicalScopeGuard {
    pub(in crate::mir::builder) fn new(builder: &mut MirBuilder) -> Self {
        builder.push_lexical_scope();
        Self { builder }
    }
}

impl Drop for LexicalScopeGuard {
    fn drop(&mut self) {
        // This compatibility adapter is compiled only for legacy tests.  The
        // production API has no raw-pointer scope owner or Drop restoration.
        unsafe { (&mut *self.builder).pop_lexical_scope_for_test() }
    }
}
