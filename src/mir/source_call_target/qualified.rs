use std::collections::BTreeMap;

use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};

use super::{
    QualifiedReceiverLexicalFactV1, QualifiedStaticCallCandidateV1,
    QualifiedStaticCallTargetErrorV1, QualifiedStaticReceiverV1, ReservedQualifiedReceiverRouteV1,
    StaticImportAliasViewErrorV1, VerifiedQualifiedStaticCallTargetV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1,
};

impl VerifiedStaticImportAliasViewV1 {
    pub(crate) fn seal(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        rows: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, StaticImportAliasViewErrorV1> {
        let mut rows = rows.into_iter().collect::<Vec<_>>();
        rows.sort();

        let mut aliases = BTreeMap::new();
        for (alias, canonical_owner) in rows {
            if alias.is_empty() {
                return Err(StaticImportAliasViewErrorV1::EmptyAlias);
            }
            if canonical_owner.is_empty() {
                return Err(StaticImportAliasViewErrorV1::EmptyCanonicalOwner {
                    alias: alias.into(),
                });
            }
            if aliases.contains_key(alias.as_str()) {
                return Err(StaticImportAliasViewErrorV1::DuplicateAlias {
                    alias: alias.into(),
                });
            }
            let owner_is_static = declarations.keys().any(|key| {
                key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod
                    && key.owner() == canonical_owner
            });
            if !owner_is_static {
                return Err(StaticImportAliasViewErrorV1::TargetOwnerOutsideCatalog {
                    alias: alias.into(),
                    canonical_owner: canonical_owner.into(),
                });
            }
            aliases.insert(alias.into_boxed_str(), canonical_owner.into_boxed_str());
        }
        Ok(Self { aliases })
    }
}

impl VerifiedSourceStaticCallTargetCatalogV1 {
    pub(crate) fn seal_qualified(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        imports: &VerifiedStaticImportAliasViewV1,
        candidates: impl IntoIterator<Item = QualifiedStaticCallCandidateV1>,
    ) -> Result<Self, QualifiedStaticCallTargetErrorV1> {
        let mut candidates = candidates.into_iter().collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            (left.caller(), left.site()).cmp(&(right.caller(), right.site()))
        });

        let mut rows = BTreeMap::new();
        for candidate in candidates {
            if declarations.declaration(candidate.caller()).is_none() {
                return Err(QualifiedStaticCallTargetErrorV1::CallerOutsideCatalog {
                    caller: candidate.caller().clone(),
                });
            }
            let row_key = (candidate.caller().clone(), candidate.site().clone());
            if rows.contains_key(&row_key) {
                return Err(QualifiedStaticCallTargetErrorV1::DuplicateCallSite {
                    caller: candidate.caller().clone(),
                    site: candidate.site().clone(),
                });
            }

            if candidate.reserved_route() != ReservedQualifiedReceiverRouteV1::Ordinary {
                return Err(QualifiedStaticCallTargetErrorV1::ReservedReceiverRoute {
                    receiver: candidate.receiver().into(),
                    route: candidate.reserved_route(),
                });
            }

            let receiver =
                if let Some(canonical_owner) = imports.canonical_owner(candidate.receiver()) {
                    QualifiedStaticReceiverV1::ImportedAlias {
                        source_alias: candidate.receiver().into(),
                        canonical_owner: canonical_owner.into(),
                    }
                } else {
                    if candidate.lexical_fact() == QualifiedReceiverLexicalFactV1::Bound {
                        return Err(
                            QualifiedStaticCallTargetErrorV1::DirectReceiverLexicallyShadowed {
                                receiver: candidate.receiver().into(),
                            },
                        );
                    }
                    QualifiedStaticReceiverV1::UnshadowedCanonicalOwner {
                        canonical_owner: candidate.receiver().into(),
                    }
                };

            let canonical_owner = receiver.canonical_owner();
            let Some(target) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                canonical_owner,
                candidate.method(),
                candidate.arity() as usize,
            ) else {
                return Err(QualifiedStaticCallTargetErrorV1::TargetOutsideCatalog {
                    receiver: candidate.receiver().into(),
                    canonical_owner: canonical_owner.into(),
                    method: candidate.method().into(),
                    arity: candidate.arity(),
                });
            };

            rows.insert(
                row_key,
                VerifiedSourceStaticCallTargetV1::QualifiedStatic(
                    VerifiedQualifiedStaticCallTargetV1::new(receiver, target.key().clone()),
                ),
            );
        }
        Ok(Self { rows })
    }
}
