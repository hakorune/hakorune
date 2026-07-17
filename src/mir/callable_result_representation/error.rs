use crate::mir::builder::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultCatalogErrorV1 {
    RequiredArgumentOrdinalOutOfRange {
        key: CanonicalSameModuleCallableKeyV1,
        ordinal: u32,
        arity: u32,
    },
    CallArityOverflow {
        caller: CanonicalSameModuleCallableKeyV1,
        arity: usize,
    },
    ResultRowCardinalityMismatch {
        static_declarations: usize,
        rows: usize,
    },
}
