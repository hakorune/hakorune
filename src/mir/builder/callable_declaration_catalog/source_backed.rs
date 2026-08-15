//! Source-backed callable catalog issued from the final parser source loan.
//!
//! Canonical keys are lookup/selection keys. Pairing identity remains the
//! parser-issued opaque identity retained privately beside selected rows.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::parser::{
    CallableDeclarationIdentityV1, FinalCallableDeclarationModeV1,
    FinalCallableSemanticSyntaxLoanErrorV1, VerifiedFinalCallableProgramSourceV1,
};
use std::ptr;

use super::catalog::validate_parameters;
use super::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableCatalogBrandV1,
    SelectedCallableConsumptionRoleV1, SelectedNormalCallableKeyV1,
    SelectedNormalCallableSourceSiteV1, SelectedTopLevelFunctionKeyV1,
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
    VerifiedSelectedNormalCallableSourceInventoryV1,
};

#[derive(Debug)]
pub(crate) enum SourceBackedCallableCatalogIssueV1 {
    ParserSyntax(FinalCallableSemanticSyntaxLoanErrorV1),
    SourceShape,
    ArityOverflow,
    DuplicateCanonicalKey,
    DuplicateSelectedKey,
}

#[derive(Debug)]
struct SourceBackedSelectedIdentityV1 {
    key: SelectedNormalCallableKeyV1,
    identity: CallableDeclarationIdentityV1,
    role: SelectedCallableConsumptionRoleV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedSourceBackedSameModuleCallableCatalogV1 {
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    selected: Box<[SourceBackedSelectedIdentityV1]>,
}

impl VerifiedSourceBackedSameModuleCallableCatalogV1 {
    pub(crate) fn catalog(&self) -> &VerifiedSameModuleCallableDeclarationCatalogV1 {
        &self.catalog
    }

    pub(crate) fn selected_identities(
        &self,
    ) -> impl ExactSizeIterator<
        Item = (
            &SelectedNormalCallableKeyV1,
            &CallableDeclarationIdentityV1,
            SelectedCallableConsumptionRoleV1,
        ),
    > {
        self.selected
            .iter()
            .map(|row| (&row.key, &row.identity, row.role))
    }

    pub(crate) fn into_catalog(self) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
        self.catalog
    }
}

pub(crate) fn issue_source_backed_same_module_callable_catalog_v1(
    source: &VerifiedFinalCallableProgramSourceV1,
) -> Result<VerifiedSourceBackedSameModuleCallableCatalogV1, SourceBackedCallableCatalogIssueV1> {
    source
        .with_callable_semantic_syntax(|loan| {
            let main_expansion =
                crate::mir::builder::VerifiedRawRootExpansionV1::from_program(source.ast())
                    .map_err(|_| SourceBackedCallableCatalogIssueV1::SourceShape)?;
            let brand = SameModuleCallableCatalogBrandV1::fresh();
            let mut rows_by_key = BTreeMap::new();
            let mut static_lookup =
                BTreeMap::<(Box<str>, u32), Vec<CanonicalSameModuleCallableKeyV1>>::new();
            let mut selected_rows = Vec::new();
            let mut selected_identities = Vec::new();
            let mut selected_keys = BTreeSet::new();

            for row in loan.rows() {
                let ASTNode::FunctionDeclaration {
                    name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    ..
                } = row.declaration()
                else {
                    return Err(SourceBackedCallableCatalogIssueV1::SourceShape);
                };
                let arity = u32::try_from(params.len())
                    .map_err(|_| SourceBackedCallableCatalogIssueV1::ArityOverflow)?;
                let (selected_key, role) = match row.mode() {
                    FinalCallableDeclarationModeV1::TopLevel => {
                        let crate::parser::InitialCallableFinalSlotV1::TopLevel { statement } =
                            row.final_slot()
                        else {
                            return Err(SourceBackedCallableCatalogIssueV1::SourceShape);
                        };
                        (
                            SelectedNormalCallableKeyV1::TopLevel(
                                SelectedTopLevelFunctionKeyV1::new(
                                    statement as usize,
                                    name,
                                    params.len(),
                                ),
                            ),
                            SelectedCallableConsumptionRoleV1::ordinary(),
                        )
                    }
                    FinalCallableDeclarationModeV1::StaticBoxMethod
                    | FinalCallableDeclarationModeV1::InstanceBoxMethod => {
                        let owner = row
                            .owner_name()
                            .ok_or(SourceBackedCallableCatalogIssueV1::SourceShape)?;
                        let key = match row.mode() {
                            FinalCallableDeclarationModeV1::StaticBoxMethod => {
                                CanonicalSameModuleCallableKeyV1::static_box_method(
                                    owner, name, arity,
                                )
                            }
                            FinalCallableDeclarationModeV1::InstanceBoxMethod => {
                                CanonicalSameModuleCallableKeyV1::instance_box_method(
                                    owner, name, arity,
                                )
                            }
                            FinalCallableDeclarationModeV1::TopLevel => unreachable!(),
                        };
                        validate_parameters(&key, params, param_decls)
                            .map_err(|_| SourceBackedCallableCatalogIssueV1::SourceShape)?;
                        let declaration = VerifiedSameModuleCallableDeclarationV1 {
                            key: key.clone(),
                            params: params.clone().into_boxed_slice(),
                            param_decls: param_decls.clone().into_boxed_slice(),
                            return_type_name: return_type_name.clone().map(String::into_boxed_str),
                            body: body.clone().into_boxed_slice(),
                            uses: uses.clone().into_boxed_slice(),
                            attrs: attrs.clone(),
                        };
                        if rows_by_key.insert(key.clone(), declaration).is_some() {
                            return Err(SourceBackedCallableCatalogIssueV1::DuplicateCanonicalKey);
                        }
                        if matches!(row.mode(), FinalCallableDeclarationModeV1::StaticBoxMethod) {
                            static_lookup
                                .entry((name.clone().into_boxed_str(), arity))
                                .or_default()
                                .push(key.clone());
                        }
                        let role =
                            if let crate::mir::builder::VerifiedRawRootExpansionV1::App(main) =
                                &main_expansion
                            {
                                if ptr::eq(main.root().source(), row.declaration()) {
                                    continue;
                                }
                                if let Some(child) = main
                                    .static_children()
                                    .iter()
                                    .find(|child| ptr::eq(child.source(), row.declaration()))
                                {
                                    if row.mode() != FinalCallableDeclarationModeV1::StaticBoxMethod
                                    {
                                        return Err(
                                            SourceBackedCallableCatalogIssueV1::SourceShape,
                                        );
                                    }
                                    let observation = row
                                        .method_source_observation()
                                        .ok_or(SourceBackedCallableCatalogIssueV1::SourceShape)?;
                                    let crate::parser::InitialCallableFinalSlotV1::BoxMethod {
                                        statement,
                                        method,
                                    } = row.final_slot()
                                    else {
                                        return Err(
                                            SourceBackedCallableCatalogIssueV1::SourceShape,
                                        );
                                    };
                                    let source_site = observation.source_site();
                                    if !observation.identity().same_as(row.identity())
                                        || source_site.box_statement_ordinal() != statement
                                        || source_site.member_ordinal()
                                            != method.inventory_ordinal()
                                        || statement != child.statement_index()
                                        || method != child.method_ordinal()
                                    {
                                        return Err(
                                            SourceBackedCallableCatalogIssueV1::SourceShape,
                                        );
                                    }
                                    SelectedCallableConsumptionRoleV1::app_main_static_child(
                                        statement, method,
                                    )
                                } else {
                                    SelectedCallableConsumptionRoleV1::ordinary()
                                }
                            } else {
                                SelectedCallableConsumptionRoleV1::ordinary()
                            };
                        (SelectedNormalCallableKeyV1::Cataloged(key), role)
                    }
                };
                if !selected_keys.insert(selected_key.clone()) {
                    return Err(SourceBackedCallableCatalogIssueV1::DuplicateSelectedKey);
                }
                let site = match row.final_slot() {
                    crate::parser::InitialCallableFinalSlotV1::TopLevel { statement } => {
                        SelectedNormalCallableSourceSiteV1::ProgramFunction {
                            statement_index: statement as usize,
                        }
                    }
                    crate::parser::InitialCallableFinalSlotV1::BoxMethod { statement, .. } => {
                        SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
                            statement_index: statement as usize,
                            method_key: name.clone().into_boxed_str(),
                        }
                    }
                };
                selected_rows.push((selected_key.clone(), site));
                selected_identities.push(SourceBackedSelectedIdentityV1 {
                    key: selected_key,
                    identity: row.identity().clone(),
                    role,
                });
            }

            let static_keys_by_method_and_arity = static_lookup
                .into_iter()
                .map(|(lookup, mut keys)| {
                    keys.sort();
                    (lookup, keys.into_boxed_slice())
                })
                .collect();
            selected_identities.sort_by(|left, right| left.key.cmp(&right.key));
            let selected_source_inventory = VerifiedSelectedNormalCallableSourceInventoryV1::seal(
                brand.clone(),
                selected_rows,
                Vec::new(),
            );
            Ok(VerifiedSourceBackedSameModuleCallableCatalogV1 {
                catalog: VerifiedSameModuleCallableDeclarationCatalogV1 {
                    brand,
                    rows_by_key,
                    static_keys_by_method_and_arity,
                    selected_source_inventory,
                },
                selected: selected_identities.into_boxed_slice(),
            })
        })
        .map_err(SourceBackedCallableCatalogIssueV1::ParserSyntax)?
}
