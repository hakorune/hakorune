//! Atomic owner of one canonical module Builder candidate.
//!
//! The caller's Builder is never mutated until the complete candidate module
//! has passed Builder lowering and compiler post-processing. Dropping this
//! session on any error discards all module, function, scope, and ID effects.

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::mir::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};




static NEXT_COMPILER_DOMAIN: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum SourceBindingErrorV1 {
    DomainExhausted,
    OrdinalExhausted,
}

impl std::fmt::Display for SourceBindingErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][source_binding] {self:?}")
    }
}

impl std::error::Error for SourceBindingErrorV1 {}

#[derive(Debug)]
pub(super) struct InvocationIdentityIssuerV1 {
    domain: Option<NonZeroU64>,
    next_ordinal: u64,
}

impl InvocationIdentityIssuerV1 {
    pub(super) const fn new() -> Self {
        Self {
            domain: None,
            next_ordinal: 1,
        }
    }

    /// Mint one invocation token for normal compilation.
    pub(super) fn issue_raw(&mut self) -> Result<ModuleInvocationTokenV1, SourceBindingErrorV1> {
        let domain = match self.domain {
            Some(domain) => domain,
            None => {
                let raw = NEXT_COMPILER_DOMAIN
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                        value.checked_add(1)
                    })
                    .map_err(|_| SourceBindingErrorV1::DomainExhausted)?;
                let domain = NonZeroU64::new(raw).ok_or(SourceBindingErrorV1::DomainExhausted)?;
                self.domain = Some(domain);
                domain
            }
        };
        let ordinal =
            NonZeroU64::new(self.next_ordinal).ok_or(SourceBindingErrorV1::OrdinalExhausted)?;
        self.next_ordinal = self
            .next_ordinal
            .checked_add(1)
            .ok_or(SourceBindingErrorV1::OrdinalExhausted)?;
        Ok(ModuleInvocationTokenV1::from_issued(
            domain, ordinal, ModuleInvocationFamilyV1::Raw,
        ))
    }
}
