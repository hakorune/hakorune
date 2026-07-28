//! Disconnected source/catalog seal for `Main0WithPlainInstanceBoxes0`.
//!
//! This owner consumes the shared source inventory before Builder effects. It
//! does not participate in canonical dispatch, expose the owned AST, or lower
//! functions. A later total normal classifier may consume this product.

use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
};

use super::inventory::NormalSourceSurfaceInventoryV1;
use super::main_source::{
    borrow_exact_main_function_v1, NormalMainFunctionSourceErrorV1, NormalMainFunctionSourceViewV1,
};
use super::product::{
    NormalMainMethodSiteV1, NormalTopLevelSiteV1, PreparedNormalSourcePlanInputV1,
};
use super::rejection::{NormalSourcePlanErrorV1, NormalUnsupportedTopLevelKindV1};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalInstanceBoxSiteV1 {
    site: NormalTopLevelSiteV1,
    name: Box<str>,
}

impl NormalInstanceBoxSiteV1 {
    fn new(site: NormalTopLevelSiteV1, name: impl Into<Box<str>>) -> Self {
        Self {
            site,
            name: name.into(),
        }
    }

    pub(crate) fn statement_index(&self) -> usize {
        self.site.statement_index()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct NormalInstanceMethodSourceViewV1<'source> {
    key: &'source CanonicalSameModuleCallableKeyV1,
    function: &'source ASTNode,
    declaration: &'source VerifiedSameModuleCallableDeclarationV1,
}

impl<'source> NormalInstanceMethodSourceViewV1<'source> {
    pub(super) const fn key(&self) -> &'source CanonicalSameModuleCallableKeyV1 {
        self.key
    }

    pub(super) const fn function(&self) -> &'source ASTNode {
        self.function
    }

    pub(super) const fn declaration(&self) -> &'source VerifiedSameModuleCallableDeclarationV1 {
        self.declaration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NormalInstanceMethodSourceLoanErrorV1 {
    WrongNamespace,
    MissingBoxSite,
    SourceDrift,
    CatalogDrift,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalModuleSourceV1 {
    input: PreparedNormalSourcePlanInputV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    instance_boxes: Box<[NormalInstanceBoxSiteV1]>,
    callables: VerifiedSameModuleCallableDeclarationCatalogV1,
    _seal: Main0WithPlainInstanceBoxes0SealV1,
}

#[derive(Debug)]
struct Main0WithPlainInstanceBoxes0SealV1;

impl VerifiedNormalModuleSourceV1 {
    pub(super) fn seal(
        inventory: NormalSourceSurfaceInventoryV1,
    ) -> Result<Self, RejectedNormalModuleSourceV1> {
        let verified = match verify_inventory(&inventory) {
            Ok(verified) => verified,
            Err(error) => return Err(RejectedNormalModuleSourceV1::new(inventory, error)),
        };
        let NormalSourceSurfaceInventoryV1 { input, .. } = inventory;
        Ok(Self {
            input,
            main_box: verified.main_box,
            main_method: verified.main_method,
            instance_boxes: verified.instance_boxes,
            callables: verified.callables,
            _seal: Main0WithPlainInstanceBoxes0SealV1,
        })
    }

    pub(crate) fn source_identity(&self) -> &str {
        self.input.identity().display_name()
    }

    pub(crate) fn main_statement_index(&self) -> usize {
        self.main_box.statement_index()
    }

    pub(crate) fn main_arity(&self) -> usize {
        self.main_method.arity()
    }

    pub(crate) fn instance_boxes(&self) -> &[NormalInstanceBoxSiteV1] {
        &self.instance_boxes
    }

    pub(crate) fn callable_catalog(&self) -> &VerifiedSameModuleCallableDeclarationCatalogV1 {
        &self.callables
    }

    pub(super) fn borrow_exact_main_function(
        &self,
    ) -> Result<NormalMainFunctionSourceViewV1<'_>, NormalMainFunctionSourceErrorV1> {
        borrow_exact_main_function_v1(&self.input, &self.main_box, &self.main_method)
    }

    pub(super) fn borrow_instance_method_source<'source>(
        &'source self,
        key: &'source CanonicalSameModuleCallableKeyV1,
    ) -> Result<NormalInstanceMethodSourceViewV1<'source>, NormalInstanceMethodSourceLoanErrorV1>
    {
        if key.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod {
            return Err(NormalInstanceMethodSourceLoanErrorV1::WrongNamespace);
        }
        let box_site = self
            .instance_boxes
            .iter()
            .find(|site| site.name() == key.owner())
            .ok_or(NormalInstanceMethodSourceLoanErrorV1::MissingBoxSite)?;
        let ASTNode::Program { statements, .. } = self.input.source() else {
            return Err(NormalInstanceMethodSourceLoanErrorV1::SourceDrift);
        };
        let Some(ASTNode::BoxDeclaration { name, methods, .. }) =
            statements.get(box_site.statement_index())
        else {
            return Err(NormalInstanceMethodSourceLoanErrorV1::SourceDrift);
        };
        let Some(function @ ASTNode::FunctionDeclaration { params, .. }) = methods.get(key.name())
        else {
            return Err(NormalInstanceMethodSourceLoanErrorV1::SourceDrift);
        };
        if name != key.owner() || params.len() != key.arity() as usize {
            return Err(NormalInstanceMethodSourceLoanErrorV1::SourceDrift);
        }
        let declaration = self
            .callables
            .declaration(key)
            .ok_or(NormalInstanceMethodSourceLoanErrorV1::CatalogDrift)?;
        if declaration.params() != params || declaration.body() != function_body(function) {
            return Err(NormalInstanceMethodSourceLoanErrorV1::CatalogDrift);
        }
        Ok(NormalInstanceMethodSourceViewV1 {
            key,
            function,
            declaration,
        })
    }
}

fn function_body(function: &ASTNode) -> &[ASTNode] {
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        unreachable!("[normal-module-source/invariant] function source drift")
    };
    body
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalModuleSourceStageV1 {
    Family,
    Entry,
    BoxShape,
    Catalog,
    Correspondence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalModuleUnsupportedTopLevelKindV1 {
    NestedProgram,
    Using,
    Import,
    BuildGate,
    Enum,
    Brand,
    TypeAlias,
    Global,
    StaticConstTable,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalModuleBoxSourceErrorV1 {
    MainSurfaceNotPlain,
    BoxStatementDrift,
    DuplicateBoxName,
    UnsupportedBoxSurface,
    ConstructorUnsupported,
    MethodMustBeFunction,
    StaticMethod,
    MethodNameMismatch,
    MethodContractUnsupported,
    MethodOverrideUnsupported,
    ArityOverflow,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalModuleSourceErrorV1 {
    ScriptSurface,
    TopLevelCallable,
    MissingMain,
    DuplicateMain,
    MissingInstanceBox,
    BoxInventoryMismatch,
    UnsupportedTopLevel {
        statement_index: usize,
        kind: NormalModuleUnsupportedTopLevelKindV1,
    },
    Entry(NormalSourcePlanErrorV1),
    MainHelpersUnsupported,
    BoxShape {
        statement_index: usize,
        cause: NormalModuleBoxSourceErrorV1,
    },
    CatalogRejected,
    Correspondence {
        expected: usize,
        actual: usize,
    },
    CallableRowDrift {
        owner: Box<str>,
        method: Box<str>,
    },
}

impl NormalModuleSourceErrorV1 {
    fn stage(&self) -> NormalModuleSourceStageV1 {
        match self {
            Self::ScriptSurface
            | Self::TopLevelCallable
            | Self::MissingMain
            | Self::DuplicateMain
            | Self::MissingInstanceBox
            | Self::BoxInventoryMismatch
            | Self::UnsupportedTopLevel { .. } => NormalModuleSourceStageV1::Family,
            Self::Entry(_) | Self::MainHelpersUnsupported => NormalModuleSourceStageV1::Entry,
            Self::BoxShape { .. } => NormalModuleSourceStageV1::BoxShape,
            Self::CatalogRejected => NormalModuleSourceStageV1::Catalog,
            Self::Correspondence { .. } | Self::CallableRowDrift { .. } => {
                NormalModuleSourceStageV1::Correspondence
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalModuleSourceV1 {
    owner: NormalSourceSurfaceInventoryV1,
    error: NormalModuleSourceErrorV1,
}

impl RejectedNormalModuleSourceV1 {
    fn new(owner: NormalSourceSurfaceInventoryV1, error: NormalModuleSourceErrorV1) -> Self {
        Self { owner, error }
    }

    pub(crate) fn source_identity(&self) -> &str {
        self.owner.input.identity().display_name()
    }

    pub(crate) fn stage(&self) -> NormalModuleSourceStageV1 {
        self.error.stage()
    }

    pub(crate) fn error(&self) -> &NormalModuleSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ExpectedCallableV1 {
    namespace: SameModuleCallableNamespaceV1,
    owner: Box<str>,
    method: Box<str>,
    arity: u32,
    statement_index: usize,
}

struct VerifiedModulePartsV1 {
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    instance_boxes: Box<[NormalInstanceBoxSiteV1]>,
    callables: VerifiedSameModuleCallableDeclarationCatalogV1,
}

fn verify_inventory(
    inventory: &NormalSourceSurfaceInventoryV1,
) -> Result<VerifiedModulePartsV1, NormalModuleSourceErrorV1> {
    verify_family_closure(inventory)?;
    let main_surface = &inventory.main_boxes[0];
    let (main_box, main_method, helpers) = super::main_source::validate_main_surface(main_surface)
        .map_err(NormalModuleSourceErrorV1::Entry)?;
    if !helpers.is_empty() {
        return Err(NormalModuleSourceErrorV1::MainHelpersUnsupported);
    }
    super::main_source::verify_main_source_parts(&inventory.input, &main_box, &main_method)
        .map_err(|_| NormalModuleSourceErrorV1::BoxShape {
            statement_index: main_box.statement_index(),
            cause: NormalModuleBoxSourceErrorV1::MainSurfaceNotPlain,
        })?;
    verify_plain_main(&inventory.input, &main_box)?;

    let (instance_boxes, mut expected) = verify_instance_boxes(inventory)?;
    expected.push(ExpectedCallableV1 {
        namespace: SameModuleCallableNamespaceV1::StaticBoxMethod,
        owner: "Main".into(),
        method: "main".into(),
        arity: 0,
        statement_index: main_box.statement_index(),
    });
    expected.sort();

    let callables =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(inventory.input.source())
            .map_err(|_| NormalModuleSourceErrorV1::CatalogRejected)?;
    verify_catalog_correspondence(&inventory.input, &callables, &expected)?;

    Ok(VerifiedModulePartsV1 {
        main_box,
        main_method,
        instance_boxes,
        callables,
    })
}

fn verify_family_closure(
    inventory: &NormalSourceSurfaceInventoryV1,
) -> Result<(), NormalModuleSourceErrorV1> {
    if !inventory.script_sites.is_empty() {
        return Err(NormalModuleSourceErrorV1::ScriptSurface);
    }
    if !inventory.top_level_callables.is_empty() {
        return Err(NormalModuleSourceErrorV1::TopLevelCallable);
    }
    match inventory.main_boxes.len() {
        0 => return Err(NormalModuleSourceErrorV1::MissingMain),
        1 => {}
        _ => return Err(NormalModuleSourceErrorV1::DuplicateMain),
    }
    if inventory.non_main_box_sites.is_empty() {
        return Err(NormalModuleSourceErrorV1::MissingInstanceBox);
    }

    let mut box_unsupported = 0;
    for site in &inventory.unsupported {
        if matches!(site.kind, NormalUnsupportedTopLevelKindV1::Box) {
            box_unsupported += 1;
            continue;
        }
        return Err(NormalModuleSourceErrorV1::UnsupportedTopLevel {
            statement_index: site.statement_index,
            kind: module_unsupported_kind(&site.kind),
        });
    }
    if box_unsupported != inventory.non_main_box_sites.len() {
        return Err(NormalModuleSourceErrorV1::BoxInventoryMismatch);
    }
    Ok(())
}

fn module_unsupported_kind(
    kind: &NormalUnsupportedTopLevelKindV1,
) -> NormalModuleUnsupportedTopLevelKindV1 {
    match kind {
        NormalUnsupportedTopLevelKindV1::NestedProgram => {
            NormalModuleUnsupportedTopLevelKindV1::NestedProgram
        }
        NormalUnsupportedTopLevelKindV1::Using => NormalModuleUnsupportedTopLevelKindV1::Using,
        NormalUnsupportedTopLevelKindV1::Import => NormalModuleUnsupportedTopLevelKindV1::Import,
        NormalUnsupportedTopLevelKindV1::BuildGate => {
            NormalModuleUnsupportedTopLevelKindV1::BuildGate
        }
        NormalUnsupportedTopLevelKindV1::Enum => NormalModuleUnsupportedTopLevelKindV1::Enum,
        NormalUnsupportedTopLevelKindV1::Brand => NormalModuleUnsupportedTopLevelKindV1::Brand,
        NormalUnsupportedTopLevelKindV1::TypeAlias => {
            NormalModuleUnsupportedTopLevelKindV1::TypeAlias
        }
        NormalUnsupportedTopLevelKindV1::Global => NormalModuleUnsupportedTopLevelKindV1::Global,
        NormalUnsupportedTopLevelKindV1::StaticConstTable => {
            NormalModuleUnsupportedTopLevelKindV1::StaticConstTable
        }
        NormalUnsupportedTopLevelKindV1::Box => {
            unreachable!("[normal-module-source/invariant] Box is accepted here")
        }
    }
}

fn verify_plain_main(
    input: &PreparedNormalSourcePlanInputV1,
    site: &NormalTopLevelSiteV1,
) -> Result<(), NormalModuleSourceErrorV1> {
    let ASTNode::Program { statements, .. } = input.source() else {
        unreachable!("[normal-module-source/invariant] inventory root drift")
    };
    let Some(ASTNode::BoxDeclaration {
        fields,
        field_decls,
        public_fields,
        private_fields,
        methods,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        transitions,
        is_interface,
        is_record,
        extends,
        implements,
        type_parameters,
        is_sync,
        is_static,
        static_init,
        attrs,
        ..
    }) = statements.get(site.statement_index())
    else {
        return Err(NormalModuleSourceErrorV1::BoxShape {
            statement_index: site.statement_index(),
            cause: NormalModuleBoxSourceErrorV1::MainSurfaceNotPlain,
        });
    };
    if !*is_static
        || methods.len() != 1
        || !fields.is_empty()
        || !field_decls.is_empty()
        || !public_fields.is_empty()
        || !private_fields.is_empty()
        || !constructors.is_empty()
        || !init_fields.is_empty()
        || !weak_fields.is_empty()
        || !delegates.is_empty()
        || !invariants.is_empty()
        || !transitions.is_empty()
        || *is_interface
        || *is_record
        || !extends.is_empty()
        || !implements.is_empty()
        || !type_parameters.is_empty()
        || *is_sync
        || static_init.is_some()
        || !attrs.is_empty()
    {
        return Err(NormalModuleSourceErrorV1::BoxShape {
            statement_index: site.statement_index(),
            cause: NormalModuleBoxSourceErrorV1::MainSurfaceNotPlain,
        });
    }
    Ok(())
}

fn verify_instance_boxes(
    inventory: &NormalSourceSurfaceInventoryV1,
) -> Result<(Box<[NormalInstanceBoxSiteV1]>, Vec<ExpectedCallableV1>), NormalModuleSourceErrorV1> {
    let ASTNode::Program { statements, .. } = inventory.input.source() else {
        unreachable!("[normal-module-source/invariant] inventory root drift")
    };
    let mut names = BTreeSet::new();
    let mut boxes = Vec::with_capacity(inventory.non_main_box_sites.len());
    let mut expected = Vec::new();

    for site in &inventory.non_main_box_sites {
        let statement_index = site.statement_index();
        let Some(ASTNode::BoxDeclaration {
            name,
            methods,
            constructors,
            delegates,
            invariants,
            transitions,
            is_interface,
            is_record,
            extends,
            implements,
            type_parameters,
            is_sync,
            is_static,
            static_init,
            attrs,
            ..
        }) = statements.get(statement_index)
        else {
            return Err(box_error(
                statement_index,
                NormalModuleBoxSourceErrorV1::BoxStatementDrift,
            ));
        };
        if name == "Main" {
            return Err(box_error(
                statement_index,
                NormalModuleBoxSourceErrorV1::BoxStatementDrift,
            ));
        }
        if !names.insert(name.as_str()) {
            return Err(box_error(
                statement_index,
                NormalModuleBoxSourceErrorV1::DuplicateBoxName,
            ));
        }
        if !constructors.is_empty() {
            return Err(box_error(
                statement_index,
                NormalModuleBoxSourceErrorV1::ConstructorUnsupported,
            ));
        }
        if *is_static
            || *is_interface
            || *is_record
            || *is_sync
            || static_init.is_some()
            || !extends.is_empty()
            || !implements.is_empty()
            || !type_parameters.is_empty()
            || !delegates.is_empty()
            || !invariants.is_empty()
            || !transitions.is_empty()
            || !attrs.is_empty()
        {
            return Err(box_error(
                statement_index,
                NormalModuleBoxSourceErrorV1::UnsupportedBoxSurface,
            ));
        }

        let mut method_rows = methods.iter().collect::<Vec<_>>();
        method_rows.sort_by(|(left, _), (right, _)| left.cmp(right));
        for (method_key, method) in method_rows {
            let ASTNode::FunctionDeclaration {
                name: declaration_name,
                params,
                contracts,
                is_static,
                is_override,
                ..
            } = method
            else {
                return Err(box_error(
                    statement_index,
                    NormalModuleBoxSourceErrorV1::MethodMustBeFunction,
                ));
            };
            if *is_static {
                return Err(box_error(
                    statement_index,
                    NormalModuleBoxSourceErrorV1::StaticMethod,
                ));
            }
            if method_key != declaration_name {
                return Err(box_error(
                    statement_index,
                    NormalModuleBoxSourceErrorV1::MethodNameMismatch,
                ));
            }
            if !contracts.is_empty() {
                return Err(box_error(
                    statement_index,
                    NormalModuleBoxSourceErrorV1::MethodContractUnsupported,
                ));
            }
            if *is_override {
                return Err(box_error(
                    statement_index,
                    NormalModuleBoxSourceErrorV1::MethodOverrideUnsupported,
                ));
            }
            let arity = u32::try_from(params.len()).map_err(|_| {
                box_error(statement_index, NormalModuleBoxSourceErrorV1::ArityOverflow)
            })?;
            expected.push(ExpectedCallableV1 {
                namespace: SameModuleCallableNamespaceV1::InstanceBoxMethod,
                owner: name.as_str().into(),
                method: method_key.as_str().into(),
                arity,
                statement_index,
            });
        }
        boxes.push(NormalInstanceBoxSiteV1::new(
            NormalTopLevelSiteV1::new(statement_index),
            name.as_str(),
        ));
    }
    Ok((boxes.into_boxed_slice(), expected))
}

fn verify_catalog_correspondence(
    input: &PreparedNormalSourcePlanInputV1,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
    expected: &[ExpectedCallableV1],
) -> Result<(), NormalModuleSourceErrorV1> {
    let actual = catalog
        .keys()
        .map(|key| (key.namespace(), key.owner(), key.name(), key.arity()))
        .collect::<Vec<_>>();
    let expected_keys = expected
        .iter()
        .map(|row| {
            (
                row.namespace,
                row.owner.as_ref(),
                row.method.as_ref(),
                row.arity,
            )
        })
        .collect::<Vec<_>>();
    if actual != expected_keys {
        return Err(NormalModuleSourceErrorV1::Correspondence {
            expected: expected_keys.len(),
            actual: actual.len(),
        });
    }

    let ASTNode::Program { statements, .. } = input.source() else {
        unreachable!("[normal-module-source/invariant] inventory root drift")
    };
    for row in expected {
        let Some(ASTNode::BoxDeclaration { methods, .. }) = statements.get(row.statement_index)
        else {
            return Err(callable_drift(row));
        };
        let Some(ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            ..
        }) = methods.get(row.method.as_ref())
        else {
            return Err(callable_drift(row));
        };
        let Some(declaration) = catalog.declaration_for(
            row.namespace,
            row.owner.as_ref(),
            row.method.as_ref(),
            row.arity as usize,
        ) else {
            return Err(callable_drift(row));
        };
        if declaration.params() != params
            || declaration.param_decls() != param_decls
            || declaration.return_type_name() != return_type_name.as_deref()
            || declaration.body() != body
            || declaration.uses() != uses
            || declaration.attrs() != attrs
        {
            return Err(callable_drift(row));
        }
    }
    Ok(())
}

fn box_error(
    statement_index: usize,
    cause: NormalModuleBoxSourceErrorV1,
) -> NormalModuleSourceErrorV1 {
    NormalModuleSourceErrorV1::BoxShape {
        statement_index,
        cause,
    }
}

fn callable_drift(row: &ExpectedCallableV1) -> NormalModuleSourceErrorV1 {
    NormalModuleSourceErrorV1::CallableRowDrift {
        owner: row.owner.as_ref().into(),
        method: row.method.as_ref().into(),
    }
}
