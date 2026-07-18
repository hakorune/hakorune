//! Disconnected associated-input boundary for Local initializers.
//!
//! This box owns only one borrowed Local syntax observation, the existing
//! declaration preflight, ordered initializer demand, and one completion
//! through the existing from-values owner. Typed-array and record construction
//! remain behind explicit hooks so a later located port can require exact
//! inactive-subtree proof before using those direct legacy routes.

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::variable_stmt::{
    build_local_statement_from_values_with_types_and_preclaims,
    observe_preflighted_local_statement, preflight_exact_numeric_local_initializers,
};

pub(in crate::mir::builder) struct RawLegacyLocalInputV1 {
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<String>>,
}

impl RawLegacyLocalInputV1 {
    pub(in crate::mir::builder) const fn new(
        variables: Vec<String>,
        initial_values: Vec<Option<Box<ASTNode>>>,
        declared_type_names: Vec<Option<String>>,
    ) -> Self {
        Self {
            variables,
            initial_values,
            declared_type_names,
        }
    }
}

pub(in crate::mir::builder) struct LocalStatementSyntaxViewV1<'input> {
    variables: &'input [String],
    initial_values: &'input [Option<Box<ASTNode>>],
    declared_type_names: &'input [Option<String>],
}

impl<'input> LocalStatementSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(
        variables: &'input [String],
        initial_values: &'input [Option<Box<ASTNode>>],
        declared_type_names: &'input [Option<String>],
    ) -> Self {
        Self {
            variables,
            initial_values,
            declared_type_names,
        }
    }

    pub(in crate::mir::builder) const fn variables(&self) -> &'input [String] {
        self.variables
    }

    pub(in crate::mir::builder) const fn initial_values(&self) -> &'input [Option<Box<ASTNode>>] {
        self.initial_values
    }

    pub(in crate::mir::builder) const fn declared_type_names(&self) -> &'input [Option<String>] {
        self.declared_type_names
    }
}

pub(in crate::mir::builder) trait LocalStatementDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type LocalInput;

    fn local_syntax<'input>(
        &self,
        input: &'input Self::LocalInput,
    ) -> Result<LocalStatementSyntaxViewV1<'input>, String>;

    fn local_initializer_expression_input(
        &self,
        input: &Self::LocalInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String>;

    fn lower_typed_array_literal_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::LocalInput,
        index: usize,
        elements: &[ASTNode],
    ) -> Result<(ValueId, String), String>;

    fn lower_record_constructor_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::LocalInput,
        index: usize,
        class: &str,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String>;
}

impl LocalStatementDescentPortV1 for RawLegacyChildLoweringPortV1 {
    type LocalInput = RawLegacyLocalInputV1;

    fn local_syntax<'input>(
        &self,
        input: &'input Self::LocalInput,
    ) -> Result<LocalStatementSyntaxViewV1<'input>, String> {
        Ok(LocalStatementSyntaxViewV1::new(
            &input.variables,
            &input.initial_values,
            &input.declared_type_names,
        ))
    }

    fn local_initializer_expression_input(
        &self,
        input: &Self::LocalInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        input
            .initial_values
            .get(index)
            .and_then(|value| value.as_deref())
            .cloned()
            .ok_or_else(|| {
                format!("[local-statement-descent/raw-initializer-missing] index={index}")
            })
    }

    fn lower_typed_array_literal_initializer(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::LocalInput,
        _index: usize,
        elements: &[ASTNode],
    ) -> Result<(ValueId, String), String> {
        builder.build_typed_array_literal(elements.to_vec())
    }

    fn lower_record_constructor_initializer(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::LocalInput,
        _index: usize,
        class: &str,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String> {
        builder.build_record_constructor_value(class.to_string(), arguments.to_vec())
    }
}

pub(in crate::mir::builder) fn drive_local_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::LocalInput,
) -> Result<ValueId, String>
where
    Port: LocalStatementDescentPortV1,
{
    let (variables, initial_values, declared_type_names) = {
        let syntax = port.local_syntax(input)?;
        preflight_exact_numeric_local_initializers(
            syntax.variables(),
            syntax.initial_values(),
            syntax.declared_type_names(),
        )?;
        observe_preflighted_local_statement(syntax.variables(), syntax.initial_values());
        (
            syntax.variables().to_vec(),
            syntax.initial_values().to_vec(),
            syntax.declared_type_names().to_vec(),
        )
    };

    let mut evaluated_values = Vec::with_capacity(variables.len());
    let mut preclaimed_arrays = Vec::with_capacity(variables.len());
    for index in 0..variables.len() {
        let typed_spec = declared_type_names
            .get(index)
            .and_then(|value| value.as_deref())
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten();
        let initializer = initial_values.get(index).and_then(|value| value.as_deref());
        let mut preclaimed = None;
        let value = match initializer {
            Some(ASTNode::ArrayLiteral { elements, .. }) if typed_spec.is_some() => {
                let (value, contract_id) =
                    port.lower_typed_array_literal_initializer(builder, input, index, elements)?;
                preclaimed = Some((contract_id, typed_spec.expect("guarded typed spec")));
                value
            }
            Some(ASTNode::New {
                class, arguments, ..
            }) if builder.is_record_constructor_class(class) => {
                port.lower_record_constructor_initializer(builder, input, index, class, arguments)?
            }
            Some(_) => {
                let expression_input = port.local_initializer_expression_input(input, index)?;
                drive_legacy_expression_v1(builder, port, expression_input)?
            }
            None => crate::mir::builder::emission::constant::emit_null(builder)?,
        };
        evaluated_values.push(value);
        preclaimed_arrays.push(preclaimed);
    }

    build_local_statement_from_values_with_types_and_preclaims(
        builder,
        variables,
        evaluated_values,
        declared_type_names,
        preclaimed_arrays,
    )
}

pub(in crate::mir::builder) fn drive_raw_local_statement_v1(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
    declared_type_names: Vec<Option<String>>,
) -> Result<ValueId, String> {
    let input = RawLegacyLocalInputV1::new(variables, initial_values, declared_type_names);
    let mut port = RawLegacyChildLoweringPortV1;
    drive_local_statement_v1(builder, &mut port, &input)
}
