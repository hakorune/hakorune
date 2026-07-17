use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::QualifiedStaticCallTargetErrorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QualifiedReceiverLexicalFactV1 {
    Unbound,
    Bound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReservedQualifiedReceiverRouteV1 {
    Ordinary,
    FastMem,
    MirIntrinsic,
    ReplIntrinsic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QualifiedStaticCallCandidateV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    receiver: Box<str>,
    method: Box<str>,
    arity: u32,
    lexical_fact: QualifiedReceiverLexicalFactV1,
    reserved_route: ReservedQualifiedReceiverRouteV1,
}

impl QualifiedStaticCallCandidateV1 {
    pub(crate) fn new(
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        receiver: &str,
        method: &str,
        arity: usize,
        lexical_fact: QualifiedReceiverLexicalFactV1,
        reserved_route: ReservedQualifiedReceiverRouteV1,
    ) -> Result<Self, QualifiedStaticCallTargetErrorV1> {
        if receiver.is_empty() {
            return Err(QualifiedStaticCallTargetErrorV1::EmptyReceiver);
        }
        if method.is_empty() {
            return Err(QualifiedStaticCallTargetErrorV1::EmptyMethod {
                receiver: receiver.into(),
            });
        }
        let arity =
            u32::try_from(arity).map_err(|_| QualifiedStaticCallTargetErrorV1::ArityOverflow {
                receiver: receiver.into(),
                method: method.into(),
            })?;
        Ok(Self {
            caller,
            site,
            receiver: receiver.into(),
            method: method.into(),
            arity,
            lexical_fact,
            reserved_route,
        })
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn receiver(&self) -> &str {
        &self.receiver
    }

    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) const fn lexical_fact(&self) -> QualifiedReceiverLexicalFactV1 {
        self.lexical_fact
    }

    pub(crate) const fn reserved_route(&self) -> ReservedQualifiedReceiverRouteV1 {
        self.reserved_route
    }
}

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
pub(crate) struct CurrentOwnerStaticCallCandidateV1 {
    pub(super) caller: CanonicalSameModuleCallableKeyV1,
    pub(super) site: SourceExprSiteV1,
    pub(super) receiver: CurrentOwnerStaticReceiverV1,
    pub(super) method: Box<str>,
    pub(super) arity: u32,
}

impl CurrentOwnerStaticCallCandidateV1 {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver(&self) -> CurrentOwnerStaticReceiverV1 {
        self.receiver
    }

    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }
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

#[derive(Debug)]
pub(crate) struct VerifiedStaticImportAliasViewV1 {
    pub(super) aliases: BTreeMap<Box<str>, Box<str>>,
}

impl VerifiedStaticImportAliasViewV1 {
    pub(crate) fn canonical_owner(&self, alias: &str) -> Option<&str> {
        self.aliases.get(alias).map(Box::as_ref)
    }

    pub(crate) fn len(&self) -> usize {
        self.aliases.len()
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedSourceStaticCallTargetCatalogV1 {
    pub(super) rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedSourceStaticCallTargetV1,
    >,
}

impl VerifiedSourceStaticCallTargetCatalogV1 {
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
