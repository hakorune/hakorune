use super::canonicalize_callsites;
use crate::ast::Span;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirType, UserBoxFieldDecl, ValueId,
};

mod mcl;
mod ncl;
mod ucm;
