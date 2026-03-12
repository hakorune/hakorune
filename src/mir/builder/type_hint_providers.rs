//! Type Hint Providers - Type annotation from calls and method signatures
//!
//! Purpose: Register result types from Call/Await instructions
//!
//! Responsibilities:
//! - Annotate Call result types from function signatures
//! - Annotate Constructor Callee with value_origin_newbox
//! - Provide Unknown fallback for unknown targets
//!
//! Called by: `finalize_module()` in module_lifecycle.rs
//!
//! Critical Constraint:
//! Must execute BEFORE phi_type_inference (type annotation prerequisite)

use super::MirBuilder;
use crate::mir::{MirFunction, MirModule, MirType};

/// Annotate missing result types from Call/Await instructions
///
/// Phase 84-5: Guard hardening to ensure all value-producing instructions
/// have types registered before return type inference.
///
/// # Type Registration
/// - `Await`: Unwrap Future<T> → T
/// - `Call`: Lookup function signature return type
/// - `Call(Constructor)`: Register Box type + value_origin_newbox
/// - `Call(Method/Extern/Value/etc)`: Unknown fallback
///
/// # Arguments
/// - `builder`: MirBuilder with type_ctx for registration
/// - `function`: Function to scan for instructions
/// - `module`: Module for function signature lookup
pub(super) fn annotate_missing_result_types_from_calls_and_await(
    builder: &mut MirBuilder,
    function: &MirFunction,
    module: &MirModule,
) {
    use crate::mir::definitions::Callee;
    use crate::mir::MirInstruction;

    for (_bid, bb) in function.blocks.iter() {
        for inst in bb.instructions.iter() {
            match inst {
                MirInstruction::Await { dst, future } => {
                    if builder.type_ctx.value_types.contains_key(dst) {
                        continue;
                    }
                    let inferred = match builder.type_ctx.value_types.get(future) {
                        Some(MirType::Future(inner)) => (**inner).clone(),
                        _ => MirType::Unknown,
                    };
                    builder.type_ctx.value_types.insert(*dst, inferred);
                }
                MirInstruction::Call {
                    dst: Some(dst),
                    callee,
                    ..
                } => {
                    if builder.type_ctx.value_types.contains_key(dst) {
                        continue;
                    }
                    let inferred = match callee {
                        Some(callee) => match callee {
                            Callee::Global(name) => module
                                .functions
                                .get(name)
                                .map(|f| f.signature.return_type.clone())
                                .or_else(|| {
                                    crate::mir::builder::types::annotation::annotate_from_function(
                                        builder, *dst, name,
                                    );
                                    builder.type_ctx.value_types.get(dst).cloned()
                                })
                                .unwrap_or(MirType::Unknown),
                            Callee::Constructor { box_type } => {
                                let ret = MirType::Box(box_type.clone());
                                builder
                                    .type_ctx
                                    .value_origin_newbox
                                    .insert(*dst, box_type.clone());
                                ret
                            }
                            _ => MirType::Unknown,
                        },
                        None => MirType::Unknown,
                    };
                    builder.type_ctx.value_types.insert(*dst, inferred);
                }
                _ => {}
            }
        }
    }
}
