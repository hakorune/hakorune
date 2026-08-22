//! Identity-bound Deferred transport for one selected-callable resolver batch.
//!
//! Parser owners create each input beside its borrowed syntax view. The
//! resolver observes only cause/site and moves the opaque source identity into
//! a Deferred row; it never reconstructs identity from names, paths, or input
//! order.

use crate::parser::{CallableDeclarationIdentityV1, ConstructorSourceIdV1};

use super::{FunctionSyntaxViewV1, SourceResolverDeferredV1};

#[derive(Debug, Clone)]
pub(crate) enum SelectedCallableResolverSourceIdentityV1 {
    Callable {
        identity: CallableDeclarationIdentityV1,
        diagnostic_owner: Option<Box<str>>,
        diagnostic_name: Box<str>,
    },
    Constructor {
        source_id: ConstructorSourceIdV1,
        diagnostic_box: Box<str>,
        diagnostic_key: Box<str>,
    },
}

#[derive(Debug)]
pub(crate) struct SelectedCallableResolverInputV1<'source> {
    source: SelectedCallableResolverSourceIdentityV1,
    view: FunctionSyntaxViewV1<'source>,
}

impl<'source> SelectedCallableResolverInputV1<'source> {
    pub(crate) fn callable(
        identity: CallableDeclarationIdentityV1,
        diagnostic_owner: Option<&str>,
        diagnostic_name: &str,
        view: FunctionSyntaxViewV1<'source>,
    ) -> Self {
        Self {
            source: SelectedCallableResolverSourceIdentityV1::Callable {
                identity,
                diagnostic_owner: diagnostic_owner.map(Box::<str>::from),
                diagnostic_name: diagnostic_name.into(),
            },
            view,
        }
    }

    pub(crate) fn constructor(
        source_id: ConstructorSourceIdV1,
        diagnostic_box: &str,
        diagnostic_key: &str,
        view: FunctionSyntaxViewV1<'source>,
    ) -> Self {
        Self {
            source: SelectedCallableResolverSourceIdentityV1::Constructor {
                source_id,
                diagnostic_box: diagnostic_box.into(),
                diagnostic_key: diagnostic_key.into(),
            },
            view,
        }
    }

    pub(super) fn source(&self) -> &SelectedCallableResolverSourceIdentityV1 {
        &self.source
    }

    pub(super) const fn view(&self) -> FunctionSyntaxViewV1<'source> {
        self.view
    }
}

#[derive(Debug)]
pub(crate) struct SelectedCallableResolverDeferredV1 {
    source: SelectedCallableResolverSourceIdentityV1,
    observation: SourceResolverDeferredV1,
}

impl SelectedCallableResolverDeferredV1 {
    pub(super) fn from_parts(
        source: SelectedCallableResolverSourceIdentityV1,
        observation: SourceResolverDeferredV1,
    ) -> Self {
        Self {
            source,
            observation,
        }
    }

    pub(crate) const fn source(&self) -> &SelectedCallableResolverSourceIdentityV1 {
        &self.source
    }

    pub(crate) const fn observation(&self) -> &SourceResolverDeferredV1 {
        &self.observation
    }
}

impl SelectedCallableResolverSourceIdentityV1 {
    pub(crate) fn callable_identity(&self) -> Option<&CallableDeclarationIdentityV1> {
        match self {
            Self::Callable { identity, .. } => Some(identity),
            Self::Constructor { .. } => None,
        }
    }

    pub(crate) fn constructor_source_id(&self) -> Option<&ConstructorSourceIdV1> {
        match self {
            Self::Callable { .. } => None,
            Self::Constructor { source_id, .. } => Some(source_id),
        }
    }
}

/// Structurally non-empty Deferred batch. No default/empty constructor exists.
#[derive(Debug)]
pub(crate) struct SelectedCallableResolverDeferredBatchV1 {
    first: SelectedCallableResolverDeferredV1,
    rest: Box<[SelectedCallableResolverDeferredV1]>,
}

impl SelectedCallableResolverDeferredBatchV1 {
    pub(super) fn from_non_empty_parts(
        first: SelectedCallableResolverDeferredV1,
        rest: Box<[SelectedCallableResolverDeferredV1]>,
    ) -> Self {
        Self { first, rest }
    }

    pub(crate) const fn first(&self) -> &SelectedCallableResolverDeferredV1 {
        &self.first
    }

    pub(crate) fn rest(&self) -> &[SelectedCallableResolverDeferredV1] {
        &self.rest
    }

    pub(crate) fn len(&self) -> usize {
        1 + self.rest.len()
    }
}
