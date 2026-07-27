use std::collections::BTreeMap;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QualifiedStaticReceiverV1 {
    ImportedAlias {
        source_alias: Box<str>,
        canonical_owner: Box<str>,
    },
    UnshadowedCanonicalOwner {
        canonical_owner: Box<str>,
    },
}

impl QualifiedStaticReceiverV1 {
    pub(crate) fn canonical_owner(&self) -> &str {
        match self {
            Self::ImportedAlias {
                canonical_owner, ..
            }
            | Self::UnshadowedCanonicalOwner { canonical_owner } => canonical_owner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedQualifiedStaticCallTargetV1 {
    receiver: QualifiedStaticReceiverV1,
    target: CanonicalSameModuleCallableKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CurrentOwnerStaticReceiverV1 {
    /// The parser's semantic `ASTNode::Me` receiver.
    ///
    /// Canonical source `me` and deprecated `this` normalize to this same AST
    /// shape; original token spelling is intentionally not an authority here.
    CanonicalMe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCurrentOwnerStaticCallTargetV1 {
    receiver: CurrentOwnerStaticReceiverV1,
    target: CanonicalSameModuleCallableKeyV1,
}

impl VerifiedCurrentOwnerStaticCallTargetV1 {
    pub(super) const fn new(
        receiver: CurrentOwnerStaticReceiverV1,
        target: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        Self { receiver, target }
    }

    pub(crate) const fn receiver(&self) -> CurrentOwnerStaticReceiverV1 {
        self.receiver
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

impl VerifiedQualifiedStaticCallTargetV1 {
    pub(super) const fn new(
        receiver: QualifiedStaticReceiverV1,
        target: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        Self { receiver, target }
    }

    pub(crate) const fn receiver(&self) -> &QualifiedStaticReceiverV1 {
        &self.receiver
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedSourceStaticCallTargetV1 {
    QualifiedStatic(VerifiedQualifiedStaticCallTargetV1),
    CurrentOwnerStatic(VerifiedCurrentOwnerStaticCallTargetV1),
}

impl VerifiedSourceStaticCallTargetV1 {
    /// Returns the canonical callable selected by the already-sealed route.
    ///
    /// Consumers must not reopen receiver precedence from the variant.  This
    /// projection is the sole route-neutral target identity view.
    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::QualifiedStatic(target) => target.target(),
            Self::CurrentOwnerStatic(target) => target.target(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedStaticImportAliasViewV1<'catalog> {
    pub(super) catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    pub(super) aliases: BTreeMap<Box<str>, Box<str>>,
}

impl VerifiedStaticImportAliasViewV1<'_> {
    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        std::ptr::eq(self.catalog, declarations)
    }

    pub(crate) fn canonical_owner(&self, alias: &str) -> Option<&str> {
        self.aliases.get(alias).map(Box::as_ref)
    }

    pub(crate) fn len(&self) -> usize {
        self.aliases.len()
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    pub(super) declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    pub(super) rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedSourceStaticCallTargetV1,
    >,
}

impl VerifiedSourceStaticCallTargetCatalogV1<'_> {
    /// Tests whether this catalog retains the exact declaration authority.
    ///
    /// Key equality is insufficient: equal declarations from another source
    /// unit must not be composed with these exact source-site rows.
    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        std::ptr::eq(self.declarations, declarations)
    }

    pub(crate) fn target(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedSourceStaticCallTargetV1> {
        self.rows.get(&(caller.clone(), site.clone()))
    }

    pub(crate) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
            &VerifiedSourceStaticCallTargetV1,
        ),
    > {
        self.rows.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }
}
