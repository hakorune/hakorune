//! Source-only co-seal for the selected pre-loop outer Call argument.
//!
//! This module joins the exact nested Integer association with the structural
//! `CallArgument` relation issued by the same catalog-backed Raw source view.
//! It deliberately owns no Builder state, destination, physical Call receipt,
//! or type-publication capability.

use std::ptr;

use crate::mir::source_call_target::VerifiedRawLocatedCallArgumentV1;

use super::{OwnedNestedInstanceResultRebindWitnessV1, PreparedPreloopNestedResultAssociationV1};

#[derive(Debug)]
pub(crate) struct PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    selected: VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
    association: PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
    _seal: PreparedPreloopLocatedArgumentSealV1,
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    selected: VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
    association: PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedArgumentStageV1,
    cause: PreloopLocatedArgumentErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopLocatedArgumentStageV1 {
    SourceView,
    SelectedSite,
    SelectedSyntax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopLocatedArgumentErrorV1 {
    ForeignView,
    SiteMismatch,
    SyntaxMismatch,
}

#[derive(Debug)]
struct PreparedPreloopLocatedArgumentSealV1(());

impl PreparedPreloopLocatedArgumentSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

/// Co-seal the structural outer `CallArgument` child with the exact selected
/// inner MethodCall association. The retained owners have no retry terminal.
pub(crate) fn prepare_preloop_located_argument_v1<'site, 'view, 'catalog>(
    selected: VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
    association: PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
) -> Result<
    PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    RejectedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
> {
    if !ptr::eq(selected.parent().view(), association.input().view()) {
        return Err(reject(
            selected,
            association,
            PreloopLocatedArgumentStageV1::SourceView,
            PreloopLocatedArgumentErrorV1::ForeignView,
        ));
    }
    if selected.child().site() != association.input().site() {
        return Err(reject(
            selected,
            association,
            PreloopLocatedArgumentStageV1::SelectedSite,
            PreloopLocatedArgumentErrorV1::SiteMismatch,
        ));
    }
    if !ptr::eq(selected.child().node(), association.input().node()) {
        return Err(reject(
            selected,
            association,
            PreloopLocatedArgumentStageV1::SelectedSyntax,
            PreloopLocatedArgumentErrorV1::SyntaxMismatch,
        ));
    }

    Ok(PreparedPreloopLocatedArgumentV1 {
        selected,
        association,
        _seal: PreparedPreloopLocatedArgumentSealV1::new(),
    })
}

impl<'site, 'view, 'catalog> PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    pub(crate) const fn selected(&self) -> &VerifiedRawLocatedCallArgumentV1<'view, 'catalog> {
        &self.selected
    }

    pub(crate) const fn association(
        &self,
    ) -> &PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog> {
        &self.association
    }

    pub(crate) fn discard(self) {}

    pub(crate) fn into_owned_rebind_witness(self) -> OwnedNestedInstanceResultRebindWitnessV1 {
        let Self {
            selected,
            association,
            ..
        } = self;
        selected.discard();
        association.into_owned_rebind_witness()
    }
}

impl<'site, 'view, 'catalog> RejectedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    pub(crate) const fn stage(&self) -> PreloopLocatedArgumentStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> PreloopLocatedArgumentErrorV1 {
        self.cause
    }

    pub(crate) fn discard(self) {}
}

fn reject<'site, 'view, 'catalog>(
    selected: VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
    association: PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedArgumentStageV1,
    cause: PreloopLocatedArgumentErrorV1,
) -> RejectedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    RejectedPreloopLocatedArgumentV1 {
        selected,
        association,
        stage,
        cause,
    }
}
