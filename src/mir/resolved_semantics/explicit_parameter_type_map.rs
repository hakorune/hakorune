//! Test-only source receipt for explicit parameter type annotations.
//!
//! The receipt is issued only from one `VerifiedResolvedSourceUnitV1`, so a
//! header/body view cannot be paired with a foreign resolver product. The map
//! preserves source spelling and resolver BindingRef provenance only; numeric
//! classification belongs to a later policy receipt.

use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

use super::{
    BindingKindV1, BindingOriginV1, BindingRefV1, CallableFunctionSyntaxViewV1, FunctionOriginV1,
    SemanticOwnerSourceKindV1, SourceBindingSiteV1, VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExplicitParameterTypeMapRejectV1 {
    SourceUnitInputUnavailable,
    SourceRootUnsupported,
    SourceOwnerMismatch,
    SourceOriginMismatch,
    SourceKindMismatch,
    ParameterDeclarationCardinality,
    ParameterNameMismatch { index: u32 },
    ParameterIndexOverflow,
    MissingParameterBinding { index: u32 },
    BindingOwnerMismatch { index: u32 },
    BindingKindMismatch { index: u32 },
    BindingOriginMismatch { index: u32 },
}

/// Opaque pair issued from one exact source unit. It is intentionally
/// non-cloneable so the map issuer consumes the only paired source receipt.
#[derive(Debug)]
pub(crate) struct VerifiedExplicitParameterSourceReceiptV1<'a> {
    source: CallableFunctionSyntaxViewV1<'a>,
    source_owner: super::FunctionOwnerIdV1,
    source_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    function: &'a VerifiedResolvedFunctionV1,
}

impl<'a> VerifiedExplicitParameterSourceReceiptV1<'a> {
    pub(crate) fn from_source_unit(
        unit: &'a VerifiedResolvedSourceUnitV1,
    ) -> Result<Self, ExplicitParameterTypeMapRejectV1> {
        let input = unit
            .root_function_input()
            .map_err(|_| ExplicitParameterTypeMapRejectV1::SourceUnitInputUnavailable)?;
        let source = CallableFunctionSyntaxViewV1::from_function_ast(input.source().root())
            .ok_or(ExplicitParameterTypeMapRejectV1::SourceRootUnsupported)?;
        Ok(Self {
            source,
            source_owner: input.owner(),
            source_origin: input.function().function_origin(),
            source_kind: input.function().source_kind(),
            function: input.function(),
        })
    }

    fn from_test_parts(
        source: CallableFunctionSyntaxViewV1<'a>,
        source_owner: super::FunctionOwnerIdV1,
        source_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        function: &'a VerifiedResolvedFunctionV1,
    ) -> Self {
        Self {
            source,
            source_owner,
            source_origin,
            source_kind,
            function,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ExplicitParameterTypeRowV1 {
    index: u32,
    binding: BindingRefV1,
    declared_type_name: Option<Box<str>>,
}

impl ExplicitParameterTypeRowV1 {
    #[cfg(test)]
    pub(in crate::mir::resolved_semantics) fn from_test_parts(
        index: u32,
        binding: BindingRefV1,
        declared_type_name: Option<&str>,
    ) -> Self {
        Self {
            index,
            binding,
            declared_type_name: declared_type_name.map(Into::into),
        }
    }

    pub(crate) const fn index(&self) -> u32 {
        self.index
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}

/// Move-only, AST-free source annotation map. Rows are contiguous and sorted
/// by explicit parameter index; no type inference or numeric policy occurs.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedExplicitParameterTypeMapV1 {
    owner: super::FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    rows: Box<[ExplicitParameterTypeRowV1]>,
}

impl VerifiedExplicitParameterTypeMapV1 {
    #[cfg(test)]
    pub(in crate::mir::resolved_semantics) fn from_test_parts(
        owner: super::FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        rows: Box<[ExplicitParameterTypeRowV1]>,
    ) -> Self {
        Self {
            owner,
            function_origin,
            source_kind,
            rows,
        }
    }

    pub(crate) const fn owner(&self) -> super::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn rows(&self) -> &[ExplicitParameterTypeRowV1] {
        &self.rows
    }
}

pub(crate) fn issue_explicit_parameter_type_map_v1(
    receipt: VerifiedExplicitParameterSourceReceiptV1<'_>,
) -> Result<VerifiedExplicitParameterTypeMapV1, ExplicitParameterTypeMapRejectV1> {
    let VerifiedExplicitParameterSourceReceiptV1 {
        source,
        source_owner,
        source_origin,
        source_kind,
        function,
    } = receipt;
    if source_owner != function.owner() {
        return Err(ExplicitParameterTypeMapRejectV1::SourceOwnerMismatch);
    }
    if source_origin != function.function_origin() {
        return Err(ExplicitParameterTypeMapRejectV1::SourceOriginMismatch);
    }
    if source_kind != function.source_kind()
        || source_kind != SemanticOwnerSourceKindV1::DeclaredFunction
    {
        return Err(ExplicitParameterTypeMapRejectV1::SourceKindMismatch);
    }

    let header = source.header();
    if header.params().len() != header.param_decls().len() {
        return Err(ExplicitParameterTypeMapRejectV1::ParameterDeclarationCardinality);
    }

    let mut rows = Vec::with_capacity(header.params().len());
    for (index, (name, declaration)) in header.params().iter().zip(header.param_decls()).enumerate()
    {
        let index = u32::try_from(index)
            .map_err(|_| ExplicitParameterTypeMapRejectV1::ParameterIndexOverflow)?;
        if declaration.name != *name {
            return Err(ExplicitParameterTypeMapRejectV1::ParameterNameMismatch { index });
        }
        let site = SourceBindingSiteV1::Parameter { index };
        let binding = function
            .declaration_binding(&site)
            .ok_or(ExplicitParameterTypeMapRejectV1::MissingParameterBinding { index })?;
        if binding.owner() != function.owner() {
            return Err(ExplicitParameterTypeMapRejectV1::BindingOwnerMismatch { index });
        }
        let record = function
            .binding(binding)
            .ok_or(ExplicitParameterTypeMapRejectV1::BindingOwnerMismatch { index })?;
        if !matches!(record.kind(), BindingKindV1::Parameter { index: actual } if actual == index) {
            return Err(ExplicitParameterTypeMapRejectV1::BindingKindMismatch { index });
        }
        if !matches!(
            record.origin(),
            BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: actual })
                if *actual == index
        ) {
            return Err(ExplicitParameterTypeMapRejectV1::BindingOriginMismatch { index });
        }
        rows.push(ExplicitParameterTypeRowV1 {
            index,
            binding,
            declared_type_name: declaration.declared_type_name.as_deref().map(Into::into),
        });
    }

    Ok(VerifiedExplicitParameterTypeMapV1 {
        owner: function.owner(),
        function_origin: function.function_origin(),
        source_kind,
        rows: rows.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, ParamDecl};
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::tests as lease_tests;
    use crate::parser::NyashParser;

    const SOURCE: &str = r#"
function mixed(first: i64, second) {
    return first
}
"#;

    fn function_ast(source: &str) -> ASTNode {
        let root = NyashParser::parse_from_string(source).expect("parameter fixture parses");
        let ASTNode::Program { statements, .. } = root else {
            panic!("parameter fixture must be a Program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("parameter fixture function")
    }

    fn source_receipt<'a>(
        unit: &'a VerifiedResolvedSourceUnitV1,
    ) -> VerifiedExplicitParameterSourceReceiptV1<'a> {
        VerifiedExplicitParameterSourceReceiptV1::from_source_unit(unit).expect("source receipt")
    }

    fn mutated_receipt<'a>(
        ast: &'a ASTNode,
        unit: &'a VerifiedResolvedSourceUnitV1,
    ) -> VerifiedExplicitParameterSourceReceiptV1<'a> {
        let input = unit.root_function_input().expect("root input");
        VerifiedExplicitParameterSourceReceiptV1::from_test_parts(
            CallableFunctionSyntaxViewV1::from_function_ast(ast).expect("function view"),
            input.owner(),
            input.function().function_origin(),
            input.function().source_kind(),
            input.function(),
        )
    }

    #[test]
    fn preserves_typed_and_untyped_parameter_annotations() {
        let unit = lease_tests::unit(SOURCE);
        let map = issue_explicit_parameter_type_map_v1(source_receipt(&unit)).expect("map");
        assert_eq!(map.rows().len(), 2);
        assert_eq!(map.rows()[0].index(), 0);
        assert_eq!(map.rows()[0].binding().owner(), map.owner());
        assert_eq!(map.rows()[0].declared_type_name(), Some("i64"));
        assert_eq!(map.rows()[1].index(), 1);
        assert_eq!(map.rows()[1].declared_type_name(), None);
    }

    #[test]
    fn rejects_parameter_declaration_cardinality_without_fallback() {
        let unit = lease_tests::unit(SOURCE);
        let mut ast = function_ast(SOURCE);
        let ASTNode::FunctionDeclaration { param_decls, .. } = &mut ast else {
            panic!("function fixture")
        };
        param_decls.pop();
        assert_eq!(
            issue_explicit_parameter_type_map_v1(mutated_receipt(&ast, &unit)),
            Err(ExplicitParameterTypeMapRejectV1::ParameterDeclarationCardinality)
        );
    }

    #[test]
    fn rejects_parameter_name_mismatch_before_map_issue() {
        let unit = lease_tests::unit(SOURCE);
        let mut ast = function_ast(SOURCE);
        let ASTNode::FunctionDeclaration { param_decls, .. } = &mut ast else {
            panic!("function fixture")
        };
        param_decls[0] = ParamDecl {
            name: "foreign".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        };
        assert_eq!(
            issue_explicit_parameter_type_map_v1(mutated_receipt(&ast, &unit)),
            Err(ExplicitParameterTypeMapRejectV1::ParameterNameMismatch { index: 0 })
        );
    }

    #[test]
    fn rejects_extra_parameter_without_resolver_binding() {
        let unit = lease_tests::unit(SOURCE);
        let mut ast = function_ast(SOURCE);
        let ASTNode::FunctionDeclaration {
            params,
            param_decls,
            ..
        } = &mut ast
        else {
            panic!("function fixture")
        };
        params.push("extra".to_owned());
        param_decls.push(ParamDecl {
            name: "extra".to_owned(),
            declared_type_name: None,
        });
        assert_eq!(
            issue_explicit_parameter_type_map_v1(mutated_receipt(&ast, &unit)),
            Err(ExplicitParameterTypeMapRejectV1::MissingParameterBinding { index: 2 })
        );
    }

    #[test]
    fn rejects_foreign_source_owner_receipt() {
        let unit = lease_tests::unit(SOURCE);
        let foreign = lease_tests::unit(SOURCE);
        let source_input = unit.root_function_input().expect("source input");
        let input = foreign.root_function_input().expect("foreign input");
        let source = CallableFunctionSyntaxViewV1::from_function_ast(source_input.source().root())
            .expect("source view");
        let receipt = VerifiedExplicitParameterSourceReceiptV1::from_test_parts(
            source,
            source_input.owner(),
            source_input.function().function_origin(),
            source_input.function().source_kind(),
            input.function(),
        );
        assert_eq!(
            issue_explicit_parameter_type_map_v1(receipt),
            Err(ExplicitParameterTypeMapRejectV1::SourceOwnerMismatch)
        );
    }
}
