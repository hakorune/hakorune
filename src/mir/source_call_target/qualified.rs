use std::collections::BTreeMap;

use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};

use super::{
    QualifiedReceiverAdmissionV1, QualifiedStaticCallTargetErrorV1, QualifiedStaticReceiverV1,
    StaticImportAliasViewErrorV1, VerifiedQualifiedCallRouteFactsV1,
    VerifiedQualifiedStaticCallTargetV1, VerifiedSourceCallTargetV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1,
};

impl<'catalog> VerifiedStaticImportAliasViewV1<'catalog> {
    pub(crate) fn seal(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
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
        Ok(Self {
            catalog: declarations,
            aliases,
        })
    }
}

impl<'catalog> VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    pub(crate) fn seal_qualified<'facts>(
        imports: &'facts VerifiedStaticImportAliasViewV1<'catalog>,
        facts: impl IntoIterator<Item = VerifiedQualifiedCallRouteFactsV1<'facts, 'catalog>>,
    ) -> Result<Self, QualifiedStaticCallTargetErrorV1> {
        let declarations = imports.catalog;
        let mut facts = facts.into_iter().collect::<Vec<_>>();
        facts.sort_by(|left, right| {
            let left = left.call();
            let right = right.call();
            (left.caller(), left.site()).cmp(&(right.caller(), right.site()))
        });

        let mut rows = BTreeMap::new();
        for facts in facts {
            let call = facts.call();
            if !std::ptr::eq(call.catalog(), declarations) {
                return Err(QualifiedStaticCallTargetErrorV1::RouteFactCatalogMismatch {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                });
            }
            if !facts.matches_import_view(imports) {
                return Err(
                    QualifiedStaticCallTargetErrorV1::RouteFactImportViewMismatch {
                        caller: call.caller().clone(),
                        site: call.site().clone(),
                    },
                );
            }

            let row_key = (call.caller().clone(), call.site().clone());
            if rows.contains_key(&row_key) {
                return Err(QualifiedStaticCallTargetErrorV1::DuplicateCallSite {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                });
            }

            let source_receiver = facts.receiver();
            let receiver = match facts.admission() {
                QualifiedReceiverAdmissionV1::ImportedAlias => {
                    QualifiedStaticReceiverV1::ImportedAlias {
                        source_alias: source_receiver.into(),
                        canonical_owner: facts.canonical_owner().into(),
                    }
                }
                QualifiedReceiverAdmissionV1::DirectCanonicalOwner => {
                    QualifiedStaticReceiverV1::UnshadowedCanonicalOwner {
                        canonical_owner: facts.canonical_owner().into(),
                    }
                }
            };

            let canonical_owner = receiver.canonical_owner();
            let Some(target) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                canonical_owner,
                call.method(),
                call.arity() as usize,
            ) else {
                return Err(QualifiedStaticCallTargetErrorV1::TargetOutsideCatalog {
                    receiver: source_receiver.into(),
                    canonical_owner: canonical_owner.into(),
                    method: call.method().into(),
                    arity: call.arity(),
                });
            };

            rows.insert(
                row_key,
                VerifiedSourceCallTargetV1::Static(
                    VerifiedSourceStaticCallTargetV1::QualifiedStatic(
                        VerifiedQualifiedStaticCallTargetV1::new(receiver, target.key().clone()),
                    ),
                ),
            );
        }
        Ok(Self { declarations, rows })
    }
}
