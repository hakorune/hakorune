use crate::mir::extern_call_route_plan::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::same_module_body_shape::supported_backend_global;
use crate::mir::{Callee, MirInstruction};

use super::GenericPureStringAnalysisContext;
use crate::mir::global_call_route_plan::generic_string_facts::{
    set_proven_flow_value_class, set_string_handle_value_class, set_value_class, value_class,
    GenericPureValueClass,
};
use crate::mir::global_call_route_plan::generic_string_reject::GenericPureStringReject;
use crate::mir::global_call_route_plan::generic_string_route_value_class::{
    generic_pure_string_generic_i64_target_value_class, generic_pure_string_route_value_class,
};
use crate::mir::global_call_route_plan::generic_string_surface::{
    generic_pure_string_accepts_array_len_method, generic_pure_string_accepts_array_push_method,
    generic_pure_string_accepts_collection_birth_method,
    generic_pure_string_accepts_contains_method, generic_pure_string_accepts_env_set,
    generic_pure_string_accepts_indexof_method, generic_pure_string_accepts_lastindexof_method,
    generic_pure_string_accepts_length_method, generic_pure_string_accepts_map_set_method,
    generic_pure_string_accepts_substring_method, generic_pure_string_global_name_is_self,
};
use crate::mir::global_call_route_plan::model::{
    GlobalCallReturnContract, GlobalCallTargetShape, GlobalCallTargetShapeReason,
};
use crate::mir::global_call_route_plan::shape_blocker::propagated_unknown_global_target_blocker;

pub(super) fn generic_pure_string_call_reject_reason(
    ctx: &mut GenericPureStringAnalysisContext<'_>,
    instruction: &MirInstruction,
) -> Option<GenericPureStringReject> {
    match instruction {
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if matches!(
            classify_extern_call_route(name, args.len()),
            Some(
                ExternCallRouteKind::EnvGet
                    | ExternCallRouteKind::Stage1EmitProgramJson
                    | ExternCallRouteKind::Stage1EmitMirFromSource
                    | ExternCallRouteKind::Stage1EmitMirFromProgramJson
            )
        ) =>
        {
            if let Some(dst) = dst {
                *ctx.has_string_surface = true;
                set_string_handle_value_class(ctx.values, *dst, ctx.changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if generic_pure_string_accepts_env_set(name, args, ctx.values) => {
            if let Some(dst) = dst {
                set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if classify_extern_call_route(name, args.len())
            == Some(ExternCallRouteKind::HostBridgeExternInvoke) =>
        {
            if let Some(dst) = dst {
                *ctx.has_string_surface = true;
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    ctx.changed,
                );
            }
            None
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(_)),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedExternCall,
        )),
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } => {
            if let Some(class) = generic_pure_string_route_value_class(
                ctx.function,
                ctx.block,
                ctx.instruction_index,
            ) {
                if matches!(
                    class,
                    GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
                ) {
                    *ctx.has_string_surface = true;
                }
                if let Some(dst) = dst {
                    set_proven_flow_value_class(ctx.values, *dst, class, ctx.changed);
                }
                return None;
            }
            let receiver_class = value_class(ctx.values, *receiver);
            if generic_pure_string_accepts_collection_birth_method(
                box_name,
                method,
                args,
                receiver_class,
            ) {
                if let Some(dst) = dst {
                    set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                }
                return None;
            }
            if generic_pure_string_accepts_array_push_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                }
                return None;
            }
            if generic_pure_string_accepts_map_set_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                }
                return None;
            }
            if receiver_class == GenericPureValueClass::Unknown
                || args
                    .iter()
                    .any(|arg| value_class(ctx.values, *arg) == GenericPureValueClass::Unknown)
            {
                if *ctx.changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                ));
            }
            if generic_pure_string_accepts_length_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                return None;
            }
            if generic_pure_string_accepts_array_len_method(box_name, method, args, receiver_class)
            {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                return None;
            }
            if generic_pure_string_accepts_indexof_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                return None;
            }
            if generic_pure_string_accepts_lastindexof_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(ctx.values, *dst, GenericPureValueClass::I64, ctx.changed);
                return None;
            }
            if generic_pure_string_accepts_contains_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(ctx.values, *dst, GenericPureValueClass::Bool, ctx.changed);
                return None;
            }
            if generic_pure_string_accepts_substring_method(
                box_name,
                method,
                args,
                receiver_class,
                ctx.values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                *ctx.has_string_surface = true;
                set_value_class(ctx.values, *dst, GenericPureValueClass::String, ctx.changed);
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
            ))
        }
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
        )),
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if supported_backend_global(name) => None,
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if !supported_backend_global(name) => {
            if is_hostbridge_extern_invoke_symbol(name, args.len()) {
                if let Some(dst) = dst {
                    *ctx.has_string_surface = true;
                    set_proven_flow_value_class(
                        ctx.values,
                        *dst,
                        GenericPureValueClass::StringOrVoid,
                        ctx.changed,
                    );
                }
                return None;
            }
            if generic_pure_string_global_name_is_self(name, ctx.function.signature.name.as_str()) {
                if let Some(dst) = dst {
                    *ctx.has_string_surface = true;
                    set_proven_flow_value_class(
                        ctx.values,
                        *dst,
                        GenericPureValueClass::String,
                        ctx.changed,
                    );
                }
                return None;
            }
            let Some(target) = super::super::lookup_global_call_target(name, ctx.targets) else {
                return Some(GenericPureStringReject::with_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing,
                    crate::mir::naming::normalize_static_global_name(name),
                    None,
                ));
            };
            let Some(contract) = target.return_contract() else {
                return Some(GenericPureStringReject::with_shape_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                    propagated_unknown_global_target_blocker(name, target),
                ));
            };
            match contract {
                GlobalCallReturnContract::StringHandle
                | GlobalCallReturnContract::StringHandleOrNull => {
                    if let Some(dst) = dst {
                        *ctx.has_string_surface = true;
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::String,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ArrayHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::Array,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::MapHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::Map,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ObjectHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::Unknown,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::MixedRuntimeI64OrHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::ScalarOrVoid,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::VoidSentinelI64Zero => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::VoidSentinel,
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ScalarI64
                    if target.shape() == GlobalCallTargetShape::GenericI64Body =>
                {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            generic_pure_string_generic_i64_target_value_class(target),
                            ctx.changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ScalarI64 => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            ctx.values,
                            *dst,
                            GenericPureValueClass::I64,
                            ctx.changed,
                        );
                    }
                    None
                }
            }
        }
        MirInstruction::Call { .. } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedCall,
        )),
        _ => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedCall,
        )),
    }
}
