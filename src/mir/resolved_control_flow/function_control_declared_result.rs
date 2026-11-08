//! The declared result contract of a function: absent, `void`, or an annotated type name, and the
//! one projection that asks whether an object result agrees with it (O2).

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredFunctionResultContractV1 {
    Unannotated,
    Void,
    Annotated(Box<str>),
}

impl DeclaredFunctionResultContractV1 {
    /// O2: the declaration is absent, or names the box `class` that every exit returns as a
    /// definite (non-null) object. The name is compared with the class the body returns;
    /// whether that name is a declared box is the header's answer (`declared_result_box`).
    pub(crate) fn agrees_with_object_result(&self, class: &str, nullable: bool) -> bool {
        match self {
            Self::Unannotated => true,
            Self::Annotated(name) => crate::mir::exact_trivial_scalar_abi::DeclaredResultKindV1::nominal_reference(name)
                .is_some_and(|(declared, allows_null)| declared == class && (allows_null || !nullable)),
            Self::Void => false,
        }
    }
}

pub(super) fn declared_result_contract(declared_type_name: Option<&str>) -> DeclaredFunctionResultContractV1 {
    match declared_type_name {
        None => DeclaredFunctionResultContractV1::Unannotated,
        Some("void") => DeclaredFunctionResultContractV1::Void,
        Some(name) => DeclaredFunctionResultContractV1::Annotated(name.into()),
    }
}
