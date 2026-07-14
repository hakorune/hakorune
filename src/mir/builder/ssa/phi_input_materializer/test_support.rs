use crate::mir::{FunctionSignature, MirType};

pub(super) fn test_signature(name: &str) -> FunctionSignature {
    FunctionSignature {
        name: name.to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: crate::mir::EffectMask::PURE,
    }
}
