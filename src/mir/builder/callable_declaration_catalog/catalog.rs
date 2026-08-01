use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableDeclarationCatalogErrorV1,
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1, SelectedNormalCallableSourceSiteV1,
    SelectedTopLevelFunctionKeyV1, VerifiedSelectedNormalCallableSourceInventoryV1,
};

#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct VerifiedSameModuleCallableDeclarationV1 {
    key: CanonicalSameModuleCallableKeyV1,
    params: Box<[String]>,
    param_decls: Box<[ParamDecl]>,
    return_type_name: Option<Box<str>>,
    body: Box<[ASTNode]>,
    uses: Box<[String]>,
    attrs: DeclarationAttrs,
}

#[cfg_attr(not(test), allow(dead_code))]
impl VerifiedSameModuleCallableDeclarationV1 {
    pub(crate) const fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }

    pub(crate) fn params(&self) -> &[String] {
        &self.params
    }

    pub(crate) fn param_decls(&self) -> &[ParamDecl] {
        &self.param_decls
    }

    pub(crate) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(crate) fn body(&self) -> &[ASTNode] {
        &self.body
    }

    pub(crate) fn uses(&self) -> &[String] {
        &self.uses
    }

    pub(crate) const fn attrs(&self) -> &DeclarationAttrs {
        &self.attrs
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedSameModuleCallableDeclarationCatalogV1 {
    rows_by_key:
        BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationV1>,
    static_keys_by_method_and_arity:
        BTreeMap<(Box<str>, u32), Box<[CanonicalSameModuleCallableKeyV1]>>,
    selected_source_inventory: VerifiedSelectedNormalCallableSourceInventoryV1,
}

#[cfg_attr(not(test), allow(dead_code))]
impl VerifiedSameModuleCallableDeclarationCatalogV1 {
    pub(crate) fn seal_program(
        root: &ASTNode,
    ) -> Result<Self, SameModuleCallableDeclarationCatalogErrorV1> {
        let ASTNode::Program { statements, .. } = root else {
            return Err(SameModuleCallableDeclarationCatalogErrorV1::ProgramRequired);
        };

        Self::seal_statements(statements, true)
    }

    /// Seals the Builder's existing root surface without widening declaration
    /// discovery. Non-container roots own a verified empty inventory.
    pub(crate) fn seal_root(
        root: &ASTNode,
    ) -> Result<Self, SameModuleCallableDeclarationCatalogErrorV1> {
        match root {
            ASTNode::Program { statements, .. } => Self::seal_statements(statements, true),
            ASTNode::BoxDeclaration { .. } => {
                Self::seal_statements(std::slice::from_ref(root), false)
            }
            _ => Self::seal_statements(&[], false),
        }
    }

    fn seal_statements(
        statements: &[ASTNode],
        collect_selected_program_sources: bool,
    ) -> Result<Self, SameModuleCallableDeclarationCatalogErrorV1> {
        let mut rows_by_key = BTreeMap::new();
        let mut static_keys_by_method_and_arity =
            BTreeMap::<(Box<str>, u32), Vec<CanonicalSameModuleCallableKeyV1>>::new();
        let mut box_owners = BTreeSet::new();
        let mut selected_source_rows = Vec::new();

        for (statement_index, statement) in statements.iter().enumerate() {
            if collect_selected_program_sources {
                if let ASTNode::FunctionDeclaration { name, params, .. } = statement {
                    let key =
                        SelectedTopLevelFunctionKeyV1::new(statement_index, name, params.len());
                    selected_source_rows.push((
                        SelectedNormalCallableKeyV1::TopLevel(key),
                        SelectedNormalCallableSourceSiteV1::ProgramFunction { statement_index },
                    ));
                    continue;
                }
            }
            let ASTNode::BoxDeclaration {
                name,
                methods,
                is_static,
                is_sync,
                is_record,
                ..
            } = statement
            else {
                continue;
            };
            if *is_sync || *is_record {
                continue;
            }
            if !box_owners.insert(name.clone()) {
                return Err(
                    SameModuleCallableDeclarationCatalogErrorV1::DuplicateBoxOwner {
                        owner: name.clone(),
                    },
                );
            }

            let mut method_rows = methods.iter().collect::<Vec<_>>();
            method_rows.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (map_name, declaration) in method_rows {
                let ASTNode::FunctionDeclaration {
                    name: declaration_name,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    ..
                } = declaration
                else {
                    return Err(
                        SameModuleCallableDeclarationCatalogErrorV1::MethodMustBeFunction {
                            owner: name.clone(),
                            method: map_name.clone(),
                        },
                    );
                };
                let ASTNode::FunctionDeclaration {
                    is_static: method_is_static,
                    ..
                } = declaration
                else {
                    unreachable!()
                };
                let namespace = if *is_static {
                    SameModuleCallableNamespaceV1::StaticBoxMethod
                } else if !*method_is_static {
                    SameModuleCallableNamespaceV1::InstanceBoxMethod
                } else {
                    continue;
                };
                if map_name != declaration_name {
                    return Err(
                        SameModuleCallableDeclarationCatalogErrorV1::MethodNameMismatch {
                            owner: name.clone(),
                            map_name: map_name.clone(),
                            declaration_name: declaration_name.clone(),
                        },
                    );
                }
                let arity = u32::try_from(params.len()).map_err(|_| {
                    SameModuleCallableDeclarationCatalogErrorV1::ArityOverflow {
                        owner: name.clone(),
                        method: map_name.clone(),
                    }
                })?;
                let key = match namespace {
                    SameModuleCallableNamespaceV1::StaticBoxMethod => {
                        CanonicalSameModuleCallableKeyV1::static_box_method(name, map_name, arity)
                    }
                    SameModuleCallableNamespaceV1::InstanceBoxMethod => {
                        CanonicalSameModuleCallableKeyV1::instance_box_method(name, map_name, arity)
                    }
                };
                validate_parameters(&key, params, param_decls)?;
                let row = VerifiedSameModuleCallableDeclarationV1 {
                    key: key.clone(),
                    params: params.clone().into_boxed_slice(),
                    param_decls: param_decls.clone().into_boxed_slice(),
                    return_type_name: return_type_name.clone().map(String::into_boxed_str),
                    body: body.clone().into_boxed_slice(),
                    uses: uses.clone().into_boxed_slice(),
                    attrs: attrs.clone(),
                };
                if rows_by_key.insert(key.clone(), row).is_some() {
                    return Err(
                        SameModuleCallableDeclarationCatalogErrorV1::DuplicateCanonicalKey(key),
                    );
                }
                if collect_selected_program_sources && name != "Main" {
                    selected_source_rows.push((
                        SelectedNormalCallableKeyV1::Cataloged(key.clone()),
                        SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
                            statement_index,
                            method_key: map_name.clone().into_boxed_str(),
                        },
                    ));
                }
                if namespace == SameModuleCallableNamespaceV1::StaticBoxMethod {
                    static_keys_by_method_and_arity
                        .entry((map_name.clone().into_boxed_str(), arity))
                        .or_default()
                        .push(key);
                }
            }
        }

        let static_keys_by_method_and_arity = static_keys_by_method_and_arity
            .into_iter()
            .map(|(lookup, mut keys)| {
                keys.sort();
                (lookup, keys.into_boxed_slice())
            })
            .collect();
        Ok(Self {
            rows_by_key,
            static_keys_by_method_and_arity,
            selected_source_inventory: VerifiedSelectedNormalCallableSourceInventoryV1::seal(
                selected_source_rows,
            ),
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.rows_by_key.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows_by_key.is_empty()
    }

    pub(in crate::mir::builder) const fn selected_source_inventory(
        &self,
    ) -> &VerifiedSelectedNormalCallableSourceInventoryV1 {
        &self.selected_source_inventory
    }

    pub(crate) fn declaration(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&VerifiedSameModuleCallableDeclarationV1> {
        self.rows_by_key.get(key)
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &CanonicalSameModuleCallableKeyV1> {
        self.rows_by_key.keys()
    }

    /// Borrowed complete declaration inventory in canonical-key order.
    ///
    /// This is an observation surface only.  Static result inference remains
    /// owned by `static_declarations`; instance rows may use this inventory as
    /// exact caller/site owners without becoming result candidates.
    pub(crate) fn declarations(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedSameModuleCallableDeclarationV1,
        ),
    > {
        self.rows_by_key.iter()
    }

    /// Borrowed static-only declaration view for disconnected result proofs.
    /// Instance rows stay in the primary catalog but never enter static result
    /// solving or static candidate cardinality.
    pub(crate) fn static_declarations(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedSameModuleCallableDeclarationV1,
        ),
    > {
        self.rows_by_key
            .iter()
            .filter(|(key, _)| key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod)
    }

    pub(crate) fn static_candidates(
        &self,
        method: &str,
        arity: u32,
    ) -> &[CanonicalSameModuleCallableKeyV1] {
        self.static_keys_by_method_and_arity
            .get(&(method.into(), arity))
            .map(Box::as_ref)
            .unwrap_or(&[])
    }

    pub(crate) fn declaration_for(
        &self,
        namespace: SameModuleCallableNamespaceV1,
        owner: &str,
        method: &str,
        arity: usize,
    ) -> Option<&VerifiedSameModuleCallableDeclarationV1> {
        let arity = u32::try_from(arity).ok()?;
        let key = match namespace {
            SameModuleCallableNamespaceV1::StaticBoxMethod => {
                CanonicalSameModuleCallableKeyV1::static_box_method(owner, method, arity)
            }
            SameModuleCallableNamespaceV1::InstanceBoxMethod => {
                CanonicalSameModuleCallableKeyV1::instance_box_method(owner, method, arity)
            }
        };
        self.declaration(&key)
    }
}

fn validate_parameters(
    key: &CanonicalSameModuleCallableKeyV1,
    params: &[String],
    param_decls: &[ParamDecl],
) -> Result<(), SameModuleCallableDeclarationCatalogErrorV1> {
    if params.len() != param_decls.len() {
        return Err(
            SameModuleCallableDeclarationCatalogErrorV1::ParameterDeclarationCardinality {
                key: key.clone(),
                params: params.len(),
                declarations: param_decls.len(),
            },
        );
    }
    for (index, (name, declaration)) in params.iter().zip(param_decls).enumerate() {
        if name != &declaration.name {
            return Err(
                SameModuleCallableDeclarationCatalogErrorV1::ParameterNameMismatch {
                    key: key.clone(),
                    index,
                },
            );
        }
    }
    Ok(())
}
