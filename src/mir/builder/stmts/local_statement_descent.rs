//! Disconnected associated-input boundary for Local initializers.
//!
//! This box owns only one borrowed Local syntax observation, the existing
//! declaration preflight, ordered initializer demand, and one completion
//! through the existing from-values owner. Typed-array and record construction
//! remain behind explicit hooks so a later located port can require exact
//! inactive-subtree proof before using those direct legacy routes.

use crate::ast::ASTNode;
use crate::mir::builder::raw_structured_child_scope::RawStructuredChildScopePortV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::variable_stmt::{
    build_local_statement_from_values_with_types_and_preclaims,
    observe_preflighted_local_statement, preflight_exact_numeric_local_initializers,
};

pub(in crate::mir::builder) struct RawLegacyLocalInputV1 {
    statement: ASTNode,
}

impl RawLegacyLocalInputV1 {
    pub(in crate::mir::builder) const fn new(statement: ASTNode) -> Self {
        Self { statement }
    }

    pub(in crate::mir::builder) const fn statement(&self) -> &ASTNode {
        &self.statement
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

    fn lower_ordinary_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<ValueId, String>;

    fn lower_typed_array_literal_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<(ValueId, String), String>;

    fn lower_record_constructor_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
        class: &str,
    ) -> Result<ValueId, String>;
}

impl<Port> LocalStatementDescentPortV1 for Port
where
    Port: RawAstChildLoweringPortV1,
{
    type LocalInput = RawLegacyLocalInputV1;

    fn local_syntax<'input>(
        &self,
        input: &'input Self::LocalInput,
    ) -> Result<LocalStatementSyntaxViewV1<'input>, String> {
        let ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } = input.statement()
        else {
            return Err("[freeze:contract][local-descent/raw-input-requires-local]".to_owned());
        };
        Ok(LocalStatementSyntaxViewV1::new(
            variables,
            initial_values,
            declared_type_names,
        ))
    }

    fn lower_ordinary_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<ValueId, String> {
        let index = local_initializer_index(index)?;
        let source = self.prepare_expression_child_source_v1(
            input.statement(),
            ExprChildRoleV1::LocalInitializer(index),
        )?;
        let initializer = take_initializer(input, index as usize)?;
        let mut scoped = RawStructuredChildScopePortV1::new(self, vec![source], Vec::new());
        let value = drive_legacy_expression_v1(builder, &mut scoped, initializer)?;
        scoped.complete_exact_demands_v1()?;
        Ok(value)
    }

    fn lower_typed_array_literal_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
    ) -> Result<(ValueId, String), String> {
        let initializer = initializer_at(input.statement(), index)?;
        let ASTNode::ArrayLiteral { elements, .. } = initializer else {
            return Err("[freeze:contract][raw-local/typed-array-shape-drift]".to_owned());
        };
        let initializer_source = self.prepare_expression_child_source_v1(
            input.statement(),
            ExprChildRoleV1::LocalInitializer(local_initializer_index(index)?),
        )?;
        let sources = prepare_nested_expression_sources(
            &initializer_source,
            initializer,
            elements.len(),
            ExprChildRoleV1::ArrayElement,
        )?;
        let ASTNode::ArrayLiteral { elements, .. } = take_initializer(input, index)? else {
            unreachable!("typed-array shape checked before taking initializer")
        };
        let mut scoped = RawStructuredChildScopePortV1::new(self, sources, Vec::new());
        let value = builder.build_typed_array_literal_with_port_v1(&mut scoped, elements)?;
        scoped.complete_exact_demands_v1()?;
        Ok(value)
    }

    fn lower_record_constructor_initializer(
        &mut self,
        builder: &mut MirBuilder,
        input: &mut Self::LocalInput,
        index: usize,
        class: &str,
    ) -> Result<ValueId, String> {
        let initializer = initializer_at(input.statement(), index)?;
        let ASTNode::New { arguments, .. } = initializer else {
            return Err("[freeze:contract][raw-local/record-shape-drift]".to_owned());
        };
        let initializer_source = self.prepare_expression_child_source_v1(
            input.statement(),
            ExprChildRoleV1::LocalInitializer(local_initializer_index(index)?),
        )?;
        let sources = prepare_nested_expression_sources(
            &initializer_source,
            initializer,
            arguments.len(),
            ExprChildRoleV1::CallArgument,
        )?;
        let ASTNode::New { arguments, .. } = take_initializer(input, index)? else {
            unreachable!("record shape checked before taking initializer")
        };
        let mut scoped = RawStructuredChildScopePortV1::new(self, sources, Vec::new());
        let value = builder.build_record_constructor_value_with_port_v1(
            &mut scoped,
            class.to_string(),
            arguments,
        )?;
        scoped.complete_exact_demands_v1()?;
        Ok(value)
    }
}

fn take_initializer(input: &mut RawLegacyLocalInputV1, index: usize) -> Result<ASTNode, String> {
    let ASTNode::Local { initial_values, .. } = &mut input.statement else {
        return Err("[freeze:contract][local-descent/input-requires-local]".to_owned());
    };
    initial_values
        .get_mut(index)
        .and_then(Option::take)
        .map(|value| *value)
        .ok_or_else(|| format!("[local-statement-descent/raw-initializer-missing] index={index}"))
}

fn initializer_at(statement: &ASTNode, index: usize) -> Result<&ASTNode, String> {
    let ASTNode::Local { initial_values, .. } = statement else {
        return Err("[freeze:contract][local-descent/input-requires-local]".to_owned());
    };
    initial_values
        .get(index)
        .and_then(Option::as_deref)
        .ok_or_else(|| format!("[local-statement-descent/raw-initializer-missing] index={index}"))
}

fn local_initializer_index(index: usize) -> Result<u32, String> {
    u32::try_from(index)
        .map_err(|_| "[freeze:contract][raw-local/initializer-index-overflow]".to_owned())
}

fn prepare_nested_expression_sources(
    initializer_source: &crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1,
    initializer: &ASTNode,
    len: usize,
    role: impl Fn(u32) -> ExprChildRoleV1,
) -> Result<Vec<crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1>, String>
{
    (0..len)
        .map(|index| {
            let index = u32::try_from(index).map_err(|_| {
                "[freeze:contract][raw-local/nested-child-index-overflow]".to_owned()
            })?;
            initializer_source.expression_child(initializer, role(index))
        })
        .collect()
}

pub(in crate::mir::builder) fn drive_local_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    mut input: Port::LocalInput,
) -> Result<ValueId, String>
where
    Port: LocalStatementDescentPortV1,
{
    let (variables, initial_values, declared_type_names) = {
        let syntax = port.local_syntax(&input)?;
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
                    port.lower_typed_array_literal_initializer(builder, &mut input, index)?;
                preclaimed = Some((contract_id, typed_spec.expect("guarded typed spec")));
                value
            }
            Some(ASTNode::New {
                class, arguments, ..
            }) if builder.is_record_constructor_class(class) => {
                let class = class.clone();
                port.lower_record_constructor_initializer(builder, &mut input, index, &class)?
            }
            Some(_) => port.lower_ordinary_initializer(builder, &mut input, index)?,
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
