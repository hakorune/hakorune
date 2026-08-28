//! One pre-effect route for a raw direct `FunctionCall`.
//!
//! This owner observes source plus the read-only Brand/FastMem context once.
//! It does not descend children or mutate the Builder while selecting a route.

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use super::super::{EffectMask, MirBuilder, MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::{
    BareStaticRecoveryDecisionV1, BareStaticRecoveryNoRecoveryReasonV1,
    SameModuleCallableNamespaceV1,
};
use crate::mir::builder::calls::resolver::CalleeResolverBox;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{Callee, TypeOpKind};
use hakorune_mir_defs::CanonicalGlobalTargetV1;

use super::CallTarget;

pub(in crate::mir::builder) struct PreparedRawFunctionPreflightV1 {
    name: String,
    route: PreparedRawFunctionPreflightRouteV1,
}

enum PreparedRawFunctionPreflightRouteV1 {
    Brand(PreparedRawBrandConstructorV1),
    TypeOp {
        operand: ASTNode,
        raw_type_name: String,
        op: TypeOpKind,
    },
    Math {
        arguments: Vec<super::special_method_handlers::PreparedRawMathArgumentV1>,
    },
    FastMem {
        region: FastMemRegionId,
        arguments: Vec<ASTNode>,
        intrinsic: crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1,
    },
    Ordinary {
        completion: PreparedRawOrdinaryFunctionCompletionV1,
    },
}

#[derive(Clone, Copy)]
enum PreparedRawNonBrandRouteOriginV1 {
    InstalledNonBrand,
    RelationlessCompatibility,
}

pub(super) enum PreparedRawOrdinaryFunctionCompletionV1 {
    StrNormalization {
        argument: ASTNode,
    },
    Resolved {
        arguments: Vec<ASTNode>,
    },
    CatalogedTargeted {
        callee: Callee,
        arguments: Vec<ASTNode>,
    },
    Rejected {
        error: String,
    },
}

pub(in crate::mir::builder) enum PreparedRawExplicitExternCallV1 {
    Ready {
        iface_name: String,
        method_name: String,
        return_type: MirType,
        arguments: Vec<ASTNode>,
    },
}

enum PreparedRawBrandConstructorV1 {
    ArityMismatch {
        actual: usize,
        _exact_source: bool,
    },
    Ready {
        argument: ASTNode,
        exact_source: bool,
    },
}

impl PreparedRawBrandConstructorV1 {
    fn prepare(arguments: Vec<ASTNode>, exact_source: bool) -> Self {
        if arguments.len() != 1 {
            return Self::ArityMismatch {
                actual: arguments.len(),
                _exact_source: exact_source,
            };
        }
        Self::Ready {
            argument: arguments
                .into_iter()
                .next()
                .expect("exact-one Brand constructor retains one argument"),
            exact_source,
        }
    }
}

impl PreparedRawExplicitExternCallV1 {
    pub(in crate::mir::builder) fn prepare(
        source_target: String,
        resolved_target: Box<str>,
        arguments: Vec<ASTNode>,
    ) -> Result<Self, String> {
        if source_target != resolved_target.as_ref() {
            return Err("[freeze:contract][explicit-extern/source-relation-drift]".to_owned());
        }
        let return_type = super::extern_calls::explicit_extern_return_type(&resolved_target);
        let (iface_name, method_name) =
            super::extern_calls::split_explicit_extern_name(&resolved_target);
        Ok(Self::Ready {
            iface_name,
            method_name,
            return_type,
            arguments,
        })
    }
}

impl PreparedRawFunctionPreflightV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        name: String,
        arguments: Vec<ASTNode>,
    ) -> Self {
        Self::prepare_with_brand_authority(
            builder,
            name,
            arguments,
            super::RawBrandCallAuthorityV1::RelationlessCompatibility,
        )
    }

    pub(in crate::mir::builder) fn prepare_with_brand_authority(
        builder: &MirBuilder,
        name: String,
        arguments: Vec<ASTNode>,
        authority: super::RawBrandCallAuthorityV1,
    ) -> Self {
        let route = match authority {
            super::RawBrandCallAuthorityV1::InstalledConstructor(_row) => {
                PreparedRawFunctionPreflightRouteV1::Brand(PreparedRawBrandConstructorV1::prepare(
                    arguments, true,
                ))
            }
            super::RawBrandCallAuthorityV1::InstalledNonBrand { caller } => {
                prepare_non_brand_route(
                    builder,
                    &name,
                    arguments,
                    caller,
                    PreparedRawNonBrandRouteOriginV1::InstalledNonBrand,
                )
            }
            super::RawBrandCallAuthorityV1::RelationlessCompatibility => {
                if builder.comp_ctx.is_brand_declared(&name) {
                    PreparedRawFunctionPreflightRouteV1::Brand(
                        PreparedRawBrandConstructorV1::prepare(arguments, false),
                    )
                } else {
                    prepare_non_brand_route(
                        builder,
                        &name,
                        arguments,
                        None,
                        PreparedRawNonBrandRouteOriginV1::RelationlessCompatibility,
                    )
                }
            }
        };
        Self { name, route }
    }
}

fn prepare_non_brand_route(
    builder: &MirBuilder,
    name: &str,
    arguments: Vec<ASTNode>,
    caller: Option<crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    origin: PreparedRawNonBrandRouteOriginV1,
) -> PreparedRawFunctionPreflightRouteV1 {
    if let Some((raw_type_name, op)) = prepare_typeop_route(name, arguments.as_slice()) {
        let mut arguments = arguments.into_iter();
        let operand = arguments
            .next()
            .expect("TypeOp route requires exactly two arguments");
        PreparedRawFunctionPreflightRouteV1::TypeOp {
            operand,
            raw_type_name,
            op,
        }
    } else if super::special_handlers::is_math_function(name) {
        PreparedRawFunctionPreflightRouteV1::Math {
            arguments: super::special_method_handlers::prepare_raw_math_arguments_v1(arguments),
        }
    } else if let Some(region) = builder.current_fastmem_region() {
        if name.starts_with("mem.") {
            let intrinsic =
                crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1::prepare(
                    name,
                    arguments.len(),
                );
            PreparedRawFunctionPreflightRouteV1::FastMem {
                region,
                arguments,
                intrinsic,
            }
        } else {
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: prepare_ordinary_function_completion_v1(
                    builder,
                    name,
                    arguments,
                    caller.as_ref(),
                    origin,
                ),
            }
        }
    } else {
        PreparedRawFunctionPreflightRouteV1::Ordinary {
            completion: prepare_ordinary_function_completion_v1(
                builder,
                name,
                arguments,
                caller.as_ref(),
                origin,
            ),
        }
    }
}

fn prepare_ordinary_function_completion_v1(
    builder: &MirBuilder,
    name: &str,
    arguments: Vec<ASTNode>,
    caller: Option<&crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    origin: PreparedRawNonBrandRouteOriginV1,
) -> PreparedRawOrdinaryFunctionCompletionV1 {
    if name == "str" && arguments.len() == 1 {
        PreparedRawOrdinaryFunctionCompletionV1::StrNormalization {
            argument: arguments
                .into_iter()
                .next()
                .expect("exact str/1 route must retain one argument"),
        }
    } else if let Some(caller) = caller {
        match prepare_cataloged_target_v1(builder, caller, name, arguments.len()) {
            Ok(callee) => {
                PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted { callee, arguments }
            }
            Err(error) => PreparedRawOrdinaryFunctionCompletionV1::Rejected { error },
        }
    } else if matches!(origin, PreparedRawNonBrandRouteOriginV1::InstalledNonBrand)
        && is_installed_non_unified_gc_builtin_v1(name)
    {
        PreparedRawOrdinaryFunctionCompletionV1::Rejected {
            error: format!("[freeze:contract][direct-call/gc-global-retired] name={name}"),
        }
    } else {
        PreparedRawOrdinaryFunctionCompletionV1::Resolved { arguments }
    }
}

fn is_installed_non_unified_gc_builtin_v1(name: &str) -> bool {
    if !matches!(name, "gc_collect" | "gc_stats") {
        return false;
    }
    let classification =
        crate::mir::policies::call_name_classification::classify_call_name_v1(name);
    matches!(
        classification.callee_class(),
        crate::mir::policies::call_name_classification::CallNameCalleeClassV1::BuiltinGlobal
    ) && !classification.raw_unified_admission()
}

fn prepare_cataloged_target_v1(
    builder: &MirBuilder,
    caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    name: &str,
    arity: usize,
) -> Result<Callee, String> {
    let catalog = builder
        .comp_ctx
        .callable_declaration_catalog()
        .map_err(|error| format!("[freeze:contract][direct-call/catalog/{error:?}]"))?;
    if catalog
        .declaration_for(
            caller.namespace(),
            caller.owner(),
            caller.name(),
            caller.arity() as usize,
        )
        .is_none()
    {
        return Err(format!(
            "[freeze:contract][direct-call/foreign-caller] owner={} name={} arity={}",
            caller.owner(),
            caller.name(),
            caller.arity(),
        ));
    }

    let classification =
        crate::mir::policies::call_name_classification::classify_call_name_v1(name).callee_class();
    if matches!(
        classification,
        crate::mir::policies::call_name_classification::CallNameCalleeClassV1::BuiltinGlobal
    ) {
        let target = if name == "print" {
            CanonicalGlobalTargetV1::builtin_print()
        } else {
            CanonicalGlobalTargetV1::new_free_function(
                name.into(),
                u32::try_from(arity).map_err(|_| {
                    "[freeze:contract][direct-call/global-target/arity-overflow]".to_owned()
                })?,
            )
            .map_err(|error| format!("[freeze:contract][direct-call/global-target/{error:?}]"))?
        };
        return resolve_catalog_call_target_v1(builder, CallTarget::Global(target));
    }

    if let Some(declaration) = catalog.declaration_for(
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        caller.owner(),
        name,
        arity,
    ) {
        return resolve_catalog_call_target_v1(
            builder,
            CallTarget::Global(declaration.key().canonical_global_target_v1().map_err(
                |error| format!("[freeze:contract][direct-call/global-target/{error}]"),
            )?),
        );
    }

    let current_owner_has_method = catalog
        .static_declarations()
        .any(|(key, _)| key.owner() == caller.owner() && key.name() == name);
    if current_owner_has_method {
        return Err(format!(
            "[freeze:contract][direct-call/current-owner-arity-mismatch] owner={} name={} arity={arity}",
            caller.owner(),
            name,
        ));
    }

    if let Some(value) = builder
        .function_state
        .variable_ctx
        .variable_map
        .get(name)
        .copied()
    {
        return resolve_catalog_call_target_v1(builder, CallTarget::Value(value));
    }

    if matches!(
        classification,
        crate::mir::policies::call_name_classification::CallNameCalleeClassV1::Extern
    ) {
        return resolve_catalog_call_target_v1(builder, CallTarget::Extern(name.to_owned()));
    }

    match BareStaticRecoveryDecisionV1::decide(catalog, name, arity)
        .map_err(|error| format!("[freeze:contract][direct-call/{error}]"))?
    {
        BareStaticRecoveryDecisionV1::Unique(key) => resolve_catalog_call_target_v1(
            builder,
            CallTarget::Global(
                key.canonical_global_target_v1()
                    .map_err(|error| format!("[freeze:contract][direct-call/global-target/{error}]"))?,
            ),
        ),
        BareStaticRecoveryDecisionV1::NoRecovery(reason) => Err(match reason {
            BareStaticRecoveryNoRecoveryReasonV1::NoCandidate => format!(
                "[freeze:contract][direct-call/no-candidate] name={name} arity={arity}"
            ),
            BareStaticRecoveryNoRecoveryReasonV1::Ambiguous { candidate_count } => format!(
                "[freeze:contract][direct-call/ambiguous-static] name={name} arity={arity} candidates={candidate_count}"
            ),
        }),
    }
}

fn resolve_catalog_call_target_v1(
    builder: &MirBuilder,
    target: CallTarget,
) -> Result<Callee, String> {
    let resolver = CalleeResolverBox::new(
        &builder.function_state.type_ctx.value_origin_newbox,
        &builder.function_state.type_ctx.value_types,
        Some(&builder.comp_ctx.type_registry),
    );
    resolver.resolve(target)
}

fn prepare_typeop_route(name: &str, arguments: &[ASTNode]) -> Option<(String, TypeOpKind)> {
    if (name != "isType" && name != "asType") || arguments.len() != 2 {
        return None;
    }
    let raw_type_name = super::special_handlers::extract_string_literal(&arguments[1])?;
    let op = if name == "isType" {
        TypeOpKind::Check
    } else {
        TypeOpKind::Cast
    };
    Some((raw_type_name, op))
}

pub(in crate::mir::builder) fn lower_prepared_raw_function_preflight_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawFunctionPreflightV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
{
    replay_function_call_trace(builder, &prepared.name);
    match prepared.route {
        PreparedRawFunctionPreflightRouteV1::Brand(brand) => {
            lower_prepared_raw_brand_constructor_with_port_v1(builder, port, prepared.name, brand)
        }
        PreparedRawFunctionPreflightRouteV1::TypeOp {
            operand,
            raw_type_name,
            op,
        } => {
            let value = drive_legacy_expression_v1(builder, port, operand)?;
            let ty = super::special_handlers::parse_type_name_to_mir(&raw_type_name);
            let dst = builder.next_value_id();
            builder.emit_instruction(MirInstruction::TypeOp { dst, op, value, ty })?;
            Ok(dst)
        }
        PreparedRawFunctionPreflightRouteV1::Math { arguments } => {
            builder.lower_math_function_with_port_v1(port, prepared.name, arguments)
        }
        PreparedRawFunctionPreflightRouteV1::FastMem {
            region,
            arguments,
            intrinsic,
        } => {
            crate::mir::builder::fastmem::calls::lower_prepared_fastmem_function_call_with_port_v1(
                builder, region, intrinsic, arguments, port,
            )
        }
        PreparedRawFunctionPreflightRouteV1::Ordinary { completion } => builder
            .lower_prepared_raw_ordinary_function_completion_with_port_v1(
                port,
                prepared.name,
                completion,
            ),
    }
}

fn lower_prepared_raw_brand_constructor_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    name: String,
    prepared: PreparedRawBrandConstructorV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let (argument, exact_source) = match prepared {
        PreparedRawBrandConstructorV1::ArityMismatch {
            actual,
            _exact_source: _,
        } => {
            return Err(format!(
                "[brand/constructor-arity] {} expects exactly one value, got {}",
                name, actual
            ));
        }
        PreparedRawBrandConstructorV1::Ready {
            argument,
            exact_source,
        } => (argument, exact_source),
    };
    if exact_source {
        return port.with_call_argument_source_v1(0, |port| {
            drive_legacy_expression_v1(builder, port, argument)
        });
    }
    drive_legacy_expression_v1(builder, port, argument)
}

pub(in crate::mir::builder) fn lower_prepared_raw_explicit_extern_call_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawExplicitExternCallV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let (iface_name, method_name, return_type, arguments) = match prepared {
        PreparedRawExplicitExternCallV1::Ready {
            iface_name,
            method_name,
            return_type,
            arguments,
        } => (iface_name, method_name, return_type, arguments),
    };
    let arg_values = super::drive_call_arguments_v1(builder, port, arguments.as_slice())?;
    let dst = builder.next_value_id();
    builder.emit_extern_call_with_effects(
        &iface_name,
        &method_name,
        arg_values,
        Some(dst),
        EffectMask::IO,
    )?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, return_type);
    Ok(dst)
}

fn replay_function_call_trace(builder: &MirBuilder, name: &str) {
    if !crate::config::env::cli_verbose() {
        return;
    }
    let current_function = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .unwrap_or("<none>");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[builder] function-call name={} static_ctx={} in_fn={}",
        name,
        builder.comp_ctx.current_static_box.as_deref().unwrap_or(""),
        current_function
    ));
}

#[cfg(test)]
mod explicit_source_identity_tests {
    use super::PreparedRawExplicitExternCallV1;

    #[test]
    fn exact_resolver_symbol_is_required_before_argument_lowering() {
        assert!(PreparedRawExplicitExternCallV1::prepare(
            "env.get".to_owned(),
            Box::<str>::from("env.set"),
            Vec::new(),
        )
        .is_err());
        assert!(PreparedRawExplicitExternCallV1::prepare(
            "env.get".to_owned(),
            Box::<str>::from("env.get"),
            Vec::new(),
        )
        .is_ok());
    }
}

#[cfg(test)]
#[path = "function_call_preflight_route_tests.rs"]
mod tests;
