use super::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SameModuleCallableDeclarationCatalogErrorV1 {
    ProgramRequired,
    DuplicateStaticBoxOwner {
        owner: String,
    },
    StaticMethodMustBeFunction {
        owner: String,
        method: String,
    },
    MethodNameMismatch {
        owner: String,
        map_name: String,
        declaration_name: String,
    },
    ParameterDeclarationCardinality {
        key: CanonicalSameModuleCallableKeyV1,
        params: usize,
        declarations: usize,
    },
    ParameterNameMismatch {
        key: CanonicalSameModuleCallableKeyV1,
        index: usize,
    },
    ArityOverflow {
        owner: String,
        method: String,
    },
    DuplicateCanonicalKey(CanonicalSameModuleCallableKeyV1),
}
