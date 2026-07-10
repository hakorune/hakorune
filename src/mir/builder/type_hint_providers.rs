//! Type Hint Providers - Type annotation from calls and method signatures
//!
//! Purpose: Register result types from Call/Await instructions
//!
//! Responsibilities:
//! - Annotate Call result types from function signatures
//! - Annotate Constructor Callee with value_origin_newbox
//! - Register explicit Unknown for unresolved targets
//!
//! Called by: `finalize_module()` in module_lifecycle.rs
//!
//! Critical Constraint:
//! Must execute BEFORE phi_type_inference (type annotation prerequisite)

use super::type_context::TypeContext;
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
/// - `Call(Method/Extern/Value/etc)`: register explicit Unknown
///
/// # Arguments
/// - `type_ctx`: type registration state for the function being finalized
/// - `function`: Function to scan for instructions
/// - `module`: Module for function signature lookup
pub(super) fn annotate_missing_result_types_from_calls_and_await(
    type_ctx: &mut TypeContext,
    function: &MirFunction,
    module: &MirModule,
) {
    use crate::mir::definitions::Callee;
    use crate::mir::MirInstruction;

    for (_bid, bb) in function.blocks.iter() {
        for inst in bb.instructions.iter() {
            match inst {
                MirInstruction::Await { dst, future } => {
                    if type_ctx.value_types.contains_key(dst) {
                        continue;
                    }
                    let inferred = match type_ctx.value_types.get(future) {
                        Some(MirType::Future(inner)) => (**inner).clone(),
                        _ => MirType::Unknown,
                    };
                    type_ctx.value_types.insert(*dst, inferred);
                }
                MirInstruction::Call {
                    dst: Some(dst),
                    callee,
                    ..
                } => {
                    if type_ctx.value_types.contains_key(dst) {
                        continue;
                    }
                    let inferred = match callee {
                        Some(callee) => match callee {
                            Callee::Global(name) => module
                                .functions
                                .get(name)
                                .map(|f| f.signature.return_type.clone())
                                .or_else(|| {
                                    crate::mir::builder::types::annotation::infer_return_type(name)
                                })
                                .unwrap_or(MirType::Unknown),
                            Callee::Constructor { box_type } => {
                                let ret = MirType::Box(box_type.clone());
                                type_ctx.value_origin_newbox.insert(*dst, box_type.clone());
                                ret
                            }
                            _ => MirType::Unknown,
                        },
                        None => MirType::Unknown,
                    };
                    type_ctx.value_types.insert(*dst, inferred);
                }
                _ => {}
            }
        }
    }
}
