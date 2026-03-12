//! SSOT helper for closure-call shape classification.
//!
//! NCL-2:
//! - canonical closure constructor shape:
//!   `Call { callee=Closure, dst=Some(_), args=[] }`
//! - this shape is rewritten to `MirInstruction::NewClosure` by
//!   `callsite_canonicalize`.
//! - all other closure-call shapes are non-canonical and must fail-fast at
//!   backend boundaries.

use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureCallShape {
    CanonicalCtor,
    MissingDst,
    RuntimeArgs,
}

pub fn classify_closure_call_shape(dst: Option<ValueId>, args: &[ValueId]) -> ClosureCallShape {
    if dst.is_none() {
        ClosureCallShape::MissingDst
    } else if !args.is_empty() {
        ClosureCallShape::RuntimeArgs
    } else {
        ClosureCallShape::CanonicalCtor
    }
}

pub fn closure_call_reject_code(shape: ClosureCallShape) -> &'static str {
    match shape {
        ClosureCallShape::CanonicalCtor => "call-closure-not-canonical",
        ClosureCallShape::MissingDst => "call-closure-missing-dst",
        ClosureCallShape::RuntimeArgs => "call-closure-runtime-args",
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_closure_call_shape, closure_call_reject_code, ClosureCallShape};
    use crate::mir::ValueId;

    #[test]
    fn classifies_canonical_ctor_shape() {
        let shape = classify_closure_call_shape(Some(ValueId::new(1)), &[]);
        assert_eq!(shape, ClosureCallShape::CanonicalCtor);
        assert_eq!(
            closure_call_reject_code(shape),
            "call-closure-not-canonical"
        );
    }

    #[test]
    fn classifies_missing_dst_shape() {
        let shape = classify_closure_call_shape(None, &[]);
        assert_eq!(shape, ClosureCallShape::MissingDst);
        assert_eq!(closure_call_reject_code(shape), "call-closure-missing-dst");
    }

    #[test]
    fn classifies_runtime_args_shape() {
        let shape = classify_closure_call_shape(Some(ValueId::new(2)), &[ValueId::new(7)]);
        assert_eq!(shape, ClosureCallShape::RuntimeArgs);
        assert_eq!(closure_call_reject_code(shape), "call-closure-runtime-args");
    }
}
