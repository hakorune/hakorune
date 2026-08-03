//! Verified whole-function prefix for the DirectAccum pilot.
//!
//! The source projection owns the only AST read for the two-entry prefix.
//! Lowering consumes these sealed declaration/initializer rows and therefore
//! does not rediscover local names, binding identity, or literal shape.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::resolved_semantics::{BindingKindV1, BindingRefV1, SourceBindingSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumPrefixRejectV1 {
    SourceNavigation,
    BodyShape,
    LocalShape,
    InitializerShape,
    MissingBinding,
    BindingOwner,
    BindingKind,
    BindingName,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumLocalV1 {
    site: SourceBindingSiteV1,
    kind: BindingKindV1,
    binding: BindingRefV1,
    name: Box<str>,
    initial: i64,
}

impl VerifiedDirectAccumLocalV1 {
    pub(crate) fn site(&self) -> &SourceBindingSiteV1 {
        &self.site
    }

    pub(crate) const fn kind(&self) -> BindingKindV1 {
        self.kind
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn initial(&self) -> i64 {
        self.initial
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumPrefixInputV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    locals: [VerifiedDirectAccumLocalV1; 2],
    _seal: VerifiedDirectAccumPrefixInputSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedDirectAccumPrefixInputSealV1;

impl VerifiedDirectAccumPrefixInputV1 {
    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn locals(&self) -> &[VerifiedDirectAccumLocalV1; 2] {
        &self.locals
    }
}

pub(crate) fn issue_direct_accum_prefix_input_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
) -> Result<VerifiedDirectAccumPrefixInputV1, DirectAccumPrefixRejectV1> {
    let body = input
        .source()
        .root_body()
        .map_err(|_| DirectAccumPrefixRejectV1::SourceNavigation)?;
    if body.statements().len() != 2 {
        return Err(DirectAccumPrefixRejectV1::BodyShape);
    }
    let expected_loop = input
        .source()
        .body_stmt(&body, 1)
        .map_err(|_| DirectAccumPrefixRejectV1::SourceNavigation)?;
    if expected_loop.site() != loop_stmt.site() || !matches!(loop_stmt.node(), ASTNode::Loop { .. })
    {
        return Err(DirectAccumPrefixRejectV1::BodyShape);
    }
    let local = input
        .source()
        .body_stmt(&body, 0)
        .map_err(|_| DirectAccumPrefixRejectV1::SourceNavigation)?;
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = local.node()
    else {
        return Err(DirectAccumPrefixRejectV1::LocalShape);
    };
    if variables.len() != 2
        || initial_values.len() != 2
        || declared_type_names.len() != 2
        || declared_type_names.iter().any(Option::is_some)
    {
        return Err(DirectAccumPrefixRejectV1::LocalShape);
    }
    let mut locals = Vec::with_capacity(2);
    for (ordinal, (name, initial)) in variables.iter().zip(initial_values).enumerate() {
        let initial = match initial.as_deref() {
            Some(ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            }) if *value == 0 => *value,
            _ => return Err(DirectAccumPrefixRejectV1::InitializerShape),
        };
        let ordinal = u32::try_from(ordinal).map_err(|_| DirectAccumPrefixRejectV1::LocalShape)?;
        let site = SourceBindingSiteV1::Local {
            statement: local.site().clone(),
            ordinal,
        };
        let binding = input
            .function()
            .declaration_binding(&site)
            .ok_or(DirectAccumPrefixRejectV1::MissingBinding)?;
        if binding.owner() != input.owner() {
            return Err(DirectAccumPrefixRejectV1::BindingOwner);
        }
        let record = input
            .function()
            .binding(binding)
            .ok_or(DirectAccumPrefixRejectV1::MissingBinding)?;
        if record.kind() != (BindingKindV1::Local { ordinal }) {
            return Err(DirectAccumPrefixRejectV1::BindingKind);
        }
        if record.diagnostic_name() != name {
            return Err(DirectAccumPrefixRejectV1::BindingName);
        }
        locals.push(VerifiedDirectAccumLocalV1 {
            site,
            kind: BindingKindV1::Local { ordinal },
            binding,
            name: name.clone().into_boxed_str(),
            initial,
        });
    }
    let [first, second] = locals
        .try_into()
        .map_err(|_| DirectAccumPrefixRejectV1::LocalShape)?;
    Ok(VerifiedDirectAccumPrefixInputV1 {
        owner: input.owner(),
        locals: [first, second],
        _seal: VerifiedDirectAccumPrefixInputSealV1,
    })
}
